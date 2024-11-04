fn main() {
    let mut data = vec![1, 2, 3];

    for item in data.iter_mut() {
        // cannot borrow `data` as mutable more than once at a time
        // data.push(*item + 1);
        *item += 1;
    }
    println!("{:?}", data);
}