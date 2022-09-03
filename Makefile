.PHONY: devcontainer

devcontainer:
	./scripts/dockerbuild.sh all
	set -xe; cd ./src/embed; docker build . -t hcr-devcontainer:latest
