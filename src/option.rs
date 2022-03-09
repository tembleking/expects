use crate::Matcher;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Provides equality matching that can be accessed with the function [`be_some`].
pub struct SomeOptionMatcher<M, T> {
    inner_matcher: M,
    phantom: PhantomData<T>,
}

impl<M, T> Matcher<Option<T>> for SomeOptionMatcher<M, T>
where
    M: Matcher<T>,
{
    fn matches(&self, actual_value: &Option<T>) {
        match actual_value {
            Some(inner) => {
                self.inner_matcher.matches(inner);
            }
            None => {
                panic!("expected Option to be Some, but it was None");
            }
        }
    }
}

/// Expects the provided value to be an [`Option`] of variant [`Some`] that contains something that matches an inner matcher.
/// ```
/// # use std::error::Error;
/// # use expects::matcher::{be_some, be_true};
/// # use expects::Subject;
/// Some(true).should(be_some(be_true()))
/// ```
/// The following snippets should panic:
/// ```should_panic
/// # use expects::matcher::{be_some, be_true};
/// # use expects::Subject;
/// Some(false).should(be_some(be_true()));
/// ```
/// ```should_panic
/// # use expects::matcher::{be_some, be_true};
/// # use expects::Subject;
/// Option::<bool>::None.should(be_some(be_true()));
/// ```
pub fn be_some<M, T>(expected: M) -> SomeOptionMatcher<M, T>
where
    M: Matcher<T>,
{
    SomeOptionMatcher {
        inner_matcher: expected,
        phantom: PhantomData::default(),
    }
}

/// Provides equality matching that can be accessed with the function [`be_none`].
pub struct NoneOptionMatcher {}

impl<T> Matcher<Option<T>> for NoneOptionMatcher
where
    T: Debug,
{
    fn matches(&self, actual_value: &Option<T>) {
        match actual_value {
            Some(value) => {
                panic!("expected Option to be None but it was Some({:?})", value);
            }
            None => {}
        }
    }
}

/// Expects the provided value to be an [`Option`] of variant [`None`].
/// ```
/// # use expects::matcher::{be_none, be_true};
/// # use expects::{Matcher, Subject};
/// Option::<()>::None.should(be_none())
/// ```
/// The following snippet should panic:
/// ```should_panic
/// # use expects::matcher::{be_none, be_true};
/// # use expects::Subject;
/// Some(()).should(be_none());
/// ```
#[must_use]
pub fn be_none() -> NoneOptionMatcher {
    NoneOptionMatcher {}
}
