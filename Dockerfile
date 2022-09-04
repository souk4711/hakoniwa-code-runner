# devcontainer
FROM hcr-container-slim:latest as devcontainer

WORKDIR /usr/src/hcr

RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN cargo build
RUN cargo build --release

COPY . .


# devcontainer-builder
FROM hcr-devcontainer:latest as devcontainer-builder
WORKDIR /usr/src/hcr
RUN cargo install --path .


# prodcontainer
FROM hcr-container-slim:latest as prodcontainer
COPY --from=devcontainer-builder /usr/local/cargo/bin/hakoniwa-code-runner /usr/local/bin/hakoniwa-code-runner
EXPOSE 8080
CMD ["hakoniwa-code-runner", "--version"]
