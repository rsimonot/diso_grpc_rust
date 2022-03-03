use diso_grpc_v2::disogrpc;
use disogrpc::hand_shake_client::HandShakeClient;
use disogrpc::HandShakeInit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://[::1]:5000")
    .connect()
    .await?;
    // creating gRPC client from channel
    let mut client = HandShakeClient::new(channel);
    // creating a new Request
    let request = tonic::Request::new(
        HandShakeInit {
           name:String::from("DISOv2")
        },
    );
    // sending request and waiting for response
    let response = client.perform_hand_shake(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}