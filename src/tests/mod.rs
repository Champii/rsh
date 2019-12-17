#[cfg(test)]
mod tests_mod {
    use super::super::{Config, RSH};
    use std::{fs, path::Path};
    #[test]
    fn simple() {
        for file in fs::read_dir(Path::new("./src/tests")).unwrap() {
            let path = file.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "sh" {
                    RSH::new(Config {
                        input: None,
                        script_path: Some(path.to_str().unwrap().to_string()),
                    })
                    .run()
                    .unwrap();
                }
            }
        }
    }
}
