PROJECT_NAME := "pipeline-runner-oidc-server"
IMAGE_NAME := "rogueconsulting/pipeline-runner-oidc-server"

run:
    cargo-run

lint:
    pre-commit run --all-files

test:
    cargo test

docker-build:
    docker build --ssh default --tag "$(just docker-tag)" .

docker-run: docker-build
    docker run --rm -it -p 8080:8080 --name "{{ PROJECT_NAME }}" "$(just docker-tag)"

docker-push: docker-build
    docker push "$(just docker-tag)"

@docker-tag:
    printf "{{ IMAGE_NAME }}:%s\n" "$(git describe --always HEAD | sed 's/^v//')"
