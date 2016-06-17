pub use matchers::prelude::*;
use ExpectationTarget;

pub fn expect<A>(target: A) -> ExpectationTarget<A> {
    ExpectationTarget { target: target }
}
