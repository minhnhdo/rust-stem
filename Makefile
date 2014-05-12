lib:
	rustc src/stem.rs

test:
	rustc --test src/stem.rs

clean:
	$(RM) libstem*.rlib stem

.PHONY: lib test clean
