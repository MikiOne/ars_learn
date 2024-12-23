#[macro_export]
macro_rules! rs_vec {
    // vec new
    () => {
        std::vec::Vec::new()
    };
    // rs_vec!(1, 2, 3)
    ($($x:expr),*) => ({
        let mut v = std::vec::Vec::new();
        $(
            v.push($x);
        )*
        v
    });
    // rs_vec![1; 3]
    ($x:expr; $n:expr) => {
        std::vec::from_elem($x, $n)
    };
}

fn main() {
    let mut v = rs_vec!();
    v.push(3);
    println!("{:?}", v);

    let v = rs_vec![1, 2, 3];
    println!("{:?}", v);

    let v = rs_vec![1; 3];
    println!("{:?}", v);
}
