FROM rust:1.70.0-slim AS common

RUN apt-get update && \
    TZ=Asia/Tokyo apt-get install -y --fix-missing tzdata && \
    apt-get install -y --fix-missing \
    build-essential \
    git \
    clang \
    lld \
    cmake \
    libstdc++-10-dev \
    libssl-dev \
    libxxhash-dev \
    zlib1g-dev \
    pkg-config \
    libwebkit2gtk-4.0-dev \
    curl \
    wget \
    locales && \
    rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.73.0

ENV mold_version=v1.4.2
RUN git clone --branch "$mold_version" --depth 1 https://github.com/rui314/mold.git && \
    cd mold && \
    make -j$(nproc) CXX=clang++ && \
    make install && \
    mv /mold/mold /usr/bin/mold && \
    mv /mold/mold-wrapper.so /usr/bin/mold-wrapper.so && \
    make clean

ENV RUST_BACKTRACE=1

COPY . /app
WORKDIR /app

RUN rustup install nightly-2023-09-29 && \
    rustup default nightly-2023-09-29 && \
    rustup component add rust-src clippy rustfmt
RUN mold -run cargo install cargo-edit

CMD [ "/bin/bash" ]
