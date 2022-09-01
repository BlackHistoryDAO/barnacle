# Black History DAO Appchain Substrate Node

This DAO Appchain is being built on Barnacle Node.

## Appchain Overview

Black History DAO aims to collect, preserve and share the real stories of Black history and anchoring them on the blockchain. The documents and stories are meant to be verified by the DAO community and experts and stored as a struct on the blockchain and on IPFS.

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


## TODO

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

### Ignore for now
```
$ ./target/release/appchain-barnacle purge-chain --dev
$ ./target/release/appchain-barnacle --dev --enable-offchain-indexing true
```
