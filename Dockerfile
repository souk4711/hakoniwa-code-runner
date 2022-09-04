FROM hcr-devcontainer-slim:latest as devcontainer

WORKDIR /usr/src/hcr

RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN cargo build
RUN cargo build --release

COPY . .
