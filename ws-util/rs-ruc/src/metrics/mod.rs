use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

mod tests;

pub struct Metrics(HashMap<&'static str, AtomicUsize>);

impl Metrics {
    fn new(names: Vec<&'static str>) -> Self {
        let mut metrics: HashMap<&'static str, AtomicUsize> = HashMap::with_capacity(names.len());
        for name in names {
            metrics.insert(name, AtomicUsize::new(0));
        }
        Self(metrics)
    }

    fn inc(&self, name: &'static str) {
        if let Some(m) = self.0.get(name) {
            m.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn add(&self, name: &'static str, val: usize) {
        if let Some(m) = self.0.get(name) {
            m.fetch_add(val, Ordering::Relaxed);
        }
    }

    fn dec(&self, name: &'static str) {
        if let Some(m) = self.0.get(name) {
            m.fetch_sub(1, Ordering::Relaxed);
        }
    }

    fn snapshot(&self) -> Vec<(&'static str, usize)> {
        self.0.iter().map(|(key, val)| (*key, val.load(Ordering::Relaxed))).collect()
    }
}
