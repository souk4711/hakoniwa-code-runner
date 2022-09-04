.PHONY: container-slim devcontainer prodcontainer test start-server

default: test

container-slim:
	./scripts/dockerbuild.sh all
	docker build -f ./src/embed/Dockerfile . -t hcr-container-slim:latest

devcontainer:
	docker build . --target devcontainer -t hcr-devcontainer:latest

prodcontainer: devcontainer
	docker build . --target devcontainer-builder -t hcr-devcontainer-builder:latest
	docker build . --target prodcontainer -t hcr-prodcontainer:latest

test: devcontainer
	docker run --privileged --rm -it hcr-devcontainer:latest cargo test

start-server: prodcontainer
	docker run --privileged --rm -it -p 8080:8080 hcr-prodcontainer:latest hakoniwa-code-runner start
