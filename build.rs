use std::{io::Result, path::PathBuf};
use walkdir::WalkDir;

fn main() -> Result<()> {
    let proto_root = PathBuf::from("proto");
    let mut proto_files = Vec::new();

    for entry in WalkDir::new(&proto_root) {
        let entry = entry?;

        if entry.file_type().is_file() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "proto") {
                proto_files.push(path.to_owned());
            }
        }
    }

    if proto_files.is_empty() {
        eprintln!("Warnning Not found {:?}", proto_root.display());
        return Ok(());
    }

    tonic_prost_build::configure()
        // .extern_path(".cn.pecs.api.common.query.QueryDto", "QueryDto")
        .compile_protos(
            &proto_files,
            &[proto_root],
        )?;

    Ok(())
}