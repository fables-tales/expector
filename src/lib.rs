pub mod matchers;
pub mod prelude;

pub struct ExpectationTarget<A> {
    target: A
}

impl <A> ExpectationTarget<A> {
    pub fn to<T: Matcher<A>>(&self, matcher: T) {
        if !matcher.matches(&self.target) {
            panic!("Expector match failed");
        }
    }
}

pub trait Matcher<A> {
    fn matches(self, target: &A) -> bool;
}

pub fn expect<A>(target: A) -> ExpectationTarget<A> {
    ExpectationTarget { target: target }
}
