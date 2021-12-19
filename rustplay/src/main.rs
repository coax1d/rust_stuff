use std::fmt::Display;

pub trait Aggregate {
    fn aggregation(&self) -> String;
}

#[derive(Debug)]
pub struct Car<T, U> {
    gas: T,
    name: U
}

impl<T: Display, U: Display> Aggregate for Car<T, U> {
    fn aggregation(&self) -> String {
        return format!("name: {} gas: {}", self.name, self.gas);
    }
}

fn main() {
    let porsche = Car {gas: 2, name: String::from("porsche")};
    println!("{}", porsche.aggregation());

    let nums = vec![1,2,3,4,5];

    for (_, &value) in nums.iter().enumerate() {
        if value == 3 {
            println!("Found {}", value);
        }
    }

}
