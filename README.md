# Black History DAO Blockchain Substrate Node

This DAO Blockchain is being built on substrate Node.

### Docker Image

```
docker pull rafathsn/bhdao
docker run -it --rm -p 127.0.0.1:9944:9944 rafathsn/bhdao
```
### Frontend

Repository : https://github.com/BlackHistoryDAO/website
Frontend on Netlify (requires a local running node. See above) : https://fancy-cajeta-60dafd.netlify.app/
Demo Video : https://drive.google.com/file/d/1OP7KPwDUVbVeiubbsrKTe7lybYQQOJTO/view?usp=sharing
Demo Slides : https://docs.google.com/presentation/d/1lYQ-5LnGVzMvtW5HTDDqjH2BQDyU9SsaGWmi4kH2t_4/edit?usp=sharing


## Blockchain Overview

Black History DAO aims to collect, preserve and share the real stories of Black history and anchoring them on the blockchain. The documents and stories are meant to be verified by the Black History DAO community and appointed by vote experts. Once approved, the data will be stored as a structure on the blockchain and IPFS.

DAO Membership
```

  #[pallet::storage]
	#[pallet::getter(fn get_all_qualifiers)]
	pub(super) type Qualifiers<T:Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_all_collectors)]
	pub(super) type Collectors<T:Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_all_contributors)]
	pub(super) type Contributors<T:Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;
  
```

Document struct

```
pub struct Document<T:Config> {
		pub creator: T::AccountId,
		pub title: Vec<u8>,
		pub description: Vec<u8>,
		pub format: Vec<u8>,
		pub hash: Vec<u8>,// ipfshash
		pub status: DocumentStatus, // Enum {UnderReview,VoteInProgress,Verified,Rejected}
}
```

Documents can be submitted by calling

```
pub fn create_document(origin: OriginFor<T>, title: Vec<u8>, description: Vec<u8>,
		format: Vec<u8>, hash: Vec<u8>) 
```

Document verification is a two-step process with a selected committee of experts voting on the document to filter out 
any duplicates, inauthentic or otherwise inappropriate documents. Then the full membership voting to decide if the 
document is acceptable as a black history relic. Both voting processes are initially implemented as one person one vote
but that'll change before the DAO is fully operational.

```
pub fn create_qualification_voting(origin: OriginFor<T>, document_id: u64)
```

```
pub fn create_verification_voting(origin: OriginFor<T>, document_id: u64)
```

```
pub fn cast_qualification_vote(origin: OriginFor<T>, voting_id: u64, vote_cast: bool)
```

```
pub fn cast_verification_vote(origin: OriginFor<T>, voting_id: u64, vote_cast: bool)
```

```
pub fn finalize_qualification_voting(origin: OriginFor<T>, voting_id: u64)
```

```
pub fn finalize_verification_voting(origin: OriginFor<T>, voting_id: u64)
```

Voting creation, finalization, quorum and voting window functions are currently implemented 
as sudo functions only for testing purposes. 




Implementation so far includes storage elements,  membership setup and management using non-transferable NFTs, functions to add documents data to the blockchain and full end to end voting mechanism. We are currently working on vote scheduling, storage options and building fully decentralized
DAO governance.

## Local Build and Testing

### Install Rust Environment

```
curl https://getsubstrate.io/ -sSf | bash -s - --fast
```

Full instructions at https://docs.oct.network/guides/appchain-develop.html#barnacle

### Clone the repository

```
git clone https://github.com/BlackHistoryDAO/barnacle.git
```

### Build the node and run in dev mode

```
$ cargo build --release
$ ./target/release/appchain-barnacle --dev --tmp

```

### Run Tests

```
cargo test
```


