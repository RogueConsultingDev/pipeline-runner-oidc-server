PROJECT_NAME := "pipeline-runner-oidc-server"
DOCKER_REPO := "rogueconsulting/pipeline-runner-oidc-server"

run:
    cargo-run

lint:
    pre-commit run --all-files

test:
    cargo test

docker-build:
    docker build --ssh default --tag "{{ DOCKER_REPO }}:$(just docker-tag)" .

docker-run: docker-build
    docker run --rm -it -p 8080:8080 --name "{{ PROJECT_NAME }}" "{{ DOCKER_REPO }}:$(just docker-tag)"

docker-push: docker-build
    docker push "{{ DOCKER_REPO }}:$(just docker-tag)"

@docker-tag:
    git describe --always HEAD | sed 's/^v//'
