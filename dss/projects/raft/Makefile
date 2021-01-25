AUTHOR = github.com/yeluyang
PROJECT = yeluyang_raft_rs

ENV_NAME = $(PROJECT)_env_devel
ENV_IMAGE = yeluyang/rust_grpc
ENV_GOPATH = /go
ENV_WD = $(ENV_GOPATH)/src/$(AUTHOR)/$(PROJECT)

.PHONY: all
all: debug

.PHONY: env
env:
	-docker run -itd --net=host --name $(ENV_NAME) \
		-v /etc/resolv.conf:/etc/resolv.conf \
		-v /var/run/docker.sock:/var/run/docker.sock \
		-v $(shell pwd):$(ENV_WD) -w $(ENV_WD) \
		$(ENV_IMAGE) bash || docker start $(ENV_NAME)
	docker exec -it $(ENV_NAME) bash

.PHONY: debug
debug:
	cargo build

.PHONY: build
build: check
	cargo build --release
