lib:
	rustc src/stem.rs

test: lib
	rustc -L . --test src/test.rs

clean:
	$(RM) libstem*.rlib test

.PHONY: lib test clean
