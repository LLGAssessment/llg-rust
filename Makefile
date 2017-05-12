RUSTC=rustc

llg: llg.rs
	$(RUSTC) "$<"
clean:
	rm llg
