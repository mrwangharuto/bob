# bob.fun

## build

 bash
docker build -t rust-dev .
docker run -it -v $(pwd):/app rust-dev
cargo build --locked --target wasm32-unknown-unknown -p bob_miner_v2 --release
cargo build --locked --target wasm32-unknown-unknown -p bob-minter-v2 --release
ic-wasm target/wasm32-unknown-unknown/release/bob-minter-v2.wasm -o target/wasm32-unknown-unknown/release/bob-minter-v2.wasm metadata candid:service -f minter-v2/bob.did -v public
gzip -nf9 target/wasm32-unknown-unknown/release/bob-minter-v2.wasm
sha256sum  target/wasm32-unknown-unknown/release/*.wasm.gz

bob Ledger forged from the source of truth, the DFINITY https://github.com/dfinity/ic, commit 2190613d3b5bcd9b74c382b22d151580b8ac271a.

https://download.dfinity.systems/ic/2190613d3b5bcd9b74c382b22d151580b8ac271a/canisters/ic-icrc1-ledger.wasm.gz