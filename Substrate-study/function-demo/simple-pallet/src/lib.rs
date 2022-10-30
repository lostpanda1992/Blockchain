// 1. Imports and Dependencies
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // 2. Declaration of the Pallet type
    // This is a placeholder to implement traits and methods.
	//pallet类型声明
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // 3. Runtime Configuration Trait
    // All types and constants go here.
    // Use #[pallet::constant] and #[pallet::extra_constants]
    // to pass in values to metadata.
	//config trait; 设置特征
    #[pallet::config]
    pub trait Config: frame_system::Config { 
		//定义了一个关联类型
		//满足后面的类型约束（From<Event> + IsType<::Event>）
		// 第一个可以转化成Event,
		//第二个frame_system::Config的Event类型
		//即第二个得到一个自身的类型作为返回值（返回自身的类型）
		type Event: From<Event<Self>>
		+ IsType<<Self as frame_system::Config>::Event>;
	}

    // 4. Runtime Storage
    // Use to declare storage items.
	// 定义要使用的链上存储;
	//链上定义了一个存储，是一个key-value方式的存储结构
	//用于存储我们后面要使用的存证，key是u32格式，value是u128格式
    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub MyStorage<T: Config> = 
		StorageMap<_, Blake2_128Concat,u32, u128>;

    // 5. Runtime Events
    // Can stringify event types to metadata.
	//事件
	//Event是用来在我们具体的函数中做完动作之后发出的
	//一般用来通知前端做一些处理
	//这里我们在Event中定义了一个事件，就是创建存证
	//当pallet需要把运行时上的更改或变化通知给外部主体时
	//就需要用到事件。事件是一个枚举类型
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> { 
		ClaimCreated(u32, u128);
	}

    // 7. Extrinsics
    // Functions that are callable from outside the runtime.
	// 交易调用函数
	// 创建存证的逻辑
    #[pallet::call]
    impl<T:Config> Pallet<T> { 
		#[pallet::weight(0)]
		pub fun create_claim(origin: OriginFor<T>, 
			id: u32,
			claim: u128) -> DispatchResultWithPostInfo {
				ensure_signed(origin)?;
				Proofs::<T>::insert(
					&id,
					&claim,
				);

				Self::deposit_event(Event::ClaimCreated(id, claim));
				Ok(().into())
			}
	 }

}