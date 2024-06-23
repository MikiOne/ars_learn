use std::path::Path;
use std::sync::Arc;

use rocksdb::{DB, Options, WriteBatch};

use crate::store::{Batch, Store};
use crate::store::error::Error;

#[derive(Clone)]
pub(crate) struct RocksdbStore {
    db: Arc<DB>,
}

impl Store for RocksdbStore {
    type Batch = RocksdbBatch;
    type Opts = Options;

    fn new<P>(opts: &Self::Opts, path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let db = DB::open(opts, path)?;
        Ok(RocksdbStore { db: Arc::new(db) })
    }

    fn default_options() -> Self::Opts {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts
    }

    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, Error> {
        Ok(self.db.get(key)?)
    }

    fn exists<K: AsRef<[u8]>>(&self, key: K) -> Result<bool, Error> {
        Ok(self.get(key).map(|v| v.is_some())?)
    }

    fn batch(&self) -> Self::Batch {
        Self::Batch {
            db: Arc::clone(&self.db),
            wb: WriteBatch::default(),
        }
    }
}

pub(crate) struct RocksdbBatch {
    db: Arc<DB>,
    wb: WriteBatch,
}

impl Batch for RocksdbBatch {
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&mut self, key: K, value: V) {
        self.wb.put(key, value);
    }

    fn del<K: AsRef<[u8]>>(&mut self, key: K) {
        self.wb.delete(key);
    }

    fn commit(self) -> Result<(), Error> {
        Ok(self.db.write(self.wb)?)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;

    #[test]
    fn put_and_get() {
        let store = RocksdbStore::new(
            &RocksdbStore::default_options(),
            Builder::new().prefix("put_and_get").tempdir().unwrap(),
        ).unwrap();

        let mut batch = store.batch();
        batch.put([0, 0], [0, 0, 0]);
        batch.put([1, 1], [1, 1, 1]);
        batch.commit().unwrap();

        assert_eq!(Some(vec![0, 0, 0]), store.get([0, 0]).unwrap());
        assert_eq!(Some(vec![1, 1, 1]), store.get([1, 1]).unwrap());
        assert_eq!(None, store.get([2, 2]).unwrap());
    }

    #[test]
    fn exists() {
        let store = RocksdbStore::new(
            &RocksdbStore::default_options(),
            Builder::new().prefix("exists").tempdir().unwrap(),
        ).unwrap();
        assert!(!store.exists([0, 0]).unwrap());

        let mut batch = store.batch();
        batch.put([0, 0], [0, 0, 0]);
        batch.commit().unwrap();

        assert!(store.exists([0, 0]).unwrap());
    }

    #[test]
    fn delete() {
        let store = RocksdbStore::new(
            &RocksdbStore::default_options(),
            Builder::new().prefix("delete").tempdir().unwrap(),
        ).unwrap();

        let mut batch = store.batch();
        batch.put([0, 0], [0, 0, 0]);
        batch.commit().unwrap();
        assert_eq!(Some(vec![0, 0, 0]), store.get([0, 0]).unwrap());

        let mut batch = store.batch();
        batch.del([0, 0]);
        batch.commit().unwrap();
        assert_eq!(None, store.get([0, 0]).unwrap());
    }
}