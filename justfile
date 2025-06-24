dev:
    cargo watch -c -x "clippy" -x "nextest run"

docker-build:
    docker image build -t apt-repo-server .

docker-run:
    docker run --rm --name apt-repo-server -p 8088:3000 -t apt-repo-server 

bash:
    docker exec -it apt-repo-server bash

check:
    curl -v localhost:8088/health_check

debug-docker-ports:
    docker port apt-repo-server

fiddle:
    cargo clean -p debian-packaging && cargo build

