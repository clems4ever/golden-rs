use std::{
    fmt::Debug,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use serde::{de::DeserializeOwned, Serialize};

pub fn assert_equal_to_golden<
    T: Serialize + DeserializeOwned + Debug + PartialEq,
    P: AsRef<Path>,
>(
    file_path: P,
    data: &T,
) {
    let file_path = file_path.as_ref();
    let ci_mode = std::env::var("CI").is_ok();

    if !ci_mode {
        let file_exists = std::fs::metadata(file_path).is_ok();
        if !file_exists {
            if let Some(parent_dir) = file_path.parent() {
                std::fs::create_dir_all(parent_dir).expect("failed to create directory");
            }
            write_to_json_file(data, file_path).expect("failed to write data");
            return;
        }
    }

    let expected_data: T = read_from_json_file(file_path).expect("failed to read the data");
    assert_eq!(data, &expected_data);
}

fn write_to_json_file<T: Serialize>(data: &T, file_path: &Path) -> std::io::Result<()> {
    let json_data = serde_json::to_string_pretty(data).expect("failed to serialize data to JSON");

    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())
}

fn read_from_json_file<T: DeserializeOwned>(file_path: &Path) -> std::io::Result<T> {
    let mut file = File::open(file_path)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let deserialized_data: T = serde_json::from_str(&json_data)?;
    Ok(deserialized_data)
}

pub fn assert_equal_to_string_golden<P: AsRef<Path>>(file_path: P, data: &String) {
    let file_path = file_path.as_ref();
    let ci_mode = std::env::var("CI").is_ok();

    if !ci_mode {
        let file_exists = std::fs::metadata(file_path).is_ok();
        if !file_exists {
            if let Some(parent_dir) = file_path.parent() {
                std::fs::create_dir_all(parent_dir).expect("failed to create directory");
            }
            let mut file = File::create(file_path).expect("failed to create file");
            file.write_all(data.as_bytes())
                .expect("failed to write data");
            return;
        }
    }

    let mut file = File::open(file_path).expect("failed to open file");
    let mut expected_data = String::new();
    file.read_to_string(&mut expected_data)
        .expect("failed to read to string");

    assert_eq!(data, &expected_data);
}
