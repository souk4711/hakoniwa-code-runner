.PHONY: devcontainer-slim devcontainer test

devcontainer-slim:
	./scripts/dockerbuild.sh all
	set -xe; cd ./src/embed; docker build . -t hcr-devcontainer-slim:latest

devcontainer:
	docker build . -t hcr-devcontainer:latest

test: devcontainer
	docker run --privileged --rm -it hcr-devcontainer:latest cargo test
