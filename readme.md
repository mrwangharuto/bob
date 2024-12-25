# bob.fun

## build

 bash
docker build -t rust-dev .
docker run -it -v $(pwd):/app rust-dev
cargo build --target wasm32-unknown-unknown -p bob_miner_v2 --release
cargo build --target wasm32-unknown-unknown -p bob-minter-v2 --release
gzip -f target/wasm32-unknown-unknown/release/bob-minter-v2.wasm
sha256sum  target/wasm32-unknown-unknown/release/*.wasm.gz
