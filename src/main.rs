
struct Market {
    day_time: i64,
    stock: i64, 
    price: f64,
    demand_elasticity: f64, 
    base_demand: f64,
    closed: bool,
}

impl Market { 
    fn new(day_time: Option<i64>, stock: i64, price: f64, demand_elasticity: Option<f64>, base_demand: Option<f64>) -> Market {
        let day_time: i64 = match day_time {
            Some(day_time) => day_time, 
            None => 7,
        };

        let demand_elasticity: f64 = match demand_elasticity {
            Some(demand_elasticity) => demand_elasticity, 
            None => -1.0,
        };

        let base_demand: f64 = match base_demand {
            Some(base_demand) => base_demand,
            None => 1.0,
        };

        if day_time >= 20 || day_time <= 8 {
            let closed: bool = true; 
        } else {
            let closed: bool = false;
        }

        Market { day_time: day_time, stock: stock, price: price, demand_elasticity: demand_elasticity, base_demand: base_demand, closed: closed }
    }

    fn step(price_change: f64) {

    }
}

fn main() {
    println!("Hello, world!");
}
