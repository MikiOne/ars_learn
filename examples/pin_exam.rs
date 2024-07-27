struct SelfRef {
    value: String,
    pointer_to_value: *const String,
}

impl SelfRef {
    fn new(value: String) -> Self {
        Self {
            value,
            pointer_to_value: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let pointer: *const String = &self.value;
        self.pointer_to_value = pointer;
    }

    fn a(&self) -> &str {
        &*self.value
    }

    fn b(&self) -> &String {
        assert!(!self.pointer_to_value.is_null(), "init being called first");
        unsafe { &*(self.pointer_to_value) }
    }
}

fn main() {
    let test = SelfRef::new("iki".to_string());
    let mut pinned_test = Box::pin(test);
    pinned_test.init();

    println!("a: {}", pinned_test.a());
    println!("b: {}", pinned_test.b());
}