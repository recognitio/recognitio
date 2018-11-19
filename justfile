build:
	cargo-web build

copy: build
	cp target/wasm32-unknown-unknown/release/hypertoss.js static/scripts
	cp target/wasm32-unknown-unknown/release/hypertoss.wasm static/scripts

test: copy
	cargo-web start

clean:
	rm static/scripts/hypertoss.js
	rm static/scripts/hypertoss.wasm	
