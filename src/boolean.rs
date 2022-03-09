use crate::equal::EqualityMatcher;

/// Expects the provided value to match with `true`.
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
#[must_use]
pub fn be_true() -> EqualityMatcher<bool> {
    EqualityMatcher::new(true)
}

/// Expects the provided value to match with `false`.
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
#[must_use]
pub fn be_false() -> EqualityMatcher<bool> {
    EqualityMatcher::new(false)
}
