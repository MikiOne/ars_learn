use std::path::Path;

use ::rocksdb::{DB, DBIteratorWithThreadMode, Direction, IteratorMode};

use crate::store::error::Error;

pub mod rocksdb;
pub mod error;

type IteratorItem = Result<(Box<[u8]>, Box<[u8]>), ::rocksdb::Error>;

// type StoreIterItem = (Box<[u8]>, Box<[u8]>);

pub(crate) struct Kvpair {
    key: Box<[u8]>,
    value: Box<[u8]>,
}

pub(crate) struct StoreIter<T>(T);

impl<T> StoreIter<T> {
    fn new(data: T) -> Self {
        StoreIter(data)
    }
}

impl<T> Iterator for StoreIter<T>
where
    T: Iterator,
    T::Item: Into<Kvpair>,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.next().map(|item| item.into())
    }
}

impl From<IteratorItem> for Kvpair {
    fn from(value: IteratorItem) -> Self {
        match value {
            Ok((key, value)) => Kvpair {
                key: key.into(),
                value: value.into(),
            },
            Err(err) => panic!("{:?}", err),
        }
    }
}

pub(crate) trait Store {
    type Batch: Batch;
    type Opts;

    fn new<P>(opts: &Self::Opts, path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        Self: Sized;

    fn default_options() -> Self::Opts;

    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), Error>;

    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, Error>;

    fn exists<K: AsRef<[u8]>>(&self, key: K) -> Result<bool, Error>;

    fn batch(&self) -> Self::Batch; // 封装 iterator 的方法

    fn iterate(&self, mode: IteratorMode) -> DBIteratorWithThreadMode<DB>;

    fn iter_from<K: AsRef<[u8]>>(
        &self, from_key: K, direction: Direction,
    ) -> Box<dyn Iterator<Item=IteratorItem> + '_>;

    fn iter_to<K: AsRef<[u8]>>(
        &self, to_key: K, direction: Direction,
    ) -> Box<dyn Iterator<Item=Kvpair>>;
}

pub(crate) trait Batch {
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&mut self, key: K, value: V);

    fn del<K: AsRef<[u8]>>(&mut self, key: K);

    fn commit(self) -> Result<(), Error>;
}
