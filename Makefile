RUSTC ?= rustc

SRC = src/stem.rs

LIBNAME = $(shell $(RUSTC) --crate-file-name $(SRC))
TESTNAME = $(shell $(RUSTC) --test --crate-file-name $(SRC))

lib: $(LIBNAME)

$(LIBNAME):
	$(RUSTC) $(SRC)

test: $(TESTNAME)

$(TESTNAME):
	$(RUSTC) --test $(SRC)

clean:
	$(RM) $(LIBNAME) $(TESTNAME)

.PHONY: lib test clean
