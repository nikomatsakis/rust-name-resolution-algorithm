RUSTC=/home/nmatsakis/versioned/rust-m/build/i686-unknown-linux-gnu/stage2/bin/rustc

all:
	${RUSTC} src/resolve.rs

test:
	${RUSTC} --test --cfg debug src/resolve.rs;
	src/thir
