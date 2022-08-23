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
	pub struct Document {
		pub title: Vec<u8>,
		pub description: Vec<u8>,
		pub format: Vec<u8>,
		pub hash: Vec<u8>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Museum<T:Config> {
		pub address: T::AccountId,
		pub metadata: Vec<u8>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Collector<T:Config> {
		pub address: T::AccountId,
		pub metadata: Vec<u8>,
	}

	pub enum Roles {
		Admin,
		Museum,
		Collector,
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
		Document,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_museum)]
	pub(super) type Museums<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Museum<T>,
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

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MuseumAdded(T::AccountId),
		CollectorAdded(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		MuseumAlreadyExists,
		CollectorAlreadyExists,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_museum(origin: OriginFor<T>, museum: T::AccountId, metadata: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn add_collector(origin: OriginFor<T>, collector: T::AccountId, metadata: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;

			Ok(())
		}
	}
}
