.PHONY: devcontainer-slim devcontainer test

devcontainer-slim:
	./scripts/dockerbuild.sh all
	docker build -f ./src/embed/Dockerfile . -t hcr-devcontainer-slim:latest
	docker image prune -f

devcontainer:
	docker build . --target devcontainer -t hcr-devcontainer:latest
	docker image prune -f

test: devcontainer
	docker run --privileged --rm -it hcr-devcontainer:latest cargo test
