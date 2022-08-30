#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::Currency,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::{
		TypeInfo,
	};
	use sp_io::hashing::blake2_128;
	use sp_runtime::ArithmeticError;
	use sp_std::vec::Vec;


	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Document<T:Config> {
		pub creator: T::AccountId,
		pub title: Vec<u8>,
		pub description: Vec<u8>,
		pub format: Vec<u8>,
		pub hash: Vec<u8>,
		pub verified: bool,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Qualifier<T:Config> {
		pub uid: u32,
		pub address: T::AccountId,
		pub metadata: Vec<u8>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Collector<T:Config> {
		pub uid: u32,
		pub address: T::AccountId,
		pub metadata: Vec<u8>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Griot<T:Config> {
		pub uid: u32,
		pub address: T::AccountId,
		pub metadata: Vec<u8>,
	}

	pub enum Roles {
		QualifierRole,
		CollectorRole,
		GriotRole,
		VerifierRole,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_total_items)]
	pub(super) type TotalItems<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_total_transactions)]
	pub(super) type TotalTransactions<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_total_griots)]
	pub(super) type TotalGriots<T> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_total_collectors)]
	pub(super) type TotalCollectors<T> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_total_qualifiers)]
	pub(super) type TotalQualifiers<T> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_transactions_per_address)]
	pub(super) type TransactionsPerAddress<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		u64,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_document)]
	pub(super) type Documents<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64,
		Document<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_qualifier)]
	pub(super) type Qualifiers<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Qualifier<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_collector)]
	pub(super) type Collectors<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Collector<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_griot)]
	pub(super) type Griots<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Griot<T>,
		OptionQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		QualifierAdded(T::AccountId,u32),
		CollectorAdded(T::AccountId,u32),
		GriotAdded(T::AccountId,u32),
		DocumentCreated(T::AccountId,u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		QualifierAlreadyExists,
		CollectorAlreadyExists,
		GriotAlreadyExists,
		NotAQualifier,
		NotACollector,
		NotAGriot,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,3))]
		pub fn add_qualifier(origin: OriginFor<T>, who: T::AccountId, metadata: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(!Qualifiers::<T>::contains_key(&who),Error::<T>::QualifierAlreadyExists);

			let uid = Self::get_total_qualifiers().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let qualifier = Qualifier::<T> {
				uid: uid.clone(),
				address: who.clone(),
				metadata: metadata
			};

			Qualifiers::<T>::insert(who.clone(),&qualifier);
			TotalQualifiers::<T>::put(&uid);
			Self::deposit_event(Event::QualifierAdded(who,uid));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,3))]
		pub fn add_collector(origin: OriginFor<T>, who: T::AccountId, metadata: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(!Collectors::<T>::contains_key(&who),Error::<T>::CollectorAlreadyExists);

			let uid = Self::get_total_collectors().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let collector = Collector::<T> {
				uid: uid.clone(),
				address: who.clone(),
				metadata: metadata
			};

			Collectors::<T>::insert(who.clone(),&collector);
			TotalCollectors::<T>::put(&uid);
			Self::deposit_event(Event::CollectorAdded(who,uid));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,3))]
		pub fn add_griot(origin: OriginFor<T>, who: T::AccountId, metadata: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(!Griots::<T>::contains_key(&who),Error::<T>::GriotAlreadyExists);

			let uid = Self::get_total_griots().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let griot = Griot::<T> {
				uid: uid.clone(),
				address: who.clone(),
				metadata: metadata
			};

			Griots::<T>::insert(who.clone(),&griot);
			TotalGriots::<T>::put(&uid);
			Self::deposit_event(Event::GriotAdded(who,uid));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,3))]
		pub fn create_document(origin: OriginFor<T>, title: Vec<u8>, description: Vec<u8>,
		format: Vec<u8>, hash: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::ensure_griot(who.clone()),Error::<T>::NotAGriot);

			let uid = Self::get_total_items().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let document = Document::<T> {
				creator: who.clone(),
				title: title.clone(),
				description: description.clone(),
				format: format.clone(),
				hash: hash.clone(),
				verified: false
			};

			Documents::<T>::insert(uid.clone(),&document);
			TotalItems::<T>::put(&uid);

			Self::deposit_event(Event::DocumentCreated(who,uid));

			Ok(())
		}
	}

	// Helpful functions
	impl<T: Config> Pallet<T> {
		pub fn ensure_griot(who: T::AccountId) -> bool {
			let check = Griots::<T>::contains_key(who);
			check
		}
		pub fn ensure_collector(who: T::AccountId) -> bool {
			let check = Collectors::<T>::contains_key(who);
			check
		}
		pub fn ensure_qualifier(who: T::AccountId) -> bool {
			let check = Qualifiers::<T>::contains_key(who);
			check
		}
		
	}
}
