fn main() {
    let protoc_path = protoc_bin_vendored::protoc_bin_path()
        .expect("Failed to get vendored protoc path");

    let mut config = prost_build::Config::new();
    config.protoc_executable(protoc_path);

    config
        .compile_protos(&["src/proto/bid.proto"], &["src/proto"])
        .expect("Failed to compile protobuf schemas");
}