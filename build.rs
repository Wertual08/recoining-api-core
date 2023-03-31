fn main() {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(&[
            "proto/auth.proto",
            "proto/profile.proto",
            "proto/registries.proto",
            "proto/transactions.proto",
            "proto/users.proto",
        ], &[
            "proto"
        ])
        .unwrap();

    //tonic_build::compile_protos("./proto/*.proto")
    //    .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
