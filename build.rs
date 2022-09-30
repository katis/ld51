use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=raw/sheets/*.aseprite");

    for entry in fs::read_dir("raw/spritesheets").unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            let path = entry.path();

            let json_path = {
                let sheet_file = path.with_extension("sheet.json");
                let mut p = PathBuf::from("assets/sheets");
                p.push(sheet_file.file_name().unwrap());
                p
            };
            let png_path = {
                let sheet_file = path.with_extension("png");
                let mut p = PathBuf::from("assets/sheet_textures");
                p.push(sheet_file.file_name().unwrap());
                p
            };

            Command::new("aseprite")
                .args([
                    "-b",
                    "--format",
                    "json-array",
                    "--sheet-type",
                    "packed",
                    "--list-tags",
                    "--sheet",
                    png_path.to_str().unwrap(),
                    "--data",
                    json_path.to_str().unwrap(),
                    path.to_str().unwrap(),
                ])
                .output()
                .expect("failed to convert texture");
        }
    }
}
