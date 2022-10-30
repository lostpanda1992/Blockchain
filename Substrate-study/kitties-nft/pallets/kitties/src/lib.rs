#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::{DispatchResult, DispatchResultWithPostInfo},
		pallet_prelude::*,
		sp_runtime::traits::{Hash, Zero},
		traits::{Currency, ExistenceRequirement, Randomness},
		transactional,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_io::hashing::blake2_128;

	#[cfg(feature = "std")]
	use serde::{Deserialize, Serialize};

	// TODO Part II: Struct for holding Kitty information.
	// kitty的数据结构中某个元素的类型定义
	// owner是所有者，类型为AccountId
	// price是地址的balance
	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// Struct for holding Kitty information.
	// 单只kitty的数据结构
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: [u8; 16],
		pub price: Option<BalanceOf<T>>,
		pub gender: Gender,
		pub owner: AccountOf<T>,
	}

	// TODO Part II: Enum and implementation to handle Gender type in Kitty struct.
	// 自定义数据类型
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Gender {
		Male,
		Female,
	}

	#[pallet::pallet]
	#[pallet::generate_store(trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types it depends on.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Currency handler for the Kitties pallet.
		type Currency: Currency<Self::AccountId>;

		// TODO Part II: Specify the custom types for our runtime.
		// 链上随机函数
		type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;

		//最大拥有量
		#[pallet::constant]
		type MaxKittyOwned: Get<u32>;
	}

	// Errors.
	// 各种错误情况
	#[pallet::error]
	pub enum Error<T> {
		// TODO Part III
		/// Handles arithemtic overflow when incrementing the Kitty counter.
		KittyCntOverflow,
		/// An account cannot own more Kitties than `MaxKittyCount`.
		ExceedMaxKittyOwned,
		/// Buyer cannot be the owner.
		BuyerIsKittyOwner,
		/// Cannot transfer a kitty to its owner.
		TransferToSelf,
		/// Handles checking whether the Kitty exists.
		KittyNotExist,
		/// Handles checking that the Kitty is owned by the account transferring, buying or setting a price for it.
		NotKittyOwner,
		/// Ensures the Kitty is for sale.
		KittyNotForSale,
		/// Ensures that the buying price is greater than the asking price.
		KittyBidPriceTooLow,
		/// Ensures that an account has enough funds to purchase a Kitty.
		NotEnoughBalance,
	}

	//定义的广播事件
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// TODO Part III
		/// A new Kitty was sucessfully created. \[sender, kitty_id\]
		Created(T::AccountId, T::Hash),
		/// Kitty price was sucessfully set. \[sender, kitty_id, new_price\]
		PriceSet(T::AccountId, T::Hash, Option<BalanceOf<T>>),
		/// A Kitty was sucessfully transferred. \[from, to, kitty_id\]
		Transferred(T::AccountId, T::AccountId, T::Hash),
		/// A Kitty was sucessfully bought. \[buyer, seller, kitty_id, bid_price\]
		Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
	}

	// ACTION: Storage item to keep a count of all existing Kitties.
	#[pallet::storage]
	#[pallet::getter(fn kitty_cnt)] //创造一个getter函数查看数据
	/// Keeps track of the number of Kitties in existence.
	/// 将kitty猫储存在线上并保持跟踪
	pub(super) type KittyCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

	// TODO Part II: Remaining storage items.
	// 所有kitties数据结构
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Kitty<T>>;

	// kitties 拥有者数据结构
	#[pallet::storage]
	#[pallet::getter(fn kitties_owned)]
	/// Keeps track of what accounts own what Kitty.
	/// 键为AccountId 值为一个列表 其中元素为hash和对应的最大拥有量
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		_,
		Twox64Concat, //64位hash算法
		T::AccountId, //账户
		BoundedVec<T::Hash, T::MaxKittyOwned>,
		ValueQuery, //加上返回的是默认值 没有返回None
	>;

	// TODO Part III: Our pallet's genesis configuration.
	// Our pallet's genesis configuration.
	// 初始化配置
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub kitties: Vec<(T::AccountId, [u8; 16], Gender)>,
	}

	// Required to implement default for GenesisConfig.
	// 初始化时 小猫为空
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { kitties: vec![] }
		}
	}

	// build函数生成小猫（信息）
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// When building a kitty from genesis config, we require the dna and gender to be supplied.
			for (acct, dna, gender) in &self.kitties {
				let _ = <Pallet<T>>::mint(acct, Some(dna.clone()), Some(gender.clone()));
			}
		}
	}

	//所有与用户交互的函数
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO Part III: create_kitty
		// 创造函数
		#[pallet::weight(100)]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?; // 权限确认
			let kitty_id = Self::mint(&sender, None, None)?; // mint铸造加密猫
												 // Logging to the console
			log::info!("A kitty is born with ID: {:?}.", kitty_id); // 通知加密猫的id

			// ACTION #4: Deposit `Created` event
			Self::deposit_event(Event::Created(sender, kitty_id));

			Ok(())
		}

		// TODO Part III: set_price
		// 用户可以直接调用来改变价格
		#[pallet::weight(100)]
		pub fn set_price(
			origin: OriginFor<T>,
			kitty_id: T::Hash,
			new_price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Ensure the kitty exists and is called by the kitty owner
			ensure!(Self::is_kitty_owner(&kitty_id, &sender)?, <Error<T>>::NotKittyOwner);

			let mut kitty = Self::kitties(&kitty_id).ok_or(<Error<T>>::KittyNotExist)?;

			kitty.price = new_price.clone();
			<Kitties<T>>::insert(&kitty_id, kitty);

			// Deposit a "PriceSet" event.
			Self::deposit_event(Event::PriceSet(sender, kitty_id, new_price));

			Ok(())
		}

		// TODO Part III: transfer
		// 转猫
		#[pallet::weight(100)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: T::Hash,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			// Ensure the kitty exists and is called by the kitty owner //在设置价格和转移猫所有权的时候都需要检查所有权
			ensure!(Self::is_kitty_owner(&kitty_id, &from)?, <Error<T>>::NotKittyOwner);

			// Verify the kitty is not transferring back to its owner.
			// 检查是否虚假交易（将猫回传给自身）
			ensure!(from != to, <Error<T>>::TransferToSelf);

			// Verify the recipient has the capacity to receive one more kitty
			// 需要判定新的主人最大拥有量没有超限
			let to_owned = <KittiesOwned<T>>::get(&to);
			ensure!(
				(to_owned.len() as u32) < T::MaxKittyOwned::get(),
				<Error<T>>::ExceedMaxKittyOwned
			);

			//真正的转换函数
			Self::transfer_kitty_to(&kitty_id, &to)?;

			Self::deposit_event(Event::Transferred(from, to, kitty_id));

			Ok(())
		}

		// TODO Part III: buy_kitty
		// 购买小猫
		#[transactional]
		#[pallet::weight(100)]
		pub fn buy_kitty(
			origin: OriginFor<T>,
			kitty_id: T::Hash,
			bid_price: BalanceOf<T>,
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?; //权限检查

			// Check the kitty exists and buyer is not the current kitty owner
			let kitty = Self::kitties(&kitty_id).ok_or(<Error<T>>::KittyNotExist)?; //检查猫的存在与否
			ensure!(kitty.owner != buyer, <Error<T>>::BuyerIsKittyOwner); //当前账户非owner

			// Check the kitty is for sale and the kitty ask price <= bid_price
			// 猫需要设定价格并且价格低于买价
			if let Some(ask_price) = kitty.price {
				ensure!(ask_price <= bid_price, <Error<T>>::KittyBidPriceTooLow);
			} else {
				Err(<Error<T>>::KittyNotForSale)?;
			}

			// Check the buyer has enough free balance
			// 确保买家有足够的钱
			ensure!(T::Currency::free_balance(&buyer) >= bid_price, <Error<T>>::NotEnoughBalance);

			// Verify the buyer has the capacity to receive one more kitty
			// 确保买家没有超限数量
			let to_owned = <KittiesOwned<T>>::get(&buyer);
			ensure!(
				(to_owned.len() as u32) < T::MaxKittyOwned::get(),
				<Error<T>>::ExceedMaxKittyOwned
			);

			let seller = kitty.owner.clone();

			// Transfer the amount from buyer to seller
			// 转钱
			T::Currency::transfer(&buyer, &seller, bid_price, ExistenceRequirement::KeepAlive)?;

			// Transfer the kitty from seller to buyer
			// 转猫
			Self::transfer_kitty_to(&kitty_id, &buyer)?;

			Self::deposit_event(Event::Bought(buyer, seller, kitty_id, bid_price));

			Ok(())
		}

		// TODO Part III: breed_kitty
		// 繁殖小猫
		#[pallet::weight(100)]
		pub fn breed_kitty(
			origin: OriginFor<T>,
			parent1: T::Hash,
			parent2: T::Hash
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Check: Verify `sender` owns both kitties (and both kitties exist).
			// sender是这两只毛猫的owners
			ensure!(Self::is_kitty_owner(&parent1, &sender)?, <Error<T>>::NotKittyOwner);
			ensure!(Self::is_kitty_owner(&parent2, &sender)?, <Error<T>>::NotKittyOwner);

			let new_dna = Self::breed_dna(&parent1, &parent2)?;
			Self::mint(&sender, Some(new_dna), None)?; // 新的dna生成小猫

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// TODO Part III: helper functions for dispatchable functions
		// 生成一个枚举类型
		fn gen_gender() -> Gender {
			let random = T::KittyRandomness::random(&b"gender"[..]).0; //调用一个随机数生成器
			match random.as_ref()[0] % 2 { // 用随机数决定小猫性别
				0 => Gender::Male,
				_ => Gender::Female,
			}
		}

		// 小猫的基因
		fn gen_dna() -> [u8; 16] {
			let payload = (
				T::KittyRandomness::random(&b"dna"[..]).0,
				<frame_system::Pallet<T>>::block_number(),
			);
			payload.using_encoded(blake2_128) //使用blake2_128hash函数编码
		}

		// TODO: increment_nonce, random_hash, mint, transfer_from
		// Helper to mint a Kitty.
		// 铸造加密猫函数
		pub fn mint(
			owner: &T::AccountId,
			dna: Option<[u8; 16]>,
			gender: Option<Gender>,
		) -> Result<T::Hash, Error<T>> {
			let kitty = Kitty::<T> {
				dna: dna.unwrap_or_else(Self::gen_dna),
				price: None,
				gender: gender.unwrap_or_else(Self::gen_gender),
				owner: owner.clone(),
			};

			let kitty_id = T::Hashing::hash_of(&kitty); //求hash

			// Performs this operation first as it may fail
			// 总的猫数量加一
			let new_cnt = Self::kitty_cnt().checked_add(1).ok_or(<Error<T>>::KittyCntOverflow)?;

			// Performs this operation first because as it may fail
			//向owner中加入对应猫的hash
			<KittiesOwned<T>>::try_mutate(&owner, |kitty_vec| kitty_vec.try_push(kitty_id))
				.map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;

			<Kitties<T>>::insert(kitty_id, kitty);
			<KittyCnt<T>>::put(new_cnt);
			Ok(kitty_id)
		}
		
		// 判断kitty猫和调用方之间的关系（是否为owner）
		pub fn is_kitty_owner(kitty_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty.owner == *acct), //*acct得到真正的数字（所有权）
				None => Err(<Error<T>>::KittyNotExist),
			}
		}

		// 出现问题直接终止交易 回滚数据
		#[transactional]
		pub fn transfer_kitty_to(kitty_id: &T::Hash, to: &T::AccountId) -> Result<(), Error<T>> {
			let mut kitty = Self::kitties(&kitty_id).ok_or(<Error<T>>::KittyNotExist)?; //取出kitty猫

			let prev_owner = kitty.owner.clone(); //拿到前一个owner

			// Remove `kitty_id` from the KittyOwned vector of `prev_kitty_owner`
			// 在前一个owner的数据中删除当前猫
			<KittiesOwned<T>>::try_mutate(&prev_owner, |owned| {
				if let Some(ind) = owned.iter().position(|&id| id == *kitty_id) {
					owned.swap_remove(ind);
					return Ok(());
				}
				Err(())
			})
			.map_err(|_| <Error<T>>::KittyNotExist)?;

			// Update the kitty owner
			// 更新ownerS
			kitty.owner = to.clone();
			// Reset the ask price so the kitty is not for sale until `set_price()` is called
			// by the current owner.
			// 将价格变为None 只有owner可以设置价格
			kitty.price = None;

			<Kitties<T>>::insert(kitty_id, kitty); //重新设置小猫和id之间的关系

			//将小猫的id加入到当前owner账户中
			<KittiesOwned<T>>::try_mutate(to, |vec| vec.try_push(*kitty_id))
				.map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;

			Ok(())
		}

		pub fn breed_dna(parent1: &T::Hash, parent2: &T::Hash) -> Result<[u8; 16], Error<T>> {
			let dna1 = Self::kitties(parent1).ok_or(<Error<T>>::KittyNotExist)?.dna;
			let dna2 = Self::kitties(parent2).ok_or(<Error<T>>::KittyNotExist)?.dna;

			let mut new_dna = Self::gen_dna();
			for i in 0..new_dna.len() {
				new_dna[i] = (new_dna[i] & dna1[i]) | (!new_dna[i] & dna2[i]);
			} // 新dna每个数是新dna与父母dna先互与再或
			Ok(new_dna)
		}
	}
}
