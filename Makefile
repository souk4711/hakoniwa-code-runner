.PHONY: devcontainer devcontainer-slim

devcontainer:
	docker build . -t hcr-devcontainer:latest

devcontainer-slim:
	./scripts/dockerbuild.sh all
	set -xe; cd ./src/embed; docker build . -t hcr-devcontainer-slim:latest
