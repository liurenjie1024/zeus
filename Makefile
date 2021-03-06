PROTO_DIR=protobuf
PROTOC=protoc

PROTO_DEFS=$(PROTO_DIR)/zeus_data.proto \
	$(PROTO_DIR)/zeus_meta.proto \
	$(PROTO_DIR)/zeus_expr.proto \
	$(PROTO_DIR)/zeus_plan.proto \
	$(PROTO_DIR)/zeus_simple_format.proto \
	$(PROTO_DIR)/zeus_blizard_format.proto

RUST_OUT_DIR=zeus-server/src/rpc
PROTOC_RUST_PLUGIN=`which protoc-gen-rust`
GRPC_RUST_PLUGIN=`which grpc_rust_plugin`


all: RUST_BUILD JAVA_BUILD

RUST_GRPC_GEN: $(PROTO_DEFS)
	echo "Building rust grpc"
	$(PROTOC) --rust_out=$(RUST_OUT_DIR) \
	    -I$(PROTO_DIR) \
		--grpc_out=$(RUST_OUT_DIR) \
		--plugin=protoc-gen-grpc=$(GRPC_RUST_PLUGIN) \
		--plugin=protoc-gen-rust=$(PROTOC_RUST_PLUGIN) \
		$(PROTO_DEFS)

RUST_BUILD:
	echo "Building rust"
	export RUSTFLAGS="-A renamed_and_removed_lints"
	cd zeus-server && cargo build

JAVA_BUILD:
	echo "Building java"
	cd zeus-java && mvn clean install

.PHONY: clean

clean:
	echo "Cleaning rust"
	cd zeus-server && cargo clean
	echo "Cleaning java"
	cd zeus-java && mvn clean