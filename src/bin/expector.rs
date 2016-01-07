extern crate expector;
use expector::*;

fn main() {
    using_expect_syntax();
    using_should_syntax();
}

fn using_expect_syntax() {
    expect(1).to(eq(1));

    let mut bees: Vec<f64> = Vec::new();
    bees.push(37.0);
    expect(bees).to(be_an_empty_vector());
}

fn using_should_syntax() {
    use expector::should_syntax::*;
    1.should(eq(1));

    let mut bees: Vec<f64> = Vec::new();
    bees.push(37.0);
    bees.should(be_an_empty_vector());
}
