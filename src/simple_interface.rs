use crate::simple_server::BluetoothServer;
use grpc_bluez::simple_bluetooth_interface_server::SimpleBluetoothInterface;
use tonic::{Response, Status};

pub mod grpc_bluez {
    include!("rpc/grpc_bluez.rs");
}

use grpc_bluez::{
    SimpleCommandRequest, SimpleConnectRequest, SimpleDisconnectRequest, SimplePlayMediaRequest,
    SimplePowerResetRequest, SimpleRemoveRequest, SimpleReply,
};

#[tonic::async_trait]
impl SimpleBluetoothInterface for BluetoothServer {
    async fn simple_connect(
        &self,
        request: tonic::Request<SimpleConnectRequest>,
    ) -> std::result::Result<tonic::Response<SimpleReply>, tonic::Status> {
        let reply = SimpleReply { result: true };
        Ok(Response::new(reply))
    }

    async fn simple_disconnect(
        &self,
        request: tonic::Request<SimpleDisconnectRequest>,
    ) -> std::result::Result<tonic::Response<SimpleReply>, tonic::Status> {
        let reply = SimpleReply { result: true };
        Ok(Response::new(reply))
    }

    async fn simple_remove(
        &self,
        request: tonic::Request<SimpleRemoveRequest>,
    ) -> std::result::Result<tonic::Response<SimpleReply>, tonic::Status> {
        let reply = SimpleReply { result: true };
        Ok(Response::new(reply))
    }

    async fn simple_power_reset(
        &self,
        request: tonic::Request<SimplePowerResetRequest>,
    ) -> std::result::Result<tonic::Response<SimpleReply>, tonic::Status> {
        for i in 0..10000 {
            if i % 100 == 0 {
                println!("{}", i);
            }
        }
        Err(Status::permission_denied("ppp"))

        // let reply = SimpleReply {
        //     result: true,
        // };
        // Ok(Response::new(reply))
    }

    async fn simple_play_media(
        &self,
        request: tonic::Request<SimplePlayMediaRequest>,
    ) -> std::result::Result<tonic::Response<SimpleReply>, tonic::Status> {
        let reply = SimpleReply { result: true };
        Ok(Response::new(reply))
    }

    async fn simple_command(
        &self,
        request: tonic::Request<SimpleCommandRequest>,
    ) -> std::result::Result<tonic::Response<SimpleReply>, tonic::Status> {
        let reply = SimpleReply { result: true };
        Ok(Response::new(reply))
    }
}
