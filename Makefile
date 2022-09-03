.PHONY: devcontainer-slim

devcontainer-slim:
	./scripts/dockerbuild.sh all
	set -xe; cd ./src/embed; docker build . -t hcr-devcontainer-slim:latest
