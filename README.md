# Black History DAO Appchain Substrate Node

This DAO Appchain is being built on Barnacle Node.

## Appchain Overview

Black History DAO aims to collect, preserve and share the real stories of Black history and anchoring them on the blockchain. The documents and stories are meant to be verified by the DAO community and experts and stored as a struct on the blockchain and on IPFS.

DAO Membership
```
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
	pub struct Contributor<T:Config> {
		pub uid: u32,
		pub address: T::AccountId,
		pub metadata: Vec<u8>,
	}
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

Any submitted document has to undergo an expert review process and a community vote before being considered verified.
Implementation so far includes storage elements,  membership setup and functions to add documents data to the blockchain.
We are currently working on the voting mechanism and full DAO governance. We expect the first iteration of DAO testnet to
be up and running by the end of September.


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

### Ignore for now
```
$ ./target/release/appchain-barnacle purge-chain --dev
$ ./target/release/appchain-barnacle --dev --enable-offchain-indexing true
```
