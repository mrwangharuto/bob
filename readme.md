# bob.fun

## build

 bash
docker build -t rust-dev .
docker run -it -v $(pwd):/app rust-dev
cargo build --locked --target wasm32-unknown-unknown -p bob_miner_v2 --release
cargo build --locked --target wasm32-unknown-unknown -p bob-minter-v2 --release
gzip -nf9 target/wasm32-unknown-unknown/release/bob-minter-v2.wasm
sha256sum  target/wasm32-unknown-unknown/release/*.wasm.gz

bob Ledger forged from the source of truth, the DFINITY https://github.com/dfinity/ic, commit d4ee25b0865e89d3eaac13a60f0016d5e3296b31.