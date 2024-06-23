use std::path::Path;

use ::rocksdb::{DB, DBIteratorWithThreadMode, Direction, IteratorMode};

use crate::store::error::Error;

pub mod rocksdb;
pub mod error;

type IteratorItem = Result<(Box<[u8]>, Box<[u8]>), ::rocksdb::Error>;

pub(crate) trait Store {
    type Batch: Batch;
    type Opts;

    fn new<P>(opts: &Self::Opts, path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        Self: Sized;

    fn default_options() -> Self::Opts;

    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, Error>;

    fn exists<K: AsRef<[u8]>>(&self, key: K) -> Result<bool, Error>;

    fn batch(&self) -> Self::Batch; // 封装 iterator 的方法

    fn iterate(&self, mode: IteratorMode) -> DBIteratorWithThreadMode<DB>;

    fn iter_from<K: AsRef<[u8]>>(
        &self, from_key: K, direction: Direction,
    ) -> Box<dyn Iterator<Item=IteratorItem> + '_>;
}

pub(crate) trait Batch {
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&mut self, key: K, value: V);

    fn del<K: AsRef<[u8]>>(&mut self, key: K);

    fn commit(self) -> Result<(), Error>;
}
