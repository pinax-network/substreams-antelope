use std::fs;
use substreams_antelope_abigen::Abigen;

fn main() {
    println!("Generating golden bindings");
    let abi_jsons_dir = "abi/";
    for entry in fs::read_dir(abi_jsons_dir)
        .expect("failed to read ABI JSON directory")
        .flatten()
        .filter(|entry| entry.path().extension() == Some("json".as_ref()))
    {
        let path = entry.path();
        println!("Generating bindings for {}", path.to_string_lossy());
        let out_path = format!("{abi_jsons_dir}{}.rs.golden", path.file_stem().unwrap().to_string_lossy());
        Abigen::new("Contract", &path.to_string_lossy())
            .unwrap_or_else(|_| panic!("failed to read ABI JSON for {}", path.to_string_lossy()))
            .generate()
            .unwrap_or_else(|_| panic!("failed to generate contract for {}", path.to_string_lossy()))
            .write_to_file(&out_path)
            .unwrap_or_else(|_| panic!("failed to write golden file {}", out_path));
    }
}
