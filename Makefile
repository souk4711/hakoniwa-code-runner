.PHONY: devcontainer devcontainer-ci test start-server

default: test

devcontainer:
	./scripts/dockerbuild.sh all
	docker build -f ./.devcontainer/Dockerfile . -t hcr-devcontainer:latest

devcontainer-ci:
	docker build -f ./Dockerfile . -t hcr-devcontainer-ci:latest

test: devcontainer-ci
	docker run --privileged --group-add keep-groups --rm -it hcr-devcontainer-ci:latest cargo test

start-server: devcontainer-ci
	docker run --privileged --group-add keep-groups --rm -it -p 8080:8080 --stop-signal SIGINT hcr-devcontainer-ci:latest cargo run start -c ./.devcontainer/app.toml
