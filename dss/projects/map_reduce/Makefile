AUTHOR = github.com/yeluyang
PROJECT = map_reduce

ENV_NAME = $(PROJECT)_env_devel
ENV_IMAGE = yeluyang/rust_grpc
ENV_GOPATH = /go
ENV_WD = $(ENV_GOPATH)/src/$(AUTHOR)/$(PROJECT)

GRPC_PROTO_FILES = assets/pb/map_reduce.proto
GRPC_OUTPUT_DIR = src/rpc/grpc
GRPC_OUTPUT_FILES = $(GRPC_OUTPUT_DIR)/rpc.rs $(GRPC_OUTPUT_DIR)/rpc_grpc.rs

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
debug: $(GRPC_OUTPUT_FILES)
	cargo build

.PHONY: build
build: $(GRPC_OUTPUT_FILES) check
	cargo build --release

.PHONY: rpc
rpc $(GRPC_OUTPUT_FILES): $(GRPC_PROTO_FILES)
	@-mkdir -p $(GRPC_OUTPUT_DIR)
	protoc --rust_out=$(GRPC_OUTPUT_DIR) --grpc_out=$(GRPC_OUTPUT_DIR) \
	--plugin=protoc-gen-grpc=$(shell which grpc_rust_plugin) \
	$(GRPC_PROTO_FILES)
