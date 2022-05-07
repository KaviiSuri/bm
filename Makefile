.PHONY: examples

examples: ./examples/fib.bm ./examples/123.bm

build: 
	cargo build

./examples/fib.bm: build ./examples/fib.basm
	./target/debug/basm ./examples/fib.basm ./examples/fib.bm

./examples/123.bm: build ./examples/123.basm
	./target/debug/basm ./examples/123.basm ./examples/123.bm
