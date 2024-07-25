trait DoStep2 {
    fn do_step2_maybe_different(&mut self);
}

struct B<T: DoStep2> {
    t: T,
}

impl<T: DoStep2> B<T> {
    fn do_step1_common(&mut self) {
        println!("A step1");
    }

    fn do_step3_common(&mut self) {
        println!("A step3");
    }

    pub fn do_all_steps(&mut self) {
        self.do_step1_common();
        self.t.do_step2_maybe_different();
        self.do_step3_common();
    }
}

struct B1 {}
impl DoStep2 for B1 {
    fn do_step2_maybe_different(&mut self) {
        println!("B1 step2");
    }
}

fn main() {
    let mut b = B { t: B1 {} };
    b.do_all_steps();
}