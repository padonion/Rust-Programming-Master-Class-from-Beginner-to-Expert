FROM mcr.microsoft.com/devcontainers/rust:latest

# include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with cargo config file (.cargo/config.toml)
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld \
    && apt-get autoremove -y && apt-get clean -y
