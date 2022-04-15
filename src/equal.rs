use crate::Matcher;
use pretty_assertions::{assert_eq, assert_ne};
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
        assert_eq!(&self.expected.clone().into(), actual_value);
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

/// The [`InequalityMatcher`] provides inequality matching that can be accessed with functions
/// like [`not_equal`].
pub struct InequalityMatcher<T> {
    expected: T,
}

impl<T> InequalityMatcher<T> {
    pub fn new(expected: T) -> Self {
        InequalityMatcher { expected }
    }
}

impl<T, V> Matcher<T> for InequalityMatcher<V>
where
    T: PartialEq + Debug,
    V: Into<T> + Debug + Clone,
{
    fn matches(&self, actual_value: &T) {
        assert_ne!(&self.expected.clone().into(), actual_value,);
    }
}

/// Expects the provided value to match with equality.
/// ```
/// # use expects::matcher::not_equal;
/// # use expects::Subject;
/// "foo".should(not_equal("bar"));
/// 3.should(not_equal(4));
/// 5.6.should(not_equal(6.6));
/// String::from("foo").should(not_equal("bar"));
/// ```
///
/// The following snippet will panic:
/// ```should_panic
/// # use expects::matcher::not_equal;
/// # use expects::Subject;
/// "foo".should(not_equal("foo"));
/// ```
#[allow(clippy::module_name_repetitions)]
pub fn not_equal<T>(other: T) -> InequalityMatcher<T> {
    InequalityMatcher::new(other)
}
