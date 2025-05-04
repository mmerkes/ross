use log::{info, warn};
use std::collections::HashMap;
use std::{
    fs,
    io,
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct Store {
    id: String,
}

impl Store {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone)]
pub struct Storage {
    stores: Arc<RwLock<HashMap<String, Store>>>,
    root_dir: String,
}

impl Storage {
    pub fn load(root_dir: &String) -> io::Result<Self> {
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

        Ok(Self { stores: Arc::new(RwLock::new(stores)), root_dir: root_dir.to_string() })
    }

    pub fn add(&self, id: String) -> io::Result<()> {
        if self.stores.read().unwrap().contains_key(&id) {
            return Ok(());
        }

        fs::create_dir(format!("{}/{}", self.root_dir, id))?;

        self.stores.write().unwrap().insert(id.clone(), Store::new(id.clone()));

        Ok(())
    }
}