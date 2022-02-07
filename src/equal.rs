use crate::Matcher;

use std::fmt::Display;

/// The EqualMatcher provides equality matching that can be accessed with functions
/// like [equal], [be_true] or [be_false].
pub struct EqualMatcher<T> {
    expected: T,
}

impl<T, V> Matcher<T> for EqualMatcher<V>
where
    T: PartialEq + Display,
    V: Into<T> + Display + Clone,
{
    fn matches(&self, actual_value: T) {
        debug_assert!(
            self.expected.clone().into() == actual_value,
            "expected '{}' to be equal to '{}'",
            actual_value,
            self.expected
        )
    }
}

/// Expects the provided value to match with equality, for example:
/// ```
/// # use expects::equal::equal;
/// # use expects::Subject;
/// "foo".should(equal("foo"));
/// 3.should(equal(3));
/// 5.6.should(equal(5.6));
/// String::from("foo").should(equal("foo"));
/// ```
///
/// The following snippet will panic:
/// ```should_panic
/// # use expects::equal::equal;
/// # use expects::Subject;
/// "foo".should(equal("bar"));
/// ```
pub fn equal<T>(other: T) -> EqualMatcher<T> {
    EqualMatcher { expected: other }
}

/// Expects the provided value to match with `true`, for example:
/// ```
/// # use expects::equal::be_true;
/// # use expects::Subject;
/// true.should(be_true());
/// ```
///
/// The following snippet will panic:
/// ```should_panic
/// # use expects::equal::be_true;
/// # use expects::Subject;
/// false.should(be_true());
/// ```
pub fn be_true() -> EqualMatcher<bool> {
    EqualMatcher { expected: true }
}

/// Expects the provided value to match with `false`, for example:
/// ```
/// # use expects::equal::be_false;
/// # use expects::Subject;
/// false.should(be_false());
/// ```
///
/// The following snippet will panic:
/// ```should_panic
/// # use expects::equal::be_false;
/// # use expects::Subject;
/// true.should(be_false());
/// ```
pub fn be_false() -> EqualMatcher<bool> {
    EqualMatcher { expected: false }
}
