use std::{env, fs, io, path::{Path, PathBuf}, process::Command};

fn visit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                paths.extend(visit_dirs(&path)?);
            } else {
                if entry.path().extension().map_or(false, |ext| ext == "td") {
                    paths.push(entry.path());
                }
            }
        }
    }

    Ok(paths)
}

fn main() {
    // generate rs code for .td files

    let src_dir = Path::new("src");

    let td_files = visit_dirs(src_dir).expect("err");

    for file in td_files.clone() {
        println!("cargo:rerun-if-changed-file={}", file.display());

        let def_file = file.with_extension("def");
        
        let out = Command::new("ytbgen")
            .arg(file.to_owned())
            .output()
            .expect("Failed to execute ytbgen");

        fs::write(&def_file, out.stdout)
            .expect("Failed to write .def file");

    }

    if env::var("CARGO_FEATURE_CLEAN").is_ok() {
        for file in td_files {
            let def_file = file.with_extension("def");
            if def_file.exists() {
                fs::remove_file(def_file).expect("Failed to delete .def file");
            }
        }
    }

    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    println!("cargo:rerun-if-changed-env=TARGET")
}