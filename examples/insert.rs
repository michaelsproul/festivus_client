extern crate festivus_client as festivus;

use festivus::Festivus;

fn main() {
    let client = Festivus::new("http://localhost:3000");
    println!("{:?}", client.insert(50, 100));
}
