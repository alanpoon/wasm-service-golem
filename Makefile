run:
	cargo build --release --target wasm32-unknown-unknown
	cp index.html www
	cp favicon.ico www
	cp sw.js www
	cp target/wasm32-unknown-unknown/release/wasm_service.wasm www/app.wasm