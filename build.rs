fn main() {
    let idl_files = &["proto/apache/rocketmq/v2/service.proto"];
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(idl_files, &["proto"])
        .unwrap_or_else(|e| panic!("protoc failed: {}", e));
}
