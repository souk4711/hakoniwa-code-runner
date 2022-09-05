FROM hcr-devcontainer:latest

USER 1000:1000
WORKDIR /home/hako/hcr

RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
COPY --chown=1000:1000 ./Cargo.toml .
COPY --chown=1000:1000 ./Cargo.lock .

RUN cargo build
RUN cargo build --release

COPY --chown=1000:1000 . .
