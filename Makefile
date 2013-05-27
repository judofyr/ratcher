.PHONY: clean

all: test ratcher

ratcher: ratcher.rs
	rustc ratcher.rs

test-ratcher: ratcher.rs
	rustc --test ratcher.rs -o test-ratcher

test: test-ratcher
	./test-ratcher

clean:
	rm -rf ratcher{,.dSYM}
	rm -rf test-ratcher{,.dSYM}

