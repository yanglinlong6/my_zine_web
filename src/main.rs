use zino::prelude::*;
mod controller;
mod router;

fn main() {
    println!("Hello, world!");
    zino::Cluster::boot().register(router::routes()).run()
}
