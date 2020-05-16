fn main() {
    prost_build::compile_protos(&["src/repro.proto", "src/repro_submodule.proto"], &["src/"])
        .unwrap();
}
