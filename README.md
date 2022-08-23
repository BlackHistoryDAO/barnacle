# Octopus Appchain Template

# TODO

### Install Rust Environment

```
curl https://getsubstrate.io/ -sSf | bash -s - --fast
```


```
$ cargo build --release
$ ./target/release/appchain-barnacle --dev --tmp
$ ./target/release/appchain-barnacle purge-chain --dev
$ ./target/release/appchain-barnacle --dev --enable-offchain-indexing true
```
