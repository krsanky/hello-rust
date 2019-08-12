RUSTC=/usr/local/bin/rustc
SRCDIR =  src

all: main

main: $(SRCDIR)/$@.rs
	$(RUSTC) $(SRCDIR)/$@.rs

.SUFFIXES: .rs
.rs:
	$(RUSTC) $<

clean:
	rm main

.PHONY: clean

