extern crate expector;
use expector::prelude::*;

#[test]
fn equality_matcher_matches_things_that_are_equal() {
    #[derive(PartialEq)]
    struct Foo(i32);

    expect(1).to(eq(1));
    expect(2).to(eq(2));
    expect(true).to(eq(true));
    expect(Some(1)).to(eq(Some(1)));
    expect(None::<i32>).to(eq(None));
    expect(Foo(2)).to(eq(Foo(2)));
}

#[test]
#[should_panic]
fn eq_matcher_fails_when_things_are_not_equal() {
    expect(1).to(eq(2));
}

#[test]
fn be_an_empty_vec_matches_emtpy_vecs() {
    expect(Vec::<i32>::new()).to(be_an_empty_vector());
    expect(Vec::<String>::new()).to(be_an_empty_vector());
}

#[test]
#[should_panic]
fn be_an_empty_vec_fails_when_vecs_are_not_empty() {
    expect(vec![1]).to(be_an_empty_vector())
}
