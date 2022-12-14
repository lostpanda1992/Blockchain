//假如没有启用这个feature就启动no_std属性，这个属性会阻止std引入作用域
#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use frame_support::transactional; //类似于回滚函数 原子级别操作 发生错误强制回滚

  // 2. Declaration of the Pallet type
  // This is a placeholder to implement traits and methods.
  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  // 3. Runtime Configuration Trait
  // All types and constants go here.
  // Use #[pallet::constant] and #[pallet::extra_constants]
  // to pass in values to metadata.
  #[pallet::config]
  pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> 
         + IsType<<Self as frame_system::Config>::Event>;
  }

  // 4. Runtime Storage
  // 储存参数设置
  #[pallet::storage]
  #[pallet::getter(fn my_param)]
  pub type Param<T: Config> = StorageValue<_, u32, ValueQuery>;

  //储存：标识符设置
  #[pallet::storage]
  pub type SetFlag<T: Config> = StorageValue<_, bool, ValueQuery>;

  // 5. Runtime Events
  // Can stringify event types to metadata.
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    SetParam(u32),
  }

  // 8. Runtime Errors
  #[pallet::error] 
  pub enum Error<T> {
    // 参数必须大于100 
        ParamInvalid,
    }

  // 7. Extrinsics
  // Functions that are callable from outside the runtime.
  #[pallet::call] //调度函数
  impl<T: Config> Pallet<T> {
    #[transactional] //事故回滚
    #[pallet::weight(0)] //权重为0（固定）实际需要调整
    pub fn set_param_bigger_than_100(
        origin: OriginFor<T>, 
         param: u32) -> DispatchResult { //返回值为调度结果
      //1、判断调用者权限
      ensure_signed(origin)?;

      //2、开始业务逻辑
      //2.1、将标志位设置为true
      SetFlag::<T>::put(true);

      //2.2、如果参数大于100,则写入到storage praram中
      if param <= 100u32 {
        return Err(Error::<T>::ParamInvalid.into())
      }
	  //写入参数
      Param::<T>::put(param);

      //3、发出事件
      Self::deposit_event(Event::SetParam(param));

      Ok(().into())
    }
  }
}
