pub mod matchers;
pub mod dsl;

pub struct ExpectationTarget<A> {
    target: A
}

impl <A> ExpectationTarget<A> {
    pub fn to<T: Matcher<A>>(&self, matcher: T) {
        if !matcher.matches(&self.target) {
            panic!("Expector match failed");
        }
    }
    pub fn to_not<T: Matcher<A>>(&self, matcher: T) {
        if matcher.matches(&self.target) {
            panic!("Expector not-match failed");
        }
    }

    pub fn not_to<T: Matcher<A>>(&self, matcher: T) {
        if matcher.matches(&self.target) {
            panic!("Expector not-match failed");
        }
    }
}

pub trait Matcher<A> {
    fn matches(self, target: &A) -> bool;
}
