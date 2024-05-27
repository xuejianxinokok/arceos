DIR_payload=../../payload
APP=hello_app

bin: build
	rust-objcopy --binary-architecture=riscv64 --strip-all -O binary target/riscv64gc-unknown-none-elf/release/${APP} ./${APP}.bin
    # if 输入文件名  of 输出文件名
    # ibs=bytes：一次读入bytes个字节，即指定一个块大小为bytes个字节。
    # obs=bytes：一次输出bytes个字节，即指定一个块大小为bytes个字节。
    # bs=bytes：同时设置读入/输出的块大小为bytes个字节
    # notrunc：不截短输出文件
    # seek=blocks：从输出文件开头跳过blocks个块后再开始复制
	dd if=/dev/zero of=./apps.bin bs=1M count=32
	dd if=./${APP}.bin of=./apps.bin conv=notrunc
	mkdir -p ${DIR_payload}
	mv ./apps.bin ${DIR_payload}/apps.bin

build:
	cargo build --target riscv64gc-unknown-none-elf --release

obj: build
	rust-objcopy --binary-architecture=riscv64 --strip-all -O binary target/riscv64gc-unknown-none-elf/release/${APP} ./${APP}.bin
	cp ./${APP}.bin ${DIR_payload}

dump:
	xxd -ps ./${APP}.bin

clean:
	rm -f ${DIR_payload}/apps.bin
	rm -f ./${APP}.bin

