use log::{info, warn};
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct Store {
    id: String,
}

impl Store {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

pub fn load(root_dir: &String) -> io::Result<HashMap<String, Store>> {
    let mut stores = HashMap::new();
    info!("Loading stores from local storage");

    for entry in fs::read_dir(root_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name().to_str().unwrap().to_string();

        if path.is_dir() {
            let _ = &stores.insert(file_name.clone(), Store::new(file_name.clone()));
        } else {
            warn!("Ignoring file {0}", file_name);
        }
    }

    Ok(stores)
}