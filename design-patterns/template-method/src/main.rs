/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b80de6d4e6d75bf59bb37db386264fed
trait DoStep2 {
    fn do_step2_maybe_different(&mut self);
}

/// “父类”
struct B<T> {
    id: u32,
    t: T, // 或者Box<&dyn DoStep2>
}

impl<T> B<T> {
    fn do_step1_common(&mut self) {
        println!("step1: use common field id({}) do sth common for all B", self.id);
    }

    fn do_step3_common(&mut self) {
        println!("step3: use common field id({}) do sth common for all B\n", self.id);
    }
}

impl<T> B<T>
where
    Self: DoStep2
{
    pub fn do_all_steps(&mut self) {
        self.do_step1_common();
        self.do_step2_maybe_different();
        self.do_step3_common();
    }
}

struct B1 {
    token: String,
}

/// 为子类一实现，能访问到父类成员
impl DoStep2 for B<B1> {
    fn do_step2_maybe_different(&mut self) {
        println!("step2: use B1's token({}) and B's id({}) do sth for B<B1>", self.t.token, self.id);
    }
}

struct B2 {
    role: String,
}

/// 为子类二实现，也能访问到父类成员
impl DoStep2 for B<B2> {
    fn do_step2_maybe_different(&mut self) {
        println!("step2: use B2's role({}) and B's id({}) do sth for B<B2>", self.t.role, self.id);
    }
}

fn main() {
    let sub_b1 = B1 { token: "permit".to_string() };
    let mut b = B { id: 56, t: sub_b1 };
    b.do_all_steps();

    let sub_b2 = B2 { role: "manager".to_string() };
    let mut b = B { id: 56, t: sub_b2 };
    b.do_all_steps();

    // 下面这种就不行，没为这种实现do_all_steps，不能调用
    // let mut b: B<&'static str> = B { id: 56, t: "a sample static str" };
    // b.do_all_steps(); 
}