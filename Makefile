PROTO_DIR=protobuf
PROTOC=protoc

PROTO_DEFS=$(PROTO_DIR)/zeus_data.proto \
	$(PROTO_DIR)/zeus_meta.proto

RUST_OUT_DIR=zeus-server/src/rpc
PROTOC_RUST_PLUGIN=/Users/liurenjie/.cargo/bin/protoc-gen-rust
GRPC_RUST_PLUGIN=/Users/liurenjie/.cargo/bin/grpc_rust_plugin

RUST_GRPC: $(PROTO_DEFS)
	echo "Building rust grpc"
	$(PROTOC) --rust_out=$(RUST_OUT_DIR) \
		--grpc_out=$(RUST_OUT_DIR) \
		--plugin=protoc-gen-grpc=$(GRPC_RUST_PLUGIN) \
		--plugin=protoc-gen-rust=$(PROTOC_RUST_PLUGIN) \
		$(PROTO_DEFS)

RUST_BUILD: RUST_GRPC
	echo "Building rust"
	cd zeus-server && cargo build

JAVA_BUILD:
	echo "Building java"
	cd zeus-java && mvn clean install

all: RUST_BUILD JAVA_BUILD

.PHONY: clean

clean:
	echo "Cleaning rust"
	cd zeus-server && cargo clean
	echo "Cleaning java"
	cd zeus-java && mvn clean