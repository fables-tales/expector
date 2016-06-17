use std::borrow::Borrow;
use std::collections::*;
use std::hash::Hash;
use std::ops::Deref;

use Matcher;

pub fn contain<T>(expected: T) -> ContainsMatcher<T> {
    ContainsMatcher { expected: expected }
}

pub struct ContainsMatcher<T> {
    expected: T,
}

#[cfg(feature = "pattern")]
impl<'a, T> Matcher<&'a str> for ContainsMatcher<T> where
    T: ::std::str::pattern::Pattern<'a>,
{
    fn matches(self, target: &&'a str) -> bool {
        target.contains(self.expected)
    }
}

#[cfg(not(feature = "pattern"))]
impl<'a, 'b> Matcher<&'a str> for ContainsMatcher<&'b str> {
    fn matches(self, target: &&'a str) -> bool {
        target.contains(self.expected)
    }
}

impl<T> Matcher<String> for ContainsMatcher<T> where
    for<'a> ContainsMatcher<T>: Matcher<&'a str>,
{
    fn matches(self, target: &String) -> bool {
        self.matches(&target.deref())
    }
}

impl<'a, T> Matcher<&'a [T]> for ContainsMatcher<T> where
    T: PartialEq,
{
    fn matches(self, target: &&'a [T]) -> bool {
        target.contains(&self.expected)
    }
}

impl<T> Matcher<Vec<T>> for ContainsMatcher<T> where
    for<'a> ContainsMatcher<T>: Matcher<&'a [T]>,
{
    fn matches(self, target: &Vec<T>) -> bool {
        self.matches(&target.deref())
    }
}

impl<T, U> Matcher<BTreeSet<T>> for ContainsMatcher<U> where
    T: Borrow<U> + Ord,
    U: Ord,
{
    fn matches(self, target: &BTreeSet<T>) -> bool {
        target.contains(&self.expected)
    }
}

impl<T, U> Matcher<HashSet<T>> for ContainsMatcher<U> where
    T: Borrow<U> + Hash + Eq,
    U: Hash + Eq,
{
    fn matches(self, target: &HashSet<T>) -> bool {
        target.contains(&self.expected)
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn contains_matcher_works_with_strs() {
        expect("foobar").to(contain("f"));
        expect("foobar").to(contain("foo"));
        expect("foobar").to(contain("foobar"));
        expect("Sam Phippen").to(contain("Sam"));
    }

    #[test]
    #[should_panic]
    fn contains_can_fail_with_strs() {
        expect("hello").to(contain("world"));
    }

    #[test]
    fn contains_matcher_works_with_strings() {
        expect("foobar".to_string()).to(contain("f"));
        expect("foobar".to_string()).to(contain("foo"));
        expect("foobar".to_string()).to(contain("foobar"));
        expect("Sam Phippen".to_string()).to(contain("Sam"));
    }

    #[test]
    #[should_panic]
    fn contains_can_fail_with_strings() {
        expect("hello".to_string()).to(contain("world"));
    }

    #[test]
    #[cfg(feature = "pattern")]
    fn contains_matcher_works_with_strs_and_patterns() {
        expect("foobar").to(contain('f'));
        expect("f00bar").to(contain(|c: char| c.is_digit(10)));
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "pattern")]
    fn contains_can_fail_with_strs_and_patterns() {
        expect("foobar").to(contain(|c: char| c.is_digit(10)));
    }

    #[test]
    fn contains_works_with_slices() {
        expect(&[1, 2, 3] as &[i32]).to(contain(1));
        expect(&["foo", "bar", "baz"] as &[&str]).to(contain("bar"));
    }

    #[test]
    #[should_panic]
    fn contains_can_fail_with_slices() {
        expect(&[1, 2, 3] as &[i32]).to(contain(4));
    }

    #[test]
    fn contains_works_with_vecs() {
        expect(vec![1, 2, 3]).to(contain(1));
        expect(vec!["foo", "bar", "baz"]).to(contain("bar"));
    }

    #[test]
    #[should_panic]
    fn contains_can_fail_with_vecs() {
        expect(vec![1, 2, 3]).to(contain(4));
    }
}
