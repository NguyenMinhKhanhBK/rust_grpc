mod generated_helloworld;
use generated_helloworld::helloworld;
use generated_helloworld::helloworld_grpc;

pub struct MyGreeter {}

impl helloworld_grpc::Greeter for MyGreeter {
    fn say_hello(
        &self,
        _: grpc::ServerHandlerContext,
        req: grpc::ServerRequestSingle<crate::helloworld::HelloRequest>,
        resp: grpc::ServerResponseUnarySink<crate::helloworld::HelloResponse>,
    ) -> grpc::Result<()> {
        println!("Received request from {}", req.message.get_name());

        let mut hello_rsp = helloworld::HelloResponse::new();
        hello_rsp.set_message(format!("Hello {}. I'm server", req.message.get_name()));
        resp.finish(hello_rsp)
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server
        .http
        .set_unix_addr("/tmp/rust_grpc".to_owned())
        .unwrap();
    server.add_service(helloworld_grpc::GreeterServer::new_service_def(
        MyGreeter {},
    ));
    let _server = server.build().unwrap();
    println!("Greeter server is running");

    loop {
        std::thread::park();
    }
}
