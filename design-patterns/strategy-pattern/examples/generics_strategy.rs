trait TravelStrategy {
    fn travel(&self, distance: i16);
}

// context 泛型
struct TravelContext<T: TravelStrategy> {
    strategy: T,
}

impl<T: TravelStrategy> TravelContext<T> {
    fn new(strategy: T) -> Self {
        TravelContext { strategy }
    }

    fn travel(&self, distance: i16) {
        self.strategy.travel(distance);
    }
}

//火车策略
struct TrainTravelStrategy;

impl TravelStrategy for TrainTravelStrategy {
    fn travel(&self, distance: i16) {
        println!("Train travel {} km.", distance);
    }
}

//飞机策略
struct AirplaneTravelStrategy;

impl TravelStrategy for AirplaneTravelStrategy {
    fn travel(&self, distance: i16) {
        println!("Airplane travel {} km.", distance);
    }
}

// 汽车策略
struct CarTravelStrategy;

impl TravelStrategy for CarTravelStrategy {
    fn travel(&self, distance: i16) {
        println!("Car travel {} km.", distance);
    }
}


fn main() {
    let train_strategy = TrainTravelStrategy;
    let airplane_strategy = AirplaneTravelStrategy;
    let car_strategy = CarTravelStrategy;

    let train_context = TravelContext::new(train_strategy);
    let airplane_context = TravelContext::new(airplane_strategy);
    let car_context = TravelContext::new(car_strategy);

    train_context.travel(100);
    airplane_context.travel(2000);
    car_context.travel(30);
}