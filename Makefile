RUSTC=/usr/local/bin/rustc

all: main

#main.rs

.SUFFIXES: .rs
.rs:
	$(RUSTC) $<

#.SUFFIXES: .scm .o
#.scm.o:
#	${CSC} -c $<

clean:
	rm main

.PHONY: clean

