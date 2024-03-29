# Scripts

## scripts/install.sh

Install a specific language development tools to `/opt` directory in local machine:

```console
# Pick all
$ ./scripts/install.sh all

# Pick a language
$ ./scripts/install.sh cpp
$ ./scripts/install.sh c
$ ./scripts/install.sh cpp
$ ./scripts/install.sh d
$ ./scripts/install.sh erlang
$ ./scripts/install.sh go
$ ./scripts/install.sh haskell
$ ./scripts/install.sh java
$ ./scripts/install.sh javascript
$ ./scripts/install.sh python
$ ./scripts/install.sh ruby
$ ./scripts/install.sh rust
$ ./scripts/install.sh scala
$ ./scripts/install.sh typescript
```

Run it:

```console
$ cargo run start -c ./.devcontainer/app.toml
```

## scripts/dockerbuild.sh

Build a specific language development environment using docker:

```console
# Pick all
$ ./scripts/dockerbuild.sh all

# Pick a language
$ ./scripts/dockerbuild.sh cpp
$ ./scripts/dockerbuild.sh c
$ ./scripts/dockerbuild.sh cpp
$ ./scripts/dockerbuild.sh d
$ ./scripts/dockerbuild.sh erlang
$ ./scripts/dockerbuild.sh go
$ ./scripts/dockerbuild.sh haskell
$ ./scripts/dockerbuild.sh java
$ ./scripts/dockerbuild.sh javascript
$ ./scripts/dockerbuild.sh python
$ ./scripts/dockerbuild.sh ruby
$ ./scripts/dockerbuild.sh rust
$ ./scripts/dockerbuild.sh scala
$ ./scripts/dockerbuild.sh typescript
```

Pack them into one image:

```console
$ make devcontainer-lang-all
```

Add `hakoniwa-code-runner` (build from source) to the image:

```console
$ make devcontainer
```

Run it:

```console
$ make start-server
```
