use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Fixed, Instance, Selector},
    poly::Rotation,
};

///电路的基本操作
/// Num 用于适配这个接口中处理的类型，
/// load_private 用于载入 witness，
/// load_constant 用于载入常量，
/// mul 用于计算两数相乘，expose_public 用于设置 instance。
/// 证明者来说在不知道一个叫做证据（witness）（比如一个哈希函数的原象或者一个确定 Merkle-tree 节点的路径）的情况下，构造出一组参数和证明是不可能的。
trait NumericInstructions<F: FieldExt>: Chip<F> {
    ///用于表示一个数的变量
    type Num;

    ///将一个数加载到电路中，用作隐私输入
    fn load_private(&self, layouter: impl Layouter<F>, a: Option<F>) -> Result<Self::Num, Error>;

    /// 将一个数加载到电路中，用作固定常数
    fn load_constant(&self, layouter: impl Layouter<F>, constant: F) -> Result<Self::Num, Error>;

    /// 执行乘法操作
    fn mul(
        &self,
        layouter: impl Layouter<F>,
        a: &Self::Num,
        b: &Self::Num,
    ) -> Result<Self::Num, Error>;

    /// 将一个数置为电路的公开输入
    fn expose_public(
        &self,
        layouter: impl Layouter<F>,
        num: Self::Num,
        row: usize,
    ) -> Result<Self::Num, Error>;
}

///使用 Halo2 进行电路开发，大多数时候，不需要自己定义 instructions 和 chip，
/// 这些模块实现特别功能，属于基础设施，一般用 Halo2 提供的就足够了。
/// 但如果需要使用复杂的，而 Halo2 没有提供的，则需要自己实现，比如实现一个新兴的密码学算法。
///如果要开发自定义 chip，则需要对 Halo2 的 chip trait 进行实现

/// 构造芯片
/// 这块芯片将实现我们的指令集！芯片存储它们自己的配置，
/// 必要情况下也要包含 type markers
struct FieldChip<F: FieldChip> {
    config: FieldConfig,
    _marker: PhantomData<F>, //PhantomData幽灵类型 PhantomData就是告诉编译器：我虽然内部是个FieldConfig，但是请把我整体当做F来看待
}

///需要为芯片配置好实现我们想要的功能所需要的那些列、置换、门。
/// 芯片的状态被存储在一个 config 结构体中，它是在配置过程中由芯片生成，
/// 并且存储在芯片内部。

/// Halo2 的电路系统用 advice 列代表 witness，instance 列代表 public input，fixed 列代表电路中固定的部分
/// Plonk 中引用出来的 lookup table 就是在 fixed 列中，selector 列是 fixed 列的辅助字段，用于确定在什么条件下激活门。
/// 因为每次生成不同的 proof，witness 和 public input 基本上不同；而 fixed 和 selector column 都是和电路有关，跟输入无关。
/// 一个 chip 的关键, 都在其 config 中。如下, 我们自定义的 chip 需要两个 advice 列, 一个 instance 列, 一个 selector 列, 以及一个 constant。
#[derive(Clone, Debug)]
struct FieldConfig {
    /// 对于这块芯片，我们将用到两个 advice 列来实现我们的指令集。
    /// 它们也是我们与电路的其他部分通信所需要用到列。
    /// advice是witness
    advice: [Column<Advice>; 2],

    /// 这是公开输入（instance）列
    instance: Column<Instance>,

    // 我们需要一个 selector 来激活乘法门，从而在用不到`NumericInstructions::mul`指令的
    //  cells 上不设置任何约束。这非常重要，尤其在构建更大型的电路的情况下，列会被多条指令集用到
    s_mul: Selector,

    /// 用来加载常数的 fixed 列
    constant: Column<Fixed>,
}

/// 结构体的实现 拥有construct 和 config两个函数
/// 公式为a^2 * b^2 = c
impl<F: FieldExt> FieldChip<F> {
    fn construct(config: <Self as Chip<F>>::Config) -> Self {
        Self {
            config,
            _marker: PhantomData, //PhantomData幽灵类型 这里的作用不清楚
        }
    }

    // 最关键的函数 configure，enable_equality 用于进行传入参数的相等性检查
    fn configure(
        meta: &mut ConstraintSystem<F>,  //ConstraintSystem 定义在halo2_proofs::src::plonk::circuit的934行 对电路环境的描述包括门，列，置换
        advice: [Column<Advice>; 2],
        instance: Column<Instance>,
        constant: Column<Fixed>,
    ) -> <Self as Chip<F>>::Config {
        meta.enable_equality(instance.into()); //定义在halo2_proofs::src::plonk::circuit的1050行 该列中数据强制相等
        meta.enable_constant(constant); //启用常数 定义在halo2_proofs::src::plonk::circuit的1034行 副作用 强制相等
        for column in &advice {
            meta.enable_equality((*column).into());
        } //两个 advice 列都启用强制相同
        let s_mul = meta.selector();

        // 定义我们的乘法门
        meta.create_gate("mul", |meta| {
            // 我们需要3个 advice cells 和 1个 selector cell 来实现乘法
            // 我们把他们按下表来排列：
            //
            // | a0  | a1  | s_mul |
            // |-----|-----|-------|
            // | lhs | rhs | s_mul |
            // | out |     |       |
            //
            // 门可以用任一相对偏移，但每一个不同的偏移都会对证明增加开销。
            // 最常见的偏移值是 0 (当前行), 1(下一行), -1(上一行)。
            // 针对这三种情况，有特定的构造函数来构造`Rotation` 结构。
            // query_advicd halo2_proofs::src::plonk::circuit 1508行 在相对位置查询advice列
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let rhs = meta.query_advice(advice[1], Rotation::cur());
            let out = meta.query_advice(advice[0], Rotation::next());
            let s_mul = meta.query_selector(s_mul);

            // 最终，我们将约束门的多项式表达式返回。
            // 对于我们的乘法门，我们仅需要一个多项式约束。
            //
            // `create_gate` 函数返回的多项式表达式，在证明系统中一定等于0。
            // 我们的表达式有以下性质：
            // - 当 s_mul = 0 时，lhs, rhs, out 可以是任意值。
            // - 当 s_mul != 0 时，lhs, rhs, out 将满足 lhs * rhs = out 这条约束。 
            vec![s_mul * (lhs * rhs - out)]
        });

        FieldConfig {
            advice,
            instance,
            s_mul,
            constant,
        }
    }
}

///每一个"芯片"类型都要实现Chip接口。
/// Chip接口定义了Layouter在做电路综合时可能需要的关于电路的某些属性，
/// 以及若将该芯片加载到电路所需要设置的任何初始状态。
/// config 函数返回自定义 chip 的配置，loaded 函数返回自定义 chip 的载入数据，此处不需要返回
impl<F: FieldExt> Chip<F> for FieldChip<F> {
    type Config = FieldConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

/// 实现芯片功能
/// 我们之前定义的 instructions 接口需要实现，我们定义 Number 的实现，是对有限域元素的封装
// ANCHOR: instructions-impl
/// A variable representing a number.
#[derive(Clone)]
struct Number<F: FieldExt>(AssignedCell<F, F>);

impl<F: FieldExt> NumericInstructions<F> for FieldChip<F> {
    type Num = Number<F>;

    //重写函数：将一个数加载到电路中，用作隐私输入
    fn load_private(
        &self,
        mut layouter: impl Layouter<F>,
        value: Value<F>,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "load private",
            |mut region| {
                region
                    .assign_advice(|| "private input", config.advice[0], 0, || value)
                    .map(Number)
            },
        )
    }

    // 重写函数：将一个数加载到电路中，用作固定常数
    fn load_constant(
        &self,
        mut layouter: impl Layouter<F>,
        constant: F,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "load constant",
            |mut region| {
                region
                    .assign_advice_from_constant(|| "constant value", config.advice[0], 0, constant)
                    .map(Number)
            },
        )
    }

    // 核心 乘法门实现
    fn mul(
        &self,
        mut layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        // cell 的位置，除了 row 和 column，还可以通过相对位置 offset 来确定，
        // 一般 offset 就三种，0 代表当前位置，1 代表下一个位置，-1 代表上一个位置。
        layouter.assign_region(
            || "mul",
            |mut region: Region<'_, F>| {
                // We only want to use a single multiplication gate in this region,
                // so we enable it at region offset 0; this means it will constrain
                // cells at offsets 0 and 1.
                config.s_mul.enable(&mut region, 0)?;

                // The inputs we've been given could be located anywhere in the circuit,
                // but we can only rely on relative offsets inside this region. So we
                // assign new cells inside the region and constrain them to have the
                // same values as the inputs.
                // 从 config 里取出两个 cell，检查和输入的 a，b 是否相等
                a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
                b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;

                // Now we can assign the multiplication result, which is to be assigned
                // into the output position.
                // 得到乘法结果
                let value = a.0.value().copied() * b.0.value();

                // Finally, we do the assignment to the output, returning a
                // variable to be used in another part of the circuit.
                // 得到结果
                region
                    .assign_advice(|| "lhs * rhs", config.advice[0], 1, || value)
                    .map(Number)
            },
        )
    }

    fn expose_public( //对外暴露？
        &self,
        mut layouter: impl Layouter<F>,
        num: Self::Num,
        row: usize,
    ) -> Result<(), Error> {
        let config = self.config();

        layouter.constrain_instance(num.0.cell(), config.instance, row)
    }
}

/// circuit trait 是开发电路的入口，我们需要定义自己的 circuit 结构，接入 witness 输入

/// 完整的电路实现
///
/// 在这个结构体中，我们保存隐私输入变量。我们使用 `Option<F>` 类型是因为，
/// 在生成密钥阶段，他们不需要有任何的值。在证明阶段中，如果它们任一为 `None` 的话，
/// 我们将得到一个错误。
#[derive(Default)]
struct MyCircuit<F: FieldExt> {
    constant: F,
    a: Option<F>,
    b: Option<F>,
}

impl<F: FieldExt> Circuit<F> for MyCircuit<F> {
    // 因为我们在任一地方值用了一个芯片，所以我们可以重用它的配置。
    type Config = FieldConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    // configure 创建 advice / instance / constant 的存储 column
    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // 我们创建两个 advice 列，作为 FieldChip 的输入。
        let advice = [meta.advice_column(), meta.advice_column()];

        // 我们还需要一个 instance 列来存储公开输入。
        let instance = meta.instance_column();

        // 创建一个 fixed 列来加载常数
        let constant = meta.fixed_column();

        FieldChip::configure(meta, advice, instance, constant)
    }

    // synthesize 是使用自定义 chip，来获取输入 witness 和 constant，最后计算结果，并返回 public input。
    // 其实对一般电路开发来说，只需要实现 circuit trait 就能满足绝大多数的场景，一些常见的功能 chip，Halo2 都已经实现了。
    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let field_chip = FieldChip::<F>::construct(config);

        // 将我们的隐私值加载到电路中。
        let a = field_chip.load_private(layouter.namespace(|| "load a"), self.a)?;
        let b = field_chip.load_private(layouter.namespace(|| "load b"), self.b)?;

        // 将常数因子加载到电路中
        let constant =
            field_chip.load_constant(layouter.namespace(|| "load constant"), self.constant)?;

        // 我们仅有乘法可用，因此我们按以下方法实现电路：
        //     asq  = a*a
        //     bsq  = b*b
        //     absq = asq*bsq
        //     c    = constant*asq*bsq
        //
        // 但是，按下面的方法实现，更加高效: 
        //     ab   = a*b
        //     absq = ab^2
        //     c    = constant*absq
        let ab = field_chip.mul(layouter.namespace(|| "a * b"), a, b)?;
        let absq = field_chip.mul(layouter.namespace(|| "ab * ab"), ab.clone(), ab)?;
        let c = field_chip.mul(layouter.namespace(|| "constant * absq"), constant, absq)?;

        // 将结果作为电路的公开输入进行公开
        field_chip.expose_public(layouter.namespace(|| "expose c"), c, 0)
    }
}

///测试电路的功能
/// 可以用 halo2::dev::MockProver 对象来测试一个电路是否正常工作
fn main() {
    use halo2_proofs::{dev::MockProver, pasta::Fp};

    // ANCHOR: test-circuit
    // The number of rows in our circuit cannot exceed 2^k. Since our example
    // circuit is very small, we can pick a very small value here.
    // 我们电路的行数不能超过 2^k. 因为我们的示例电路很小，我们选择一个较小的值
    let k = 4;

    // 准备好电路的隐私输入和公开输入
    let constant = Fp::from(7);
    let a = Fp::from(2);
    let b = Fp::from(3);
    let c = constant * a.square() * b.square();
    
    // 用隐私输入来实例化电路
    let circuit = MyCircuit {
        constant,
        a: Some(a),
        b: Some(b),
    };
    
    // 将公开输入进行排列。乘法的结果被我们放置在 instance 列的第0行，
    // 所以我们把它放在公开输入的对应位置。
    let mut public_inputs = vec![c];
    
    // 给定正确的公开输入，我们的电路能验证通过
    let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
    assert_eq!(prover.verify(), Ok(()));
    
    // 如果我们尝试用其他的公开输入，证明将失败！
    public_inputs[0] += Fp::one();
    let prover = MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
    assert!(prover.verify().is_err());
}

