<!-- ABOUT THE PROJECT -->
## About The Project

This repo is a simple gRPC example implementing in Rust programming language. There are a lot of Rust crates for Protobuf and gRPC, `tonic` and `grpc` are the most popular ones. Choosing a suitable crate depends on your project as well as your favorite coding style.


## Structure

There are 2 independent subdirectories: `tonic` and `grpc`. Both of them include example source code for gRPC server and client.
```
|-- grpc
|   |-- build.rs
|   |-- Cargo.lock
|   |-- Cargo.toml
|   |-- proto
|   |   `-- helloworld.proto // proto message
|   `-- src
|       `-- bin
|           |-- client.rs
|           |-- generated_helloworld // generated code from proto message
|           |   |-- helloworld_grpc.rs
|           |   |-- helloworld.rs
|           |   `-- mod.rs
|           `-- server.rs
`-- tonic
    |-- build.rs
    |-- Cargo.lock
    |-- Cargo.toml
    |-- proto
    |   `-- helloworld.proto
    `-- src
        `-- bin
            |-- client.rs
            |-- helloworld.rs // generated code from proto message
            `-- server.rs
```

`build.rs` files in each subdirectory generates Rust code from the proto file `helloworld.proto` whenever you build the project.


<!-- GETTING STARTED -->
## Getting Started

### What is  `tonic`?

[`tonic`](https://crates.io/crates/tonic)  is a fast production-ready gRPC library with async/await support out of the box. It focuses on flexibility and reliability.  `tonic`  has full implementation of gRPC protocols over HTTP/2.  `tonic`  has built-in support for compiling protocol buffer to Rustlang. It also supports unidirectional as well as bidirectional streaming.

### What is  `grpc`?

[`grpc`](https://crates.io/crates/grpc)  is not production-ready but is worth keeping an eye on. The crate has a working gRPC protocol implementation and supports TLS.

### Compare `tonic` vs `grpc`
|Feature  |`tonic`  |`grpc`
|--|--|--|
| Production-ready | Yes | No |
| Full async/await support | Yes | No |
| Request and response streaming | Yes | Yes |
| TLS-based authentication | Yes | Yes |

If you are familiar with asynchronous programming in some programming languages such as C#, Javascript,..., `tonic` may be your suitable choice. Otherwise, if you are used to working with Golang or C++ gRPC, choose `grpc` to make it consistent with your current programming languages (and your projects).

### Prerequisites (for `grpc` only)

`grpc` requires `protoc-gen-rust-grpc` to generate Rust code for proto message. You can install it via cargo
```
# install protoc
 sudo apt install protobuf-compiler

# install protoc-gen-rust-grpc
cargo install grpc-compiler

# install protoc-gen-rust
 cargo install protobuf-codegen
```
`build.rs` files generate Rust code for proto message every build time. But if you want to manually generate it, use `protoc` command (it will generate both protobuf and grpc code)
```
protoc --rust_out=. --rust-grpc_out=. foo.proto
```

<!-- USAGE EXAMPLES -->
## Usage
- Go to `tonic` or `grpc` subdirectory
```
cd tonic
# or
cd grpc
```
- Run server code
```
cargo run --bin server
```
- Open another terminal and run client code
```
cargo run --bin client
```
You should see these messages on your terminals
- Server
```
Greeter server is running
Received request from GRPC Client
```
- Client
```
Ok((Metadata { entries: [MetadataEntry { key: MetadataKey { name: "content-type" }, value: b"application/grpc" }] }, message: "Hello GRPC Client. I'm server", Metadata { entries: [] }))
```


<!-- LICENSE -->
## License
N/A
