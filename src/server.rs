use tonic::{transport::Server, Request, Response, Status};

use diso_grpc_v2::disogrpc;
use disogrpc::hand_shake_server::{HandShake, HandShakeServer};    // Genarated by code in disogrpc.rs
use disogrpc::{HandShakeReply, HandShakeInit};

// defining a struct for our service
#[derive(Default)]
pub struct HS {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl HandShake for HS {
    // our rpc impelemented as function
    async fn perform_hand_shake(&self, request : Request<HandShakeInit>)->Result<Response<HandShakeReply>, Status> {
        // returning a response as HandShakeReply message as defined in .proto
        Ok(Response::new(HandShakeReply {
            // reading data from request which is awrapper around our HandShakeInit message defined in .proto
             message:format!("hello {}", request.get_ref().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:5000".parse().unwrap();
    // creating a service
    let h_s = HS::default();
    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(HandShakeServer::new(h_s))
        .serve(addr)
        .await?;
    Ok(())
}