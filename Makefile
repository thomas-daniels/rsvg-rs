GIR = gir/target/bin/gir
GIR_SRC = gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
GIR_FILES = gir-files/Gtk-3.0.gir
GIR_SYS = $(rsvg-sys/Gir.toml=rsvg-sys/src/lib.rs)

# Run `gir` generating the bindings
gir : src/auto/mod.rs
gir-sys : rsvg-sys/src/lib.rs
clean :
	rm -rf src/auto
	rm -rf

src/auto/mod.rs : Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir.toml

rsvg-sys/src/lib.rs : rsvg-sys/Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c $< -o $(abspath rsvg-sys) -d gir-files

$(GIR) : $(GIR_SRC)
	rm -f gir/target/bin/gir
	cargo install --path gir --root gir/target
	rm -f gir/target/.crates.toml

$(GIR_SRC) $(GIR_FILES) :
	git submodule update --init