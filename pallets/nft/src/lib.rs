#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	use sp_runtime::ArithmeticError;
	use sp_std::vec::Vec;

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

    #[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);


	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Collection<T:Config> {
		pub total_supply: u32,
		pub created_at: T::BlockNumber,
		pub metadata: Vec<u8>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Token<T:Config> {
		pub id: u32,
		pub owner: T::AccountId,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CollectionCreated(u32),
		NFTMinted(u32, u32, T::AccountId),
		NFTBurned(u32, u32, T::AccountId),		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		CollectionExists,
		CollectionDoesNotExist,
		TokenExists,
		NotTheOwner,
		OneAccountOneToken,
		TokenMaxSupplyReached,
		TokenDoesNotExist,
		NullValue,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_total_collections)]
	pub(super) type TotalCollections<T> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_collection)]
	pub(super) type Collections<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		Collection<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_total_tokens)]
	pub(super) type TotalTokens<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		u32,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_active_tokens)]
	pub(super) type ActiveTokens<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		u32,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_token)]
	pub(super) type Tokens<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::AccountId,u32),
		Token<T>,
		OptionQuery,
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub init_qualifiers_count: u32,
		pub init_collectors_count: u32,
		pub init_contributors_count: u32,
		pub init_block: T::BlockNumber,
		pub init_max_qualifiers: u32,
		pub init_max_collectors: u32,
		pub init_max_contributors: u32,
		pub init_qualifiers: Vec<T::AccountId>,
		pub init_collectors: Vec<T::AccountId>,
		pub init_contributors: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { init_qualifiers_count: Default::default(), init_collectors_count: Default::default(),
				init_contributors_count: Default::default(), init_block: Default::default(),
				init_max_qualifiers: Default::default(), init_max_collectors: Default::default(),
				init_max_contributors: Default::default(), init_contributors:  Default::default(),
				init_qualifiers: Default::default(), init_collectors: Default::default()}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {	
			let mut index: u32 = 1;
			Collections::<T>::insert(index.clone(), Collection::<T> {total_supply: self.init_max_qualifiers,
				 created_at: self.init_block.clone(), metadata: b"Qualifiers".to_vec()});
			index = index + 1;
			Collections::<T>::insert(index.clone(), Collection::<T> {total_supply: self.init_max_collectors, 
				created_at: self.init_block.clone(), metadata: b"Collectors".to_vec()});
			index = index + 1;
			Collections::<T>::insert(index.clone(), Collection::<T> {total_supply: self.init_max_contributors,
				 created_at: self.init_block.clone(), metadata: b"Contributors".to_vec()});
		
			let total_collections = 3;

			TotalCollections::<T>::put(total_collections);

			let mut collection_id: u32 = 1;

			index = 1;

			for item in &self.init_qualifiers {
				Tokens::<T>::insert((item.clone(),collection_id.clone()), Token::<T> {
					id: index.clone(),
					owner: item.clone(),
				});
				index = index + 1;
			}

			TotalTokens::<T>::insert(collection_id.clone(),&self.init_qualifiers_count);
			ActiveTokens::<T>::insert(collection_id.clone(),&self.init_qualifiers_count);

			collection_id = collection_id + 1;
			index = 1;

			for item in &self.init_collectors {
				Tokens::<T>::insert((item.clone(),collection_id.clone()), Token::<T> {
					id: index.clone(),
					owner: item.clone(),
				});
				index = index + 1;
			}

			TotalTokens::<T>::insert(collection_id.clone(),&self.init_collectors_count);
			ActiveTokens::<T>::insert(collection_id.clone(),&self.init_collectors_count);

			collection_id = collection_id + 1;
			index = 1;

			for item in &self.init_contributors {
				Tokens::<T>::insert((item.clone(),collection_id.clone()), Token::<T> {
					id: index.clone(),
					owner: item.clone(),
				});
				index = index + 1;
			}

			TotalTokens::<T>::insert(collection_id.clone(),&self.init_contributors_count);
			ActiveTokens::<T>::insert(collection_id.clone(),&self.init_contributors_count);

		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,3))]
		pub fn create_collection(origin: OriginFor<T>,uid: u32, total_supply: u32, metadata: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;// Temporary
			//let who = ensure_signed(origin)?;
			ensure!(!Collections::<T>::contains_key(uid.clone()),Error::<T>::CollectionExists);
			let now = <frame_system::Pallet<T>>::block_number();

			let collection = Collection::<T> {
				total_supply: total_supply,
				created_at: now,
				metadata: metadata,
			};

			Collections::<T>::insert(uid.clone(),&collection);
			let total = Self::get_total_collections().checked_add(1).ok_or(ArithmeticError::Overflow)?;
			TotalCollections::<T>::put(total);

			Self::deposit_event(Event::CollectionCreated(uid));
			
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(5,3))]
		pub fn mint(origin: OriginFor<T>, collection_id: u32, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(Collections::<T>::contains_key(collection_id.clone()),Error::<T>::CollectionDoesNotExist);
			// Active Tokens <= total_supply
			let mut active = Self::get_active_tokens(collection_id);
			let collection = Self::get_collection(collection_id.clone()).unwrap();
			ensure!(active < collection.total_supply,Error::<T>::TokenMaxSupplyReached);
			// Ensure one Token per user policy
			ensure!(!Tokens::<T>::contains_key((who.clone(),collection_id.clone())),Error::<T>::OneAccountOneToken);
			//uid from total tokens
			let uid = Self::get_total_tokens(collection_id.clone()).checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let token = Token::<T> {
				id: uid.clone(),
				owner: who.clone(),
			};

			Tokens::<T>::insert((who.clone(),collection_id.clone()),token);
			active = active + 1;
			ActiveTokens::<T>::insert(collection_id.clone(),active);
			TotalTokens::<T>::insert(collection_id.clone(),uid.clone());

			Self::deposit_event(Event::NFTMinted(collection_id,uid,who));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn burn(origin: OriginFor<T>,collection_id: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			ensure!(Tokens::<T>::contains_key((who.clone(),collection_id.clone())),Error::<T>::TokenDoesNotExist);
			let token = Self::get_token((who.clone(),collection_id.clone())).ok_or(Error::<T>::NullValue)?;
			
			let uid = token.id;
			
			let mut active = Self::get_active_tokens(collection_id);

			Tokens::<T>::remove((who.clone(),collection_id.clone()));
			active = active - 1;
			ActiveTokens::<T>::insert(collection_id.clone(),active);

			Self::deposit_event(Event::NFTBurned(collection_id,uid,who));
			
			Ok(())
		}
	}

}