fn main() {
    let idl_files = &[
        "proto/apache/rocketmq/v1/service.proto",
    ];
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(idl_files, &["proto"])
        .unwrap_or_else(|e| panic!("protoc failed: {}", e));
}
