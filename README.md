# rustsdk

Rust based SDK for Gaia Pipelines

# generated

```
cargo install protobuf-codegen
```

```
cargo install grpcio-compiler
```

Then generating the code from the Gaia protoc file:

```
protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` plugin.proto
```
