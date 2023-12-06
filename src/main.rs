fn main() {
    println!("Hello, world!");
}

fn number_calc(val: i32) -> i32 {
    let ret_val = 40;
    if val < 0 {
        return 30;
    }
    return ret_val;
}

#[test]
fn test_num_calc() -> Result<(), String> {
    if number_calc(-1) == 40 {
        Ok(())
    } else {
        Err(String::from("结果不等于40,请检查原因!"))
    }
}
