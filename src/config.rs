use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_str, to_string};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub bind_address: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind_address: "127.0.0.1:8192".into(),
        }
    }
}

pub fn yaml_from_file_or_create<T: DeserializeOwned + Serialize + Default>(
    path: &Path,
) -> anyhow::Result<T> {
    if path.exists() {
        let yaml = read_to_string(path)?;
        let parsed = from_str::<T>(&yaml)?;
        Ok(parsed)
    } else {
        let config = T::default();
        let mut string = to_string(&config)?;
        string = string.replace("---\n", "");
        let mut file = File::create(path)?;
        file.write_all(string.as_bytes())?;
        Ok(config)
    }
}
