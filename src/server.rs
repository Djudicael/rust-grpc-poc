use std::fmt::format;

use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentRequest, BtcPaymentResponse};
use tonic::Request;
use tonic::{transport::Server, Response, Status};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BitcoinService {}
#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payment(
        &self,
        request: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let reply = BtcPaymentResponse {
            successfull: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr).into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let addr = "[::1]:50051".parse()?;
    let btc_service = BitcoinService::default();

    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;
    Ok(())
}
