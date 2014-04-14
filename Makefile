all:
	rustc --test src/resolve.rs
	RUST_LOG=resolve::nameresolution,resolve::test ./resolve

TAGS:
	etags src/*.rs
