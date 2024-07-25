struct A {}

impl A {
    fn do_step1_common(&mut self) {}
    fn do_step2_maybe_different(&mut self) {}

    fn do_step3_common(&mut self) {}

    pub fn do_all_steps(&mut self) {
        self.do_step1_common();
        self.do_step2_maybe_different();
        self.do_step3_common();
    }
}

struct A1 {
    a: A,
}

impl A1 {
    fn do_step2_maybe_different(&mut self) {}
}


fn main() {}