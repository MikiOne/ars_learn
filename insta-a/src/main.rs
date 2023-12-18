use similar_asserts::assert_eq;

fn main() {
    let reference = vec![1, 2, 3, 4];
    assert_eq!(reference, (0..4).collect::<Vec<_>>());
}