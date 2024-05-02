FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN echo "[source.crates-io]\n\
replace-with = 'rsproxy-sparse'\n\
[source.rsproxy]\n\
registry = \"https://rsproxy.cn/crates.io-index\"\n\
[source.rsproxy-sparse]\n\
registry = \"sparse+https://rsproxy.cn/index/\"\n\
[registries.rsproxy]\n\
index = \"https://rsproxy.cn/crates.io-index\"\n\
[net]\n\
git-fetch-with-cli = true\n" >> $CARGO_HOME/config
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt install -y openssl
COPY --from=builder /app/target/release/web3-sharp /web3-sharp
VOLUME /data/configs
CMD ["./web3-sharp","--conf","/data/configs/config.toml"]