RUSTC=/home/nmatsakis/versioned/rust-m/build/i686-unknown-linux-gnu/stage2/bin/rustc

all:
	${RUSTC} --test src/resolve.rs
	./resolve
