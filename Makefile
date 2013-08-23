lib: src/stem.rc src/stem.rs
	rustc --out-dir . src/stem.rc

check: lib
	rustc -L . --out-dir . tests/test.rs
	./test
	rm test

.PHONY: lib check
