FROM ubuntu:18.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  g++ \
  make \
  file \
  curl \
  ca-certificates \
  python \
  git \
  cmake \
  sudo \
  gdb \
  xz-utils

# FIXME: build the `ptx-linker` instead.
RUN curl -sL https://github.com/denzp/rust-ptx-linker/releases/download/v0.9.0-alpha.2/rust-ptx-linker.linux64.tar.gz | \
  tar -xzvC /usr/bin

RUN curl -sL https://nodejs.org/dist/v9.2.0/node-v9.2.0-linux-x64.tar.xz | \
  tar -xJ

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV RUST_CONFIGURE_ARGS \
  --set build.nodejs=/node-v9.2.0-linux-x64/bin/node \
  --set rust.lld

# Some run-make tests have assertions about code size, and enabling debug
# assertions in libstd causes the binary to be much bigger than it would
# otherwise normally be. We already test libstd with debug assertions in lots of
# other contexts as well
ENV NO_DEBUG_ASSERTIONS=1

ENV WASM_TARGETS=wasm32-unknown-unknown
ENV WASM_SCRIPT python2.7 /checkout/x.py test --target $WASM_TARGETS \
  src/test/run-make \
  src/test/ui \
  src/test/run-pass \
  src/test/compile-fail \
  src/test/mir-opt \
  src/test/codegen-units \
  src/libcore

ENV NVPTX_TARGETS=nvptx64-nvidia-cuda
ENV NVPTX_SCRIPT python2.7 /checkout/x.py test --target $NVPTX_TARGETS \
  src/test/run-make

ENV SCRIPT $WASM_SCRIPT && $NVPTX_SCRIPT