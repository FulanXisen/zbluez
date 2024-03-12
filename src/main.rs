use std::error::Error;

use tonic::transport::Server;

use crate::{simple_interface::grpc_bluez::simple_bluetooth_interface_server::{
    SimpleBluetoothInterfaceServer,
}, simple_server::BluetoothServer};

mod simple_interface;
mod simple_server;
mod simple_bluez;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let server: BluetoothServer = BluetoothServer::new().await?;

    println!("SimpleBluetoothInterfaceServer listening on {}", addr);

    Server::builder()
        .add_service(SimpleBluetoothInterfaceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
