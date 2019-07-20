# rustsdk

Rust based SDK for Gaia Pipelines

# generated

```
brew install protobuf
```

```
cargo install protobuf-codegen
```

```
cargo install grpcio-compiler
```

Then generating the code from the Gaia protoc file:

```
protoc --rust_out=./src/protoc --grpc_out=./src/protoc --plugin=protoc-gen-grpc=`which grpc_rust_plugin` plugin.proto
```
