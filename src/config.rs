use base64::encode;
use rand::rngs::OsRng;
use rand::{Rng, RngCore};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_str, to_string};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct MySqlCredentials {
    pub database: String,
    pub server: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub bind_address: String,
    pub mysql: MySqlCredentials,
    pub session_key_base64: String,
}

impl Default for MySqlCredentials {
    fn default() -> Self {
        Self {
            database: "smit".into(),
            server: "localhost".into(),
            username: "smit".into(),
            password: "".into(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut session_key = vec![0; 64];
        OsRng.fill_bytes(&mut session_key);

        Config {
            bind_address: "127.0.0.1:8192".into(),
            mysql: MySqlCredentials::default(),
            session_key_base64: encode(&session_key),
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
