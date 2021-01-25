GRPC_DIR = rpc/grpc
GRPC_PB = $(GRPC_DIR)/rpc.proto
GRPC_GEN_FILES = $(GRPC_DIR)/rpc.rs $(GRPC_DIR)/rpc_grpc.rs

dep:
	cargo install protobuf-codegen
	cargo install grpcio-compiler

.PHONY: rpc
rpc $(GRPC_GEN_FILES): $(GRPC_PB)
	@-mkdir -p $(GRPC_DIR)
	protoc --rust_out=$(GRPC_DIR) --grpc_out=$(GRPC_DIR) \
	--plugin=protoc-gen-grpc=$(shell which grpc_rust_plugin) \
	$(GRPC_PB)
