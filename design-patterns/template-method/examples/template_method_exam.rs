struct Base<T> {
    child: T,
}

impl<T> Base<T> {
    fn do_step1(&mut self) {
        println!("BaseFn step1");
    }
    fn do_step3(&mut self) {
        println!("BaseFn step3");
    }
}

trait AbstractStep2 {
    fn do_step2(&mut self);
}

impl<T> Base<T>
where
    Self: AbstractStep2,
{
    fn do_all_step(&mut self) {
        self.do_step1();
        self.do_step2();
        self.do_step3();
    }
}

struct Child1 {
    chile: String,
}

impl AbstractStep2 for Base<Child1> {
    fn do_step2(&mut self) {
        println!("Child_1 step2, chile: {}", self.child.chile);
    }
}

struct Child2 {
    china: String,
}

impl AbstractStep2 for Base<Child2> {
    fn do_step2(&mut self) {
        println!("Child_2 step2, china: {}", self.child.china);
    }
}

fn main() {
    let child = Child1 { chile: "智利".to_string() };
    let mut base = Base { child };
    base.do_all_step();

    println!("==============");

    let child = Child2 { china: "中国".to_string() };
    let mut base = Base { child };
    base.do_all_step();
}