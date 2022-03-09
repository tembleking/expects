#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]
#![deny(rustdoc::broken_intra_doc_links)]
//! This crate is a matcher/assertion library designed to help you write expressive tests.

/// Provides matcher functions for testing equality.
mod equal;
/// Provides matcher functions for iterable elements.
mod iter;

/// Provides matcher functions
pub mod matcher {
    pub use crate::equal::{be_err, be_false, be_ok, be_true, equal};
    pub use crate::iter::{consist_of, contain_element};
}

/// The Subject trait allows all types, once imported, to be asserted with the different matchers
/// provided by this crate.
///
/// Example of usage:
/// ```
/// use expects::matcher::equal;
/// use expects::Subject;
///
/// "foo".should(equal("foo"));
/// ```
pub trait Subject<T, V: Matcher<T>> {
    /// Provides the method to all types implementing this trait, so it can be asserted with other
    /// types that implement [Matcher].
    fn should(&self, matcher: V);
}

/// Generic implementation of the Subject trait for all the types.
impl<T, V> Subject<T, V> for T
where
    V: Matcher<T>,
{
    fn should(&self, matcher: V) {
        matcher.matches(self)
    }
}

/// A Matcher is a trait that can be implemented by custom types to perform assertions.
pub trait Matcher<T> {
    /// Contains the behavior of the implementing matcher to assert with values.
    fn matches(&self, actual_value: &T);
}
