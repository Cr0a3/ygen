use std::fs::File;
use std::io::Read;
use std::process::exit;

pub fn read_in_file(path: &String) -> String {
    let mut infile = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error: {} {}", path, err);
            exit(-1);
        },
    };

    let mut input = String::new();
    match infile.read_to_string(&mut input) {
        Ok(_) => {},
        Err(err) => {
            println!("Error: {} {}", path, err);
            exit(-1);
        }
    };

    input
}
pub fn out_file(in_path: &String, out_path: Option<String>) -> File {
    let path;

    if let Some(out_path) = out_path { path = out_path }
    else {
        let file = in_path.split("/").collect::<Vec<&str>>().last().unwrap_or(&&in_path.as_str()).to_string();
        let slices = file.split(".").collect::<Vec<&str>>();
        
        let mut name = String::new();

        for slice in &slices {
            if slices.last() == Some(slice) {
                break;
            }

            name.push_str(slice);
        }

        path = format!("{}.o", name);
    }

    match File::options().create(true).write(true).truncate(true).open(&path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error: {} {}", path, err);
            exit(-1);
        },
    }
}