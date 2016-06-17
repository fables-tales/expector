pub mod prelude;

use super::Matcher;

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

