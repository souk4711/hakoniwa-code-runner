# Hakoniwa Code Runner

Run code snippets in a variety of languages over network.


## Installation

### Cargo

* Install `libseccomp` & `protobuf-compiler`.
* Install `rust toolchain`.
* Run `cargo install hakoniwa-code-runner`.


## Usage

```sh
$ hakoniwa-code-runner start -c app.toml
2022-09-05T06:51:32.207731Z  INFO hcr::server: listening on 127.0.0.1:8080
```

More examples can be found in [examples](./examples/). E.g.

* [Run server on host](./examples/run-server-on-host/)
* [Run server inside docker container](./examples/run-server-inside-docker-container/)


## Development

After checking out the repo, run `make devcontainer` to build a container for
development. Then, run `make test` to run the tests. Or run `make start-server`
to start a `hakoniwa-code-runner` listening on 8080.


## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
