fn main() {
    tonic_build::configure()
        .out_dir("./src/rpc")
        .compile(&["src/rpc/grpc_bluez.proto"], &["src/rpc"])
        .unwrap();
}
