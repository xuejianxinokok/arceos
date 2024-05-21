# docker rmi -f arceos
# docker build -t arceos .
# docker run --name os2 --hostname os2  -v D:/work20220906/gitee/rusttest/:/root/work -w /root/work  -d arceos  sleep infinity 
# docker exec -it os2 /bin/bash
# docker rm -f os2
# 6.74GB 
FROM ubuntu:22.04
LABEL MAINTAINER xuejianxinokok@163.com
ARG QEMU_VERSION=8.2.4
ARG HOME=/root

# 0. Install general tools
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install -y  \
        curl \
        git \
        python3 \
        wget  \
        autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
        gawk build-essential bison flex texinfo gperf libtool patchutils bc \
        zlib1g-dev libexpat-dev  \
        ninja-build pkg-config libglib2.0-dev libpixman-1-dev libsdl2-dev \
        libslirp-dev libclang-dev \
        python3-distutils gdb-multiarch tmux


# 1. Set up QEMU RISC-V
# - https://learningos.github.io/rust-based-os-comp2022/0setup-devel-env.html#qemu
# - https://www.qemu.org/download/
# - https://wiki.qemu.org/Documentation/Platforms/RISCV
# - https://risc-v-getting-started-guide.readthedocs.io/en/latest/linux-qemu.html

# 1.1. Download source
WORKDIR ${HOME}

# Download&Install cross-musl-based toolchains
RUN wget https://musl.cc/aarch64-linux-musl-cross.tgz --no-check-certificate && \
    wget https://musl.cc/riscv64-linux-musl-cross.tgz --no-check-certificate && \
    wget https://musl.cc/x86_64-linux-musl-cross.tgz  --no-check-certificate && \
    tar zxf riscv64-linux-musl-cross.tgz && \
    tar zxf aarch64-linux-musl-cross.tgz && \
    tar zxf x86_64-linux-musl-cross.tgz 


# exec below command in bash OR add below info in ~/.bashrc
RUN echo 'PATH=`pwd`/x86_64-linux-musl-cross/bin:`pwd`/aarch64-linux-musl-cross/bin:`pwd`/riscv64-linux-musl-cross/bin:$PATH' >> ~/.bashrc  


RUN wget https://download.qemu.org/qemu-${QEMU_VERSION}.tar.xz && \
    tar xJf qemu-${QEMU_VERSION}.tar.xz


# 1.3. Build and install from source
WORKDIR ${HOME}/qemu-${QEMU_VERSION}
# Python's ensurepip module is not found
RUN curl https://bootstrap.pypa.io/get-pip.py -o get-pip.py && \
    python3 get-pip.py

RUN ./configure --target-list=aarch64-softmmu,aarch64-linux-user,riscv64-softmmu,riscv64-linux-user,x86_64-softmmu,x86_64-linux-user --enable-slirp && \
    make -j$(nproc) && \
    make install && \
#    apt-get clean && \
#    rm -rf /var/lib/apt/lists/*

# 1.4. Clean up
WORKDIR ${HOME}
RUN rm -rf qemu-${QEMU_VERSION} qemu-${QEMU_VERSION}.tar.xz *-linux-musl-cross.tgz



# 2. Set up Rust
# - https://learningos.github.io/rust-based-os-comp2022/0setup-devel-env.html#qemu
# - https://www.rust-lang.org/tools/install
# - https://github.com/rust-lang/docker-rust/blob/master/Dockerfile-debian.template

# 2.1. Install
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=nightly
RUN set -eux; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME;



# 2.3 Env
RUN cargo install cargo-binutils; \
    rustup target add riscv64gc-unknown-none-elf; \
	rustup component add rust-src; \
	rustup component add llvm-tools-preview; \
	rustup component add rustfmt; \
	rustup component add clippy; 

# Sanity checking
RUN qemu-system-riscv64 --version && qemu-riscv64 --version  && \
    qemu-system-aarch64 --version && qemu-aarch64 --version  && \
    qemu-system-x86_64  --version && qemu-x86_64  --version  && \
    rustup              --version && cargo --version &&  rustc --version 

# Ready to go
WORKDIR ${HOME}
