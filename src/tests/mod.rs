use super::{Config, RSH};
use std::{
    fs::{self, DirEntry},
    path::Path,
};

#[test]
fn simple() {
    for file in fs::read_dir(Path::new("./src/tests")).unwrap() {
        let path = file.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "sh" {
                RSH::new(Config {
                    script_path: Some(path.to_str().unwrap().to_string()),
                })
                .run()
                .unwrap();
            }
        }
    }
}
