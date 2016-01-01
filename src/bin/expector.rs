extern crate expector;
use expector::*;

fn main() {
    expect(1).to(eq(1));

    let mut bees: Vec<f64> = Vec::new();
    bees.push(37.0);
    expect(bees).to(be_an_empty_vector());
}
