lib:
	rustc src/stem.rs

test: lib
	rustc -L . --test src/test.rs

.PHONY: lib test
