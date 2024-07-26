trait Op {
    fn op(&self, val1: usize, val2: usize);
}

struct OpCtx {
    op: Box<dyn Op>,
}

impl OpCtx {
    fn new(op: Box<dyn Op>) -> Self {
        OpCtx { op }
    }

    fn exec(&self, val1: usize, val2: usize) {
        self.op.op(val1, val2);
    }
}

struct AddOp;

impl Op for AddOp {
    fn op(&self, val1: usize, val2: usize) {
        println!("Adding {} and {}: {}", val1, val2, val1 + val2);
    }
}
struct SubOp;

impl Op for SubOp {
    fn op(&self, val1: usize, val2: usize) {
        println!("Subtracting {} and {}: {}", val1, val2, val1 - val2);
    }
}

struct MulOp;

impl Op for MulOp {
    fn op(&self, val1: usize, val2: usize) {
        println!("Multiplying {} and {}: {}", val1, val2, val1 * val2);
    }
}
fn main() {
    let add_op = Box::new(AddOp);
    let sub_op = Box::new(SubOp);
    let mul_op = Box::new(MulOp);

    let mut ctx = OpCtx::new(add_op);
    ctx.exec(3, 5);

    ctx.op = sub_op;
    ctx.exec(7, 2);

    ctx.op = mul_op;
    ctx.exec(4, 2);
}