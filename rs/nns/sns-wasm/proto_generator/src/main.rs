use ic_sns_wasm_proto_generator::{generate_prost_files, ProtoPaths};
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR env variable is not defined"),
    );
    let out = &manifest_dir.join("../gen");
    let sns_wasm_proto = manifest_dir.join("../proto");
    let base_types_proto = manifest_dir.join("../../../types/base_types/proto");
    let sns_init_proto = manifest_dir.join("../../../sns/init/proto");

    match std::fs::remove_dir_all(out) {
        Ok(_) => (),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => panic!(
            "failed to clean up output directory {}: {}",
            out.display(),
            e
        ),
    }
    generate_prost_files(
        ProtoPaths {
            sns_wasm: &sns_wasm_proto,
            base_types: &base_types_proto,
            sns_init: &sns_init_proto,
        },
        out.as_ref(),
    );
}
