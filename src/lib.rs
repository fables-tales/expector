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


pub struct EqualityMatcher<E> {
    expected: E
}


impl <E, A> Matcher<A> for EqualityMatcher<E> where E: PartialEq<A> {
    fn matches(self, target: &A) -> bool {
        &self.expected == target
    }
}

pub struct BeAnEmptyVectorMatcher;

impl <A> Matcher<Vec<A>> for BeAnEmptyVectorMatcher {
    fn matches(self, target: &Vec<A>) -> bool {
        target.len() == 0
    }
}

pub trait Matcher<A> {
    fn matches(self, target: &A) -> bool;
}

pub fn expect<A>(target: A) -> ExpectationTarget<A> {
    ExpectationTarget { target: target }
}

pub fn eq<E>(expected: E) -> EqualityMatcher<E> {
    EqualityMatcher { expected: expected }
}

pub fn be_an_empty_vector() -> BeAnEmptyVectorMatcher {
    BeAnEmptyVectorMatcher
}
