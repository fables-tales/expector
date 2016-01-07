use super::{expect, Matcher};

pub trait ShouldSyntax: Sized {
    fn should<T: Matcher<Self>>(self, matcher: T) {
        expect(self).to(matcher)
    }
}

impl<T> ShouldSyntax for T {}
