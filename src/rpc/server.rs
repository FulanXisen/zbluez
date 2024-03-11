use grpc_bluez::simple_bluetooth_interface_server::{
    SimpleBluetoothInterface, SimpleBluetoothInterfaceServer,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod grpc_bluez {
    include!("grpc_bluez.rs");
}

use grpc_bluez::{
    SimpleCommandRequest, SimpleConnectRequest, SimpleDisconnectRequest, SimplePlayMediaRequest,
    SimpleReply,
};
#[derive(Default)]
pub struct Placeholder {}

#[tonic::async_trait]
impl SimpleBluetoothInterface for Placeholder {
    async fn simple_connect(
        &self,
        request: tonic::Request<SimpleConnectRequest>,
    ) -> std::result::Result<tonic::Response<SimpleReply>, tonic::Status> {
        todo!()
    }

    async fn simple_disconnect(
        &self,
        request: tonic::Request<SimpleDisconnectRequest>,
    ) -> std::result::Result<tonic::Response<SimpleReply>, tonic::Status> {
        todo!()
    }

    async fn simple_play_media(
        &self,
        request: tonic::Request<SimplePlayMediaRequest>,
    ) -> std::result::Result<tonic::Response<SimpleReply>, tonic::Status> {
        todo!()
    }

    async fn simple_command(
        &self,
        request: tonic::Request<SimpleCommandRequest>,
    ) -> std::result::Result<tonic::Response<SimpleReply>, tonic::Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter: Placeholder = Placeholder::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(SimpleBluetoothInterfaceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
