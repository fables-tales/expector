use matchers::*;

pub fn eq<E>(expected: E) -> EqualityMatcher<E> {
    EqualityMatcher { expected: expected }
}

pub fn be_an_empty_vector() -> BeAnEmptyVectorMatcher {
    BeAnEmptyVectorMatcher
}
