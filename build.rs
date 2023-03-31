fn main() {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(&[
            "proto/auth.proto",
            "profile.proto",
            "registries.proto",
            "transactions.proto",
            "users.proto",
        ], &[
            "proto"
        ])
        .unwrap();

    //tonic_build::compile_protos("./proto/*.proto")
    //    .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
