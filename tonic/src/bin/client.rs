use helloworld::greeter_client::GreeterClient;
use helloworld::HelloRequest;

pub mod helloworld;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic_Client".into(),
    });

    let rsp = client.say_hello(request).await?;

    println!("RESPONSE: {:?}", rsp);

    Ok(())
}
