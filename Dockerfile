FROM rust:latest

# Install dependencies
RUN apt-get update && apt-get install -y php-dev musl-tools clang libclang-dev

# Add the ARM64 target for Rust
RUN rustup target add aarch64-unknown-linux-gnu

# Set up cross-compilation toolchain
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
ENV AR_aarch64_unknown_linux_gnu=aarch64-linux-gnu-ar

WORKDIR /usr/src/php_extension

RUN rm -rf ./target
RUN rm -rf ./output

# Copy the source code
COPY . .

# Build the extension for the Linux ARM64 target
CMD ["cargo", "build", "--release", "--target", "aarch64-unknown-linux-gnu"]