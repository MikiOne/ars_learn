use crate::metrics::Metrics;
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref METRICS: Metrics = Metrics::new(vec![
        "topics",
        "clients",
        "peers",
        "broadcasts",
        "servers",
        "states",
        "subscribers"
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_metrics() {
        METRICS.inc("topics");
        METRICS.inc("subscribers");
        METRICS.add("clients", 33);

        println!("{:?}", METRICS.snapshot());
    }
}