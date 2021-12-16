pub trait Aggregate {
    fn aggregation(&self) -> String;
}

#[derive(Debug)]
pub struct Car {
    gas: i32,
    name: String
}

impl Aggregate for Car {
    fn aggregation(&self) -> String {
        return format!("name: {} gas: {}", self.name, self.gas);
    }
}

fn main() {
    let porsche = Car {gas: 2, name: String::from("porsche")};
    println!("{}", porsche.aggregation());
}
