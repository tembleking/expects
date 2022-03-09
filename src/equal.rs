use crate::Matcher;

use std::fmt::Debug;
use std::marker::PhantomData;

/// The EqualMatcher provides equality matching that can be accessed with functions
/// like [equal], [be_true] or [be_false].
pub struct EqualMatcher<T> {
    expected: T,
}

impl<T, V> Matcher<T> for EqualMatcher<V>
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
        )
    }
}

/// Expects the provided value to match with equality, for example:
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
pub fn equal<T>(other: T) -> EqualMatcher<T> {
    EqualMatcher { expected: other }
}

/// Expects the provided value to match with `true`, for example:
/// ```
/// # use expects::matcher::be_true;
/// # use expects::Subject;
/// true.should(be_true());
/// ```
///
/// The following snippet will panic:
/// ```should_panic
/// # use expects::matcher::be_true;
/// # use expects::Subject;
/// false.should(be_true());
/// ```
pub fn be_true() -> EqualMatcher<bool> {
    EqualMatcher { expected: true }
}

/// Expects the provided value to match with `false`, for example:
/// ```
/// # use expects::matcher::be_false;
/// # use expects::Subject;
/// false.should(be_false());
/// ```
///
/// The following snippet will panic:
/// ```should_panic
/// # use expects::matcher::be_false;
/// # use expects::Subject;
/// true.should(be_false());
/// ```
pub fn be_false() -> EqualMatcher<bool> {
    EqualMatcher { expected: false }
}

/// Provides equality matching that can be accessed with the function [be_ok].
pub struct OkResultMatcher<M, T> {
    inner_matcher: M,
    phantom: PhantomData<T>,
}

impl<M, T, E> Matcher<Result<T, E>> for OkResultMatcher<M, T>
where
    M: Matcher<T>,
    E: Debug,
{
    fn matches(&self, actual_value: &Result<T, E>) {
        match actual_value {
            Ok(inner) => {
                self.inner_matcher.matches(inner);
            }
            Err(err) => {
                debug_assert!(
                    false,
                    "expected to be Ok, but there was an error: {:?}",
                    err
                );
            }
        }
    }
}

/// Provides equality matching that can be accessed with the function [be_err].
pub struct ErrResultMatcher<M, E> {
    inner_matcher: M,
    phantom: PhantomData<E>,
}

impl<M, T, E> Matcher<Result<T, E>> for ErrResultMatcher<M, E>
where
    M: Matcher<E>,
    T: Debug,
{
    fn matches(&self, actual_value: &Result<T, E>) {
        match actual_value {
            Ok(ok) => {
                debug_assert!(false, "expected to be an error but it was Ok: {:?}", ok);
            }
            Err(inner) => {
                self.inner_matcher.matches(inner);
            }
        }
    }
}

/// Expects the provided value to be a result of variant Ok that contains something that matches an inner matcher, for example:
/// ```
/// # use std::error::Error;
/// # use expects::matcher::{be_ok, be_true};
/// # use expects::Subject;
/// Result::<bool, Box<dyn Error>>::Ok(true).should(be_ok(be_true()))
/// ```
/// The following snippet should panic:
/// ```should_panic
/// # use expects::matcher::{be_ok, be_true};
/// # use expects::Subject;
/// Result::<bool,()>::Ok(false).should(be_ok(be_true()));
/// Result::<bool,()>::Err(()).should(be_ok(be_true()));
/// ```
pub fn be_ok<M, T>(expected: M) -> OkResultMatcher<M, T>
where
    M: Matcher<T>,
{
    OkResultMatcher {
        inner_matcher: expected,
        phantom: PhantomData::default(),
    }
}

/// Expects the provided value to be a result of variant Err that contains something that matches an inner matcher, for example:
/// ```
/// # use expects::matcher::{be_err, be_true};
/// # use expects::{Matcher, Subject};
/// Result::<(), bool>::Err(true).should(be_err(be_true()))
/// ```
/// The following snippet should panic:
/// ```should_panic
/// # use expects::matcher::{be_err, be_true};
/// # use expects::Subject;
/// Result::<(), bool>::Err(false).should(be_err(be_true()));
/// Result::<(), bool>::Ok(()).should(be_err(be_true()));
/// ```
pub fn be_err<M, T>(expected: M) -> ErrResultMatcher<M, T>
where
    M: Matcher<T>,
{
    ErrResultMatcher {
        inner_matcher: expected,
        phantom: PhantomData::default(),
    }
}
