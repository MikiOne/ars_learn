fn main() {}

fn number_calc(val: i32) -> i32 {
    let ret_val = 40;
    if val < 0 {
        panic!("值必须大于0,传参的值为:{}", val)
    }
    return ret_val;
}

#[test]
#[should_panic(expected = "传参不能小于0")]
fn test_num_calc() {
    let res = number_calc(-1);
}
