pkg/fastblur_bg.wasm: src/lib.rs
	wasm-pack build --scope bestminr --target web --release
	# sed -i ".bak" "1s/\g';/g.wasm\';/" pkg/fastblur.js

build-wasm: pkg/fastblur_bg.wasm

copy-wasm: pkg/fastblur_bg.wasm
	cp -r pkg ~/work/blueberry/node_modules/@bestminr/fastblur
