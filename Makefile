.PHONY: all
all: check
	$(MAKE) --no-print-directory test

.PHONY: test
test:
	cargo test

.PHONY: check
check:
	cargo check
	cargo clippy
	cargo fmt --check

dies_%:
	mkdir -p tests/input/$@
	touch tests/input/$@/input.txt
	touch tests/input/$@/example.txt
	cp dies_n.rs src/$@.rs
	echo "dies!(_$*, 0, 0, 0, 0);" >> tests/tests.rs
	echo "pub mod dies_$*;" >> src/lib.rs
