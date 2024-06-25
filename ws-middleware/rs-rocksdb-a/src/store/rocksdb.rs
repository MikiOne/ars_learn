use std::path::Path;
use std::sync::Arc;

use rocksdb::{DB, DBIteratorWithThreadMode, Direction, IteratorMode, Options, WriteBatch};

use crate::store::{Batch, IteratorItem, Store};
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

    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), Error> {
        Ok(self.db.put(key, value)?)
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

    // 封装 iterator 的方法
    fn iterate(&self, mode: IteratorMode) -> DBIteratorWithThreadMode<DB> {
        self.db.iterator(mode)
    }

    fn iter_from<K: AsRef<[u8]>>(
        &self, from_key: K, direction: Direction,
    ) -> Box<dyn Iterator<Item=IteratorItem> + '_> {
        let mode = IteratorMode::From(from_key.as_ref(), direction);
        Box::new(self.db.iterator(mode)) as Box<_>
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
    use tempfile::Builder;

    use super::*;

    #[test]
    fn put() {
        let store = RocksdbStore::new(
            &RocksdbStore::default_options(),
            Builder::new().prefix("put").tempdir().unwrap(),
        ).unwrap();

        store.put([0, 0], [0, 0, 0]).unwrap();
        assert_eq!(Some(vec![0, 0, 0]), store.get([0, 0]).unwrap());
    }

    #[test]
    fn put_str() {
        let store = RocksdbStore::new(
            &RocksdbStore::default_options(),
            Builder::new().prefix("put_str").tempdir().unwrap(),
        ).unwrap();

        store.put("hello".as_bytes(), "world".as_bytes()).unwrap();
        assert_eq!(Some("world".as_bytes().to_vec()), store.get("hello".as_bytes()).unwrap());
    }

    #[test]
    fn iter_from() {
        let store = RocksdbStore::new(
            &RocksdbStore::default_options(),
            Builder::new().prefix("iter").tempdir().unwrap(),
        ).unwrap();

        let mut batch = store.batch();
        batch.put([0, 0, 0], [0, 0, 0]);
        batch.put([0, 0, 1], [0, 0, 1]);
        batch.put([1, 0, 0], [1, 0, 0]);
        batch.put([1, 0, 1], [1, 0, 1]);
        batch.put([2, 0, 0, 1], [2, 0, 0, 1]);
        batch.put([2, 0, 1, 1], [2, 0, 1, 1]);
        batch.put([2, 0, 2, 1], [2, 0, 2, 1]);
        batch.commit().unwrap();

        let mut iter = store.iter_from([0, 0, 1], Direction::Forward);
        assert_eq!(
            Some((vec![0, 0, 1], vec![0, 0, 1])),
            iter.next().map(|i| {
                let i = i.unwrap();
                (i.0.to_vec(), i.1.to_vec())
            })
        );
        assert_eq!(
            Some((vec![1, 0, 0], vec![1, 0, 0])),
            iter.next().map(|i| {
                let i = i.unwrap();
                (i.0.to_vec(), i.1.to_vec())
            })
        );
    }

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

    #[test]
    fn iterate() {
        let store = RocksdbStore::new(
            &RocksdbStore::default_options(),
            Builder::new().prefix("iter").tempdir().unwrap(),
        ).unwrap();

        let mut batch = store.batch();
        batch.put([0, 0, 0], [0, 0, 0]);
        batch.put([0, 0, 1], [0, 0, 1]);
        batch.put([1, 0, 0], [1, 0, 0]);
        batch.put([1, 0, 1], [1, 0, 1]);
        batch.put([2, 0, 0, 1], [2, 0, 0, 1]);
        batch.put([2, 0, 1, 1], [2, 0, 1, 1]);
        batch.put([2, 0, 2, 1], [2, 0, 2, 1]);
        batch.commit().unwrap();

        let iter = store.iterate(IteratorMode::Start);
        for item in iter {
            let (key, value) = item.unwrap();
            println!("获得键值对: {:?} -> {:?}", key, value);
        }

        let iter = store.iterate(IteratorMode::End);
        for item in iter {
            let (key, value) = item.unwrap();
            println!("获得键值对: {:?} -> {:?}", key, value);
        }

        // let mut iter = store.iter([0, 0, 1], IteratorDirection::Forward).unwrap();
        // assert_eq!(
        //     Some((vec![0, 0, 1], vec![0, 0, 1])),
        //     iter.next().map(|i| (i.0.to_vec(), i.1.to_vec()))
        // );
        // assert_eq!(
        //     Some((vec![1, 0, 0], vec![1, 0, 0])),
        //     iter.next().map(|i| (i.0.to_vec(), i.1.to_vec()))
        // );
        //
        // let mut iter = store.iter([0, 0, 1], IteratorDirection::Reverse).unwrap();
        // assert_eq!(
        //     Some((vec![0, 0, 1], vec![0, 0, 1])),
        //     iter.next().map(|i| (i.0.to_vec(), i.1.to_vec()))
        // );
        // assert_eq!(
        //     Some((vec![0, 0, 0], vec![0, 0, 0])),
        //     iter.next().map(|i| (i.0.to_vec(), i.1.to_vec()))
        // );
        // assert!(iter.next().is_none());
        //
        // let mut iter = store.iter([2, 0, 1], IteratorDirection::Reverse).unwrap();
        // assert_eq!(
        //     Some((vec![2, 0, 0, 1], vec![2, 0, 0, 1])),
        //     iter.next().map(|i| (i.0.to_vec(), i.1.to_vec()))
        // );
    }
}