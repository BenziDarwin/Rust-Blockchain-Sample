use rocksdb::{DB, Options};
use std::path::Path;

pub struct Db {
    db: DB,
}

impl Db {
    pub fn new(db_path: &str) -> Result<Db, Box<dyn std::error::Error>> {
        let path = Path::new(db_path);
        let mut db_opts = Options::default();
        db_opts.create_if_missing(true);

        let db = DB::open(&db_opts, path)?;

        Ok(Db { db })
    }

    pub fn save(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.db.put(key.as_bytes(), value.as_bytes())?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        match self.db.get(key.as_bytes())? {
            Some(value) => Ok(Some(String::from_utf8(value)?)),
            None => Ok(None),
        }
    }
}
