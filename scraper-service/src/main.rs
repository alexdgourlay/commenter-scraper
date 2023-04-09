mod scraper_service;

// Define module from the 'scraper' protobuf package.
pub mod scraper_proto {
    tonic::include_proto!("scraper");
}

use scraper_proto::scraper_server::ScraperServer;
use scraper_service::ScraperService;
use std::net::SocketAddr;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse()?;
    let scraper_service = ScraperService::default();

    println!("Starting server on port {}", addr.to_string());

    // Start tonic server.
    Server::builder()
        .add_service(ScraperServer::new(scraper_service))
        .serve(addr)
        .await?;

    Ok(())
}
