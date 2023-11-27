use rand::thread_rng;
use rand_distr::{Distribution, Poisson};
use ndarray::prelude::Array;

use std::{error::Error, thread::current};

#[derive(Debug)]
struct Market {
    day_time: i64,
    stock: f64,
    price: f64,
    demand_elasticity: f64,
    base_demand: f64,
    closed: bool,
    turnover: f64,
}

impl Market {
    fn new(
        day_time: i64,
        stock: f64,
        price: f64,
        demand_elasticity: Option<f64>,
        base_demand: Option<f64>,
    ) -> Market {
        let demand_elasticity: f64 = match demand_elasticity {
            Some(demand_elasticity) => demand_elasticity,
            None => -1.0,
        };

        let base_demand: f64 = match base_demand {
            Some(base_demand) => base_demand,
            None => 1.0,
        };

        let closed: bool = Market::check_if_closed(day_time, None, None);

        Market {
            day_time: day_time,
            stock: stock,
            price: price,
            demand_elasticity: demand_elasticity,
            base_demand: base_demand,
            closed: closed,
            turnover: 0.0,
        }
    }

    fn check_if_closed(
        current_time: i64,
        opening_time: Option<i64>,
        closing_time: Option<i64>,
    ) -> bool {
        let opening_time = match opening_time {
            Some(opening_time) => opening_time,
            None => 8,
        };

        let closing_time = match closing_time {
            Some(closing_time) => closing_time,
            None => 20,
        };

        let mut closed: bool = false;
        if current_time >= closing_time || current_time < opening_time {
            closed = true;
        }

        return closed;
    }

    fn step(&mut self, price_change: f64) -> () {
        self.base_demand += self.demand_elasticity * (price_change / self.price) * self.base_demand;
        self.price += price_change;

        let mut rng = thread_rng();
        let poisson = Poisson::new(self.base_demand).expect("Expect a valid lambda paramter");
        let sales = poisson.sample(&mut rng);
        let sales = if self.stock >= sales { sales } else { self.stock };

        self.day_time += 1;

        if !self.closed {
            self.stock = self.stock - sales;
            self.turnover += sales * self.price;
        }
        self.closed = Market::check_if_closed(self.day_time, None, None);
    }
}

fn main() {
    let mut market = Market::new(10, 10.0, 6.0, Some(-1.0), Some(0.8));

    println!("{:#?}", market);

    let n_time_states: usize = (20 - 8 + 1);
    let n_stock_states: usize = 10 + 1;
    let n_price_states: usize = 2 * 6;
    let n_actions: usize = 2;

    let q_table = Array::<f64, _>::zeros((n_time_states, n_stock_states, n_price_states, n_actions));

    println!("{:#?}", q_table);

    let observation_period: i64 = 1;

    for _ in 0..observation_period {
        market.step(0.0);
        println!("{:#?}", market);
    }
}
