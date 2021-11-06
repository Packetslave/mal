
STEPS = step0_repl

all: $(STEPS)

dist: mal

mal: stepA_mal
	cp $< $@

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS = Cargo.toml step0_repl.rs

step0_repl: $(STEP0_DEPS)

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal
