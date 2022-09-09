#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

    #[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	pub(super) type CollectionId = u32;
	pub(super) type TokenId = u32;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CollectionCreated(CollectionId, T::AccountId),
		CollectionDestroyed(CollectionId, T::AccountId),
		NFTMinted(CollectionId, TokenId, T::AccountId),
		NFTBurned(CollectionId, TokenId, T::AccountId),		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		CollectionExists,
		TokenExists,
		OneAccountOneToken,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn create_collection(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			//
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn destroy_collection(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			//
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn mint(origin: OriginFor<T>,collection: CollectionId, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			//
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn burn(origin: OriginFor<T>,collection: CollectionId, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			//
			Ok(())
		}
	}

}