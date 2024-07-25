trait DoStep2 {
    fn do_step2_maybe_different(&mut self);
}

struct B<T> {
    t: T,
}

impl<T> B<T> {
    fn do_step1_common(&mut self) {
        println!("B step1");
    }

    fn do_step3_common(&mut self) {
        println!("B step3");
    }
}

impl<T> B<T>
where
    Self: DoStep2,
{
    pub fn do_all_steps(&mut self) {
        self.do_step1_common();
        self.do_step2_maybe_different();
        self.do_step3_common();
    }
}

struct B1 {}
impl DoStep2 for B<B1> {
    fn do_step2_maybe_different(&mut self) {
        println!("B1 step2");
    }
}

struct B2 {}
impl DoStep2 for B<B2> {
    fn do_step2_maybe_different(&mut self) {
        println!("B2 step2");
    }
}

fn main() {
    let mut b = B { t: B1 {} };
    b.do_all_steps();

    println!("==============");

    let mut b = B { t: B2 {} };
    b.do_all_steps();
}