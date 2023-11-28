python src/build_levels.py
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./wasm --target web .\target\wasm32-unknown-unknown\release\bosconian.wasm
# Start-Process "http://localhost:8000"
# python -m http.server