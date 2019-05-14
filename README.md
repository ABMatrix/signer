A simple signer for ethereum and Abos.

# Feature

## interfaces

- sign_message

- sign_transaction

## examples

### ETH

```
let message = [1;256];
let priv_key = PrivKey::from_str("110258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6").unwrap();
let out = Eth::sign_message(&priv_key, &message.to_vec());
```

### ABOS

```
let message = [1;256];
let priv_key = PrivKey::from_str("110258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6").unwrap();
let out = ABos::sign_message(&priv_key, &message.to_vec());
```

# Hacker

## Dependences

- [protobuf 3.5.1](https://github.com/google/protobuf/releases)
- [rust-protobuf v2.0.4](https://github.com/stepancheg/rust-protobuf)

## Usage

1. Install google protoc

2. Install rust plugin

```
$ cargo install protobuf-codegen --vers 2.0.4
```

3. You can start hack, alter protos than run `./create_protobuf.sh`