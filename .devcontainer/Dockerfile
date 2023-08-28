FROM hcr-devcontainer-lang-d:2.100.0         as d-builder
FROM hcr-devcontainer-lang-erlang:25.0.4     as erlang-builder
FROM hcr-devcontainer-lang-go:1.19           as go-builder
FROM hcr-devcontainer-lang-haskell:ghc-9.4.2 as haskell-builder
FROM hcr-devcontainer-lang-java:openjdk-18   as java-builder
FROM hcr-devcontainer-lang-nodejs:18.8.0     as nodejs-builder
FROM hcr-devcontainer-lang-python:3.10.6     as python-builder
FROM hcr-devcontainer-lang-ruby:3.1.2        as ruby-builder
FROM hcr-devcontainer-lang-rust:1.63.0       as rust-builder
FROM hcr-devcontainer-lang-scala:3.1.3       as scala-builder
FROM rust:bullseye

COPY --from=d-builder       /opt/d-2.100.0/     /opt/d-2.100.0/
COPY --from=erlang-builder  /opt/erlang-25.0.4/ /opt/erlang-25.0.4/
COPY --from=go-builder      /opt/go-1.19/       /opt/go-1.19/
COPY --from=haskell-builder /opt/haskell-9.4.2/ /opt/haskell-9.4.2/
COPY --from=java-builder    /opt/openjdk-18/    /opt/openjdk-18/
COPY --from=nodejs-builder  /opt/nodejs-18.8.0/ /opt/nodejs-18.8.0/
COPY --from=python-builder  /opt/python-3.10.6/ /opt/python-3.10.6/
COPY --from=ruby-builder    /opt/ruby-3.1.2/    /opt/ruby-3.1.2/
COPY --from=rust-builder    /opt/rust-1.63.0/   /opt/rust-1.63.0/
COPY --from=scala-builder   /opt/scala-3.1.3/   /opt/scala-3.1.3/

RUN set -xe && \
  apt-get update && \
  apt-get install -y --no-install-recommends locales && \
  rm -rf /var/lib/apt/lists/* && \
  echo "en_US.UTF-8 UTF-8" > /etc/locale.gen && \
  locale-gen
ENV LANG=en_US.UTF-8 LANGUAGE=en_US:en LC_ALL=en_US.UTF-8

RUN set -xe && \
  apt-get update && \
  apt-get install -y --no-install-recommends libseccomp-dev && \
  apt-get install -y --no-install-recommends protobuf-compiler && \
  rm -rf /var/lib/apt/lists/*

RUN groupadd --gid 1000 hako \
  && useradd --uid 1000 --gid hako --shell /bin/bash --create-home hako
