.PHONY: clean all

OBJ = ratcher

all: test ${OBJ}

${OBJ}: ${OBJ}.rs
	rustc ${OBJ}.rs

test-${OBJ}: ${OBJ}.rs
	rustc --test ${OBJ}.rs -o test-${OBJ}

test: test-${OBJ}
	./test-${OBJ}

clean:
	rm -rf ${OBJ}{,.dSYM}
	rm -rf test-${OBJ}{,.dSYM}

