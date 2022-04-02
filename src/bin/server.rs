use rocketmq::pb::messaging_service_server::MessagingServiceServer;
use rocketmq::service::ServerService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:5001".parse().unwrap();
    let service = ServerService::default();
    println!("Server listens {}", address);
    Server::builder()
        .add_service(MessagingServiceServer::new(service))
        .serve(address)
        .await?;
    Ok(())
}
