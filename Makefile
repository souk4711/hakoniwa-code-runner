.PHONY: devcontainer-lang devcontainer test start-server

default: test

devcontainer-lang:
	./scripts/dockerbuild.sh all
	docker build -f ./.devcontainer/Dockerfile . -t hcr-devcontainer-lang:latest

devcontainer:
	docker build -f ./Dockerfile . -t hcr-devcontainer:latest

test: devcontainer
	docker run --privileged --group-add keep-groups --rm -it hcr-devcontainer:latest cargo test

start-server: devcontainer
	docker run --privileged --group-add keep-groups --rm -it -p 8080:8080 --stop-signal SIGINT hcr-devcontainer:latest cargo run start -c ./.devcontainer/app.toml
