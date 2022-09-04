#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//#[cfg(feature = "runtime-benchmarks")]
//mod benchmarking;

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

	#[pallet::type_value]
    pub fn DefaultQualificationVotingWindow<T: Config>() -> u32
    {
        14400u32
    }

	#[pallet::type_value]
    pub fn DefaultVerificationVotingWindow<T: Config>() -> u32
    {
        14400u32
    }

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Document<T:Config> {
		pub creator: T::AccountId,
		pub title: Vec<u8>,
		pub description: Vec<u8>,
		pub format: Vec<u8>,
		pub hash: Vec<u8>,
		pub status: DocumentStatus,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Vote<T:Config> {
		pub document_id: u64,
		pub yes_votes: u64,
		pub no_votes: u64,
		pub start: T::BlockNumber,
		pub end: T::BlockNumber,
		pub status: VoteStatus,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Roles {
		QualifierRole,
		CollectorRole,
		ContributorRole,
		VerifierRole,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum VoteStatus {
		InProgress,
		Passed,
		Failed,
		Expired,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum DocumentStatus {
		Submitted,
		UnderReview,
		SuccessfulReview,
		VoteInProgress,
		Verified,
		Rejected,
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
	#[pallet::getter(fn contributors_uid_count)]
	pub(super) type ContributorsCount<T> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn collectors_uid_count)]
	pub(super) type CollectorsCount<T> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn qualifiers_uid_count)]
	pub(super) type QualifiersCount<T> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_qualification_vote_count)]
	pub(super) type QualificationVotesCount<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_verification_vote_count)]
	pub(super) type VerificationVotesCount<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_qualification_voting_window)]
	pub(super) type QualificationVotingWindow<T> = StorageValue<_, u32,ValueQuery,DefaultQualificationVotingWindow<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_verification_voting_window)]
	pub(super) type VerificationVotingWindow<T> = StorageValue<_, u32,ValueQuery,DefaultVerificationVotingWindow<T>>;

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
	#[pallet::getter(fn get_qualification_vote)]
	pub(super) type QualificationVotes<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64,
		Vote<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_verification_vote)]
	pub(super) type VerificationVotes<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64,
		Vote<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_all_qualifiers)]
	pub(super) type Qualifiers<T:Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_all_collectors)]
	pub(super) type Collectors<T:Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_all_contributors)]
	pub(super) type Contributors<T:Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		QualifierAdded(T::AccountId,u32),
		CollectorAdded(T::AccountId,u32),
		ContributorAdded(T::AccountId,u32),
		DocumentCreated(T::AccountId,u64),
		DocumentStatusUpdated(u64,u8),
		QualificationVotingWindowChanged(u32),
		QualificationVotingStarted(u64),
		VerificationVotingWindowChanged(u32),
		VerificationVotingStarted(u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		QualifierAlreadyExists,
		CollectorAlreadyExists,
		ContributorAlreadyExists,
		NotAQualifier,
		NotACollector,
		NotAContributor,
		DocumentNotFound,
		IncorrectDocumentStatus,
		DocumentTitleNotProvided,
		DocumentDescriptionNotProvided,
		DocumentFormatNotProvided,
		DocumentIPFSHashNotProvided,
		VerificationVoteAlreadyCreated,
		VotingWindowNotValid,
		DocumentNotReviewed,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,3))]
		pub fn add_qualifier(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			let mut qualifiers = Qualifiers::<T>::get();

			let uid = Self::qualifiers_uid_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			match qualifiers.binary_search(&who) {
				Ok(_) => Err(Error::<T>::QualifierAlreadyExists.into()),
				Err(index) => {
					qualifiers.insert(index, who.clone());
					Qualifiers::<T>::put(qualifiers);
					QualifiersCount::<T>::put(uid.clone());
					Self::deposit_event(Event::QualifierAdded(who,uid));
					Ok(())
				}
			}

		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,3))]
		pub fn add_collector(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			let mut collectors = Collectors::<T>::get();

			let uid = Self::collectors_uid_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			match collectors.binary_search(&who) {
				Ok(_) => Err(Error::<T>::CollectorAlreadyExists.into()),
				Err(index) => {
					collectors.insert(index, who.clone());
					Collectors::<T>::put(collectors);
					CollectorsCount::<T>::put(uid.clone());
					Self::deposit_event(Event::CollectorAdded(who,uid));
					Ok(())
				}
			}

		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,3))]
		pub fn add_contributor(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			let mut contributors = Contributors::<T>::get();

			let uid = Self::contributors_uid_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			match contributors.binary_search(&who) {
				Ok(_) => Err(Error::<T>::ContributorAlreadyExists.into()),
				Err(index) => {
					contributors.insert(index, who.clone());
					Contributors::<T>::put(contributors);
					ContributorsCount::<T>::put(uid.clone());
					Self::deposit_event(Event::ContributorAdded(who,uid));
					Ok(())
				}
			}

		}


		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,2))]
		pub fn create_document(origin: OriginFor<T>, title: Vec<u8>, description: Vec<u8>,
		format: Vec<u8>, hash: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::ensure_contributor(who.clone()),Error::<T>::NotAContributor);
			ensure!(!title.is_empty(),Error::<T>::DocumentTitleNotProvided);
			ensure!(!description.is_empty(),Error::<T>::DocumentDescriptionNotProvided);
			ensure!(!format.is_empty(),Error::<T>::DocumentFormatNotProvided);
			ensure!(!hash.is_empty(),Error::<T>::DocumentIPFSHashNotProvided);

			let uid = Self::get_total_items().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let document = Document::<T> {
				creator: who.clone(),
				title: title.clone(),
				description: description.clone(),
				format: format.clone(),
				hash: hash.clone(),
				status: DocumentStatus::Submitted,
			};

			Documents::<T>::insert(uid.clone(),document);
			TotalItems::<T>::put(&uid);

			Self::deposit_event(Event::DocumentCreated(who,uid));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,4))]
		pub fn create_qualification_voting(origin: OriginFor<T>, document_id: u64) -> DispatchResult{

			ensure_root(origin)?;

			let mut document = Self::get_document(document_id.clone()).ok_or(Error::<T>::DocumentNotFound)?;

			ensure!(document.status == DocumentStatus::Submitted, Error::<T>::VerificationVoteAlreadyCreated);

			let uid = Self::get_qualification_vote_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let now = <frame_system::Pallet<T>>::block_number();

			let end = now + QualificationVotingWindow::<T>::get().into();

			let vote = Vote::<T> {
				document_id: document_id,
				yes_votes: 0,
				no_votes: 0,
				start: now,
				end: end,
				status: VoteStatus::InProgress,
			};

			QualificationVotes::<T>::insert(uid.clone(),&vote);
			Self::deposit_event(Event::QualificationVotingStarted(uid));

			document.status = DocumentStatus::UnderReview;
			Documents::<T>::insert(document_id.clone(),document);
			Self::deposit_event(Event::DocumentStatusUpdated(document_id,1));
			
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,4))]
		pub fn create_verification_voting(origin: OriginFor<T>, document_id: u64) -> DispatchResult{

			ensure_root(origin)?;

			let mut document = Self::get_document(document_id.clone()).ok_or(Error::<T>::DocumentNotFound)?;

			ensure!(document.status == DocumentStatus::SuccessfulReview, Error::<T>::DocumentNotReviewed);

			let uid = Self::get_verification_vote_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let now = <frame_system::Pallet<T>>::block_number();

			let end = now + VerificationVotingWindow::<T>::get().into();

			let vote = Vote::<T> {
				document_id: document_id,
				yes_votes: 0,
				no_votes: 0,
				start: now,
				end: end,
				status: VoteStatus::InProgress,
			};

			VerificationVotes::<T>::insert(uid.clone(),&vote);
			Self::deposit_event(Event::VerificationVotingStarted(uid));

			document.status = DocumentStatus::VoteInProgress;
			Documents::<T>::insert(document_id.clone(),document);
			Self::deposit_event(Event::DocumentStatusUpdated(document_id,3));
			
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn set_qualification_voting_window(origin: OriginFor<T>, window: u32) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(window > 0, Error::<T>::VotingWindowNotValid);

			QualificationVotingWindow::<T>::put(window.clone());

			Self::deposit_event(Event::QualificationVotingWindowChanged(window));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn set_verification_voting_window(origin: OriginFor<T>, window: u32) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(window > 0, Error::<T>::VotingWindowNotValid);

			VerificationVotingWindow::<T>::put(window.clone());

			Self::deposit_event(Event::VerificationVotingWindowChanged(window));

			Ok(())
		}

	}

	// Helpful functions
	impl<T: Config> Pallet<T> {
		pub fn ensure_contributor(who: T::AccountId) -> bool {
			let contributors = Contributors::<T>::get();

			let mut check: bool = false;

			match contributors.binary_search(&who) {
				Ok(_) => check = true,
				Err(index) => check = false,
			}
			
			check
		}

		pub fn ensure_collector(who: T::AccountId) -> bool {
			let collectors = Collectors::<T>::get();

			let mut check: bool = false;

			match collectors.binary_search(&who) {
				Ok(_) => check = true,
				Err(index) => check = false,
			}
			
			check
		}

		pub fn ensure_qualifier(who: T::AccountId) -> bool {
			let qualifiers = Qualifiers::<T>::get();

			let mut check: bool = false;

			match qualifiers.binary_search(&who) {
				Ok(_) => check = true,
				Err(index) => check = false,
			}
			
			check
		}

		pub fn update_document_status(document_uid: u64, status: u8) -> DispatchResult {
			let mut document = Self::get_document(document_uid).ok_or(Error::<T>::DocumentNotFound)?;

			match status {
				0 => {
					document.status = DocumentStatus::Submitted;
				},
				1 => {
					document.status = DocumentStatus::UnderReview;
				},
				2 => {
					document.status = DocumentStatus::SuccessfulReview;
				},
				3 => {
					document.status = DocumentStatus::VoteInProgress;
				},
				4 => {
					document.status = DocumentStatus::Verified;
				},
				5 => {
					document.status = DocumentStatus::Rejected;
				},
				_ => ()
			}

			Documents::<T>::insert(&document_uid, &document);
			Self::deposit_event(Event::DocumentStatusUpdated(document_uid,status));

			Ok(())
		}
		
	}
}
