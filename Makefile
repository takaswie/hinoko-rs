#!/usr/bin/make -f

GIR_EXEC = gir/target/release/gir

.PHONY: all clean test

all: hinoko-sys hinoko

clean:
	rm -rf gir-files/Hinoko-0.0.gir
	rm -rf hinoko-sys
	rm -rf hinoko/src/auto hinoko/target hinoko/Cargo.lock

gir/Cargo.toml:
	git submodule update --init gir

$(GIR_EXEC): gir/Cargo.toml
	cd gir && cargo build --release

gir-files/GLib-2.0.toml:
	git submodule update --init gir-files

gir-files/Hinoko-0.0.gir: Hinoko-0.0.gir gir-files/GLib-2.0.toml
	cp Hinoko-0.0.gir gir-files/Hinoko-0.0.gir

hinoko-sys/src: conf/gir-hinoko-sys.toml gir-files/Hinoko-0.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c conf/gir-hinoko-sys.toml -d gir-files -m sys -o hinoko-sys

hinoko-sys: hinoko-sys/src

hinoko/src/auto: conf/gir-hinoko.toml gir-files/Hinoko-0.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c conf/gir-hinoko.toml -d gir-files -m normal -o hinoko

hinoko: hinoko/src/auto hinoko/src/lib.rs hinoko/Cargo.toml hinoko-sys
