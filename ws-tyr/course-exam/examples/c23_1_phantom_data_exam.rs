use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

struct Customer<T> {
    id: u64,
    name: String,
    _tag: PhantomData<T>,
}

trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name);
    }

    fn feature2(&self) {
        println!("feature 2 for {}", self.name);
    }
}

impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!(
            "Dear {}(as our valuable customer {}), enjoy this advanced feature!",
            self.name, self.id
        );
    }
}

impl<T> Customer<T> {
    fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _tag: Default::default(),
        }
    }
}

struct FreePlan;

struct PersonalPlan(f32);


impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(value: Customer<FreePlan>) -> Self {
        Self::new(value.name)
    }
}

fn subscribe(free_customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);
    println!(
        "Dear {}(user id {}), spend {} to purchase membership",
        free_customer.name, free_customer.id, payment
    );

    free_customer.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer() {
        // 一开始是个免费用户
        let customer = Customer::<FreePlan>::new("Tyr".into());
        // 使用免费 feature
        customer.feature1();
        customer.feature2();
        // 用着用着觉得产品不错愿意付费
        let customer = subscribe(customer, 6.99);
        customer.feature1();
        customer.feature2();
        // 付费用户解锁了新技能
        customer.advance_feature();
    }
}

fn main() {}