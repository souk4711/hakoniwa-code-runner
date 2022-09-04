.PHONY: devcontainer-slim devcontainer test start-server

default: test

devcontainer-slim:
	./scripts/dockerbuild.sh all
	docker build -f ./src/embed/Dockerfile . -t hcr-devcontainer-slim:latest
	docker image prune -f

devcontainer:
	docker build . --target devcontainer -t hcr-devcontainer:latest
	docker image prune -f

test: devcontainer
	docker run --privileged --rm -it hcr-devcontainer:latest cargo test

prodcontainer:
	docker build . --target prodcontainer -t hcr-prodcontainer:latest
	docker image prune -f

start-server: prodcontainer
	docker run --privileged --rm -it -p 8080:8080 hcr-prodcontainer:latest
