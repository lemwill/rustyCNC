fn main() {
    protobuf_codegen::Codegen::new()
        .cargo_out_dir("src")
        .include("src")
        .input("src/example.proto")
        .run_from_script();
}
