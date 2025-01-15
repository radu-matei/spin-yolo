
.PHONY: lib
lib:
	cd lib && RUSTFLAGS=-Ctarget-feature=+simd128 cargo component build --release
