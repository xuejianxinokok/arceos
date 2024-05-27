DIR_payload=../../payload


bin: build
	rust-objcopy --binary-architecture=riscv64 --strip-all -O binary target/riscv64gc-unknown-none-elf/release/hello_app ./hello_app.bin
	dd if=/dev/zero of=./apps.bin bs=1M count=32
	dd if=./hello_app.bin of=./apps.bin conv=notrunc
	mkdir -p ${DIR_payload}
	mv ./apps.bin ${DIR_payload}/apps.bin

build:
	cargo build --target riscv64gc-unknown-none-elf --release


clean:
	rm -f ${DIR_payload}/apps.bin
	rm -f ./hello_app.bin
	
