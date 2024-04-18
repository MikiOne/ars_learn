fn main() {
    //     prost_build::Config::new()
    //         .default_package_filename("animals")
    //         .out_dir("src/pb")
    //         .compile_protos(&["animals.proto"], &["."])
    //         .unwrap();

    prost_build::Config::new()
        // .default_package_filename("custom_type")
        .out_dir("src/pb")
        .compile_protos(&["extensions.proto", "oneof.proto"], &["."])
        .unwrap();
}
