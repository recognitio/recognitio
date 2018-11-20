build:
	cargo-web build

copy: build
	cp target/wasm32-unknown-unknown/release/recognitio.js static/scripts
	cp target/wasm32-unknown-unknown/release/recognitio.wasm static/scripts

test: copy
	cargo-web start

clean:
	rm static/scripts/recognitio.js
	rm static/scripts/recognitio.wasm	
