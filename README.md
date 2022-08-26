# Octopus Appchain Template

# TODO

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
