use crate::Matcher;

use std::fmt::Debug;

/// The [`EqualityMatcher`] provides equality matching that can be accessed with functions
/// like [`equal`].
pub struct EqualityMatcher<T> {
    expected: T,
}

impl<T> EqualityMatcher<T> {
    pub fn new(expected: T) -> Self {
        EqualityMatcher { expected }
    }
}

impl<T, V> Matcher<T> for EqualityMatcher<V>
where
    T: PartialEq + Debug,
    V: Into<T> + Debug + Clone,
{
    fn matches(&self, actual_value: &T) {
        debug_assert!(
            &self.expected.clone().into() == actual_value,
            "expected '{:?}' to be equal to '{:?}'",
            actual_value,
            self.expected
        );
    }
}

/// Expects the provided value to match with equality.
/// ```
/// # use expects::matcher::equal;
/// # use expects::Subject;
/// "foo".should(equal("foo"));
/// 3.should(equal(3));
/// 5.6.should(equal(5.6));
/// String::from("foo").should(equal("foo"));
/// ```
///
/// The following snippet will panic:
/// ```should_panic
/// # use expects::matcher::equal;
/// # use expects::Subject;
/// "foo".should(equal("bar"));
/// ```
pub fn equal<T>(other: T) -> EqualityMatcher<T> {
    EqualityMatcher::new(other)
}
