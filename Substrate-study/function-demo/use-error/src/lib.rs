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
    // #[pallet::getter(fn something)]
	// 定义一个储存 储存班级信息
	#[pallet::storage]
	#[pallet::getter(fn my_class)]
	pub type Class<T: Config> = StorageValue<_, u32>;

	// use storageMap store (student number -> student name).
	// 定义学生信息 
	#[pallet::storage]
	#[pallet::getter(fn students_info)]
	pub type StudentsInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn dorm_info)]
	// 定义寝室信息 
	pub type DormInfo<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat,
		u32, // 房间号
		Blake2_128Concat,
        u32, // 床号
		u32, // 学生编号
		ValueQuery,
	>;
	

    // 5. Runtime Events
    // Can stringify event types to metadata.
	//事件
	//Event是用来在我们具体的函数中做完动作之后发出的
	//一般用来通知前端做一些处理
	//当pallet需要把运行时上的更改或变化通知给外部主体时
	//就需要用到事件。事件是一个枚举类型
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> { 
		SetClass(u32),
		SetStudentInfo(u32, u128),
		SetDormInfo(u32, u32, u32),
	}

	// Error 用法演示
	#[pallet::error]
    pub enum Error<T> {
		// Class 班级只允许设置一次
		SetClassDuplicate
		// 相同学号的只允许设置一次名字
		SetStudentsInfoDuplicate,
		// 相同床位只允许设置一次
		SetDormInfoDuplicate,
	}

    // 7. Extrinsics
    // Functions that are callable from outside the runtime.
	// 交易调用函数
	// 设置信息的交易函数
	// 设置班级
    #[pallet::weight(0)]
	pub fn set_class_info(origin: OriginFor<T>, class:u32) -> DispatchResultWithPostInfo {
		ensure_root(origin)?; // 判断权限

		// 添加下面的三行，使用Error类型
		if Class::<T>::exists() {
			return Err(Error::<T>::SetClassDuplicate.into());
		}

		Class::<T>::put(class); // 向Class中放入信息
		Self::deposit_event(Event::SetClass(class)); // 发送事件通知
		Ok(().into()) //返回正确标识 into()为类型转换函数
	}

	// 设置学生相关信息
	#[pallet::weight(0)]
	pub fn set_student_info(origin: OriginFor<T>, student_number:u32, student_name:u128,) -> DispatchResultWithPostInfo {
		ensure_root(origin)?; // 判断权限
		//添加如下三行，使用Error类型
		if StudentInfo::<T>::contains_key(student_number) {
			return Err(Error::<T>::SetStudentsInfoDuplicate.into());
		}

		StudentsInfo::<T>::insert(&student_number, &student_name);// 向StudentsInfo中放入信息
		Self::deposit_event(Event::SetStudentInfo(student_number, student_name)); //发送事件通知
		Ok(().into()) //返回正确标识
	}

	// 建立房间号 床号 学号之间的关系
	#[pallet::weight(0)]
	pub fn set_dorm_info(
  		origin: OriginFor<T>,
  		dorm_number: u32,
  		bed_number: u32,
  		student_number: u32,
		) -> DispatchResultWithPostInfo {
  			ensure_signed(origin)?; // 判断权限
			//添加如下三行，使用Error类型
			if DormInfo::<T>::contains_key(dorm_info, bed_number) {
				return Err(Error::<T>::SetDormInfoDuplicate.into());
			}

			DormInfo::<T>::insert(&dorm_number, &bed_number,&student_number);
			Self::deposit_event(Event::SetDormInfo(dorm_number,bed_number,student_name));
			Ok(().into()) //返回正确
	}
}