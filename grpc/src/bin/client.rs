mod generated_helloworld;
use generated_helloworld::helloworld;
use generated_helloworld::helloworld_grpc::GreeterClient;
use grpc::ClientStubExt;

fn main() {
    let client = GreeterClient::new_plain_unix("/tmp/rust_grpc", Default::default()).unwrap();

    let mut req = helloworld::HelloRequest::new();
    req.set_name("GRPC Client".to_string());

    let resp = client
        .say_hello(grpc::RequestOptions::new(), req)
        .join_metadata_result();
    println!("{:?}", futures::executor::block_on(resp));
}
