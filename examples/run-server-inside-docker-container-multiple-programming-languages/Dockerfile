FROM hcr-example-multi-go:1.19           as go-builder
FROM hcr-example-multi-python:3.10.6     as python-builder


FROM rust:bullseye

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


COPY --from=go-builder      /opt/go-1.19/       /opt/go-1.19/
COPY --from=python-builder  /opt/python-3.10.6/ /opt/python-3.10.6/


RUN set -xe && \
  cargo install hakoniwa-code-runner

WORKDIR /opt/hcr
COPY . .
USER 1000:1000
EXPOSE 8080
CMD ["hakoniwa-code-runner", "start", "-c", "app.toml"]
