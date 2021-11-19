
# furpack
A deadsimple Linux-only packer that uses LZ4 compression and AES-GCM encryption written entirely in Rust. The stub is compiled and uses the `include_bytes!` builtin macro to directly embed data into the stub.

Requires nightly rust.




## Features

- LZ4 Compression
- AES-GCM (with AES-NI) for authenticated encryption of the target binary
- `no_std` stub, less than 14kb with a binary


## Usage/Examples

```bash
cargo run --release --bin builder -- a.out      # prepare the target binary
cargo +nightly build --release --bin stub       # create the packed binary
strip target/release/stub                       # strip the stub
cp target/release/stub packed_binary            # copy the stub to the current dir
```

