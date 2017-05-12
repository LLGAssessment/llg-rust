RUSTC=rustc
RUSTC_OPTS=-O

llg: llg.rs
	$(RUSTC) $(RUSTC_OPTS) "$<"
clean:
	rm llg
