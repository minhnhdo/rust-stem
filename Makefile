RUSTC ?= rustc

SRC = src/stem.rs

TARGET = $(shell $(RUSTC) --crate-file-name $(SRC))
TESTNAME = $(shell $(RUSTC) --test --crate-file-name $(SRC))

target: $(TARGET)

$(TARGET):
	$(RUSTC) $(SRC)

test: $(TESTNAME)

$(TESTNAME):
	$(RUSTC) --test $(SRC)

clean:
	$(RM) $(TARGET) $(TESTNAME)

.PHONY: target test clean
