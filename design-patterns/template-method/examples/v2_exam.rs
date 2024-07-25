trait DoStep2 {
    fn do_step2_maybe_different(&mut self);
}

trait DoStep1 {
    fn do_step1_common(&mut self);
}

trait DoStep3 {
    fn do_step3_common(&mut self);
}
struct B<T: DoStep2> {
    t: T,
}

impl<T> DoStep1 for B<T> {
    fn do_step1_common(&mut self) {
        println!("B<T> step1");
    }
}
impl<T> DoStep3 for B<T> {
    fn do_step3_common(&mut self) {
        println!("B<T> step3");
    }
}

// impl<T> T
// where
//     T: DoStep1 + DoStep2 + DoStep3,
// {
//     pub fn do_all_steps(&mut self) {
//         self.do_step1_common();
//         self.do_step2_maybe_different();
//         self.do_step3_common();
//     }
// }

fn main() {}