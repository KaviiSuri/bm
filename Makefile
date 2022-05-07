.PHONY: examples

examples: ./examples/fib.bm ./examples/123.bm

build: 
	cargo build

./examples/fib.bm: ./examples/fib.basm
	./target/debug/ebasm ./examples/fib.basm ./examples/fib.bm

./examples/123.bm: ./examples/123.basm
	./target/debug/ebasm ./examples/123.basm ./examples/123.bm
