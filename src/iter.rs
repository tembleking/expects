use crate::Matcher;
use std::fmt::Debug;

/// Matches an iterable value looking for an specific element to be present.
/// It is used with the function [`contain_element`].
pub struct ContainsMatcher<T> {
    element_contained: T,
}

impl<T, V> Matcher<T> for ContainsMatcher<V>
where
    T: IntoIterator<Item = V> + Debug + Clone,
    V: PartialEq + Debug,
{
    fn matches(&self, actual_value: &T) {
        for x in actual_value.clone() {
            if x == self.element_contained {
                return;
            }
        }
        debug_assert!(
            false,
            "\
expected
  `{:?}`
to contain
  `{:?}`'
but it's not present",
            &actual_value, self.element_contained
        );
    }
}

/// Expects the provided value to contain an element.
/// ```
/// # use expects::matcher::contain_element;
/// # use expects::Subject;
/// let v = vec![1, 2, 3];
/// v.should(contain_element(1));
/// v.should(contain_element(2));
/// v.should(contain_element(3));
/// ```
///
/// The following snippet will panic:
/// ```should_panic
/// # use expects::matcher::contain_element;
/// # use expects::Subject;
/// let v = vec![1, 2, 3];
/// v.should(contain_element(4));
/// ```
pub fn contain_element<T>(element: T) -> ContainsMatcher<T> {
    ContainsMatcher {
        element_contained: element,
    }
}

/// Matches an iterable value looking for all the specified elements to be present, and no more,
/// without an specified order.
/// It is used with the function [`consist_of`].
pub struct ConsistOfMatcher<T>
where
    T: Debug,
{
    elements_to_consist_of: Vec<T>,
}

impl<V> ConsistOfMatcher<V>
where
    V: PartialEq + Debug,
{
    fn check_if_there_are_values_in_actual_not_present_in_expected<T>(&self, actual_value: &T)
    where
        T: IntoIterator<Item = V> + Clone + Debug,
    {
        'outer: for (pos, x) in actual_value.clone().into_iter().enumerate() {
            for y in &self.elements_to_consist_of {
                if &x == y {
                    continue 'outer;
                }
            }
            debug_assert!(
                false,
                "\
expected:
  `{:?}`
to consist of:
  `{:?}`
but the extra element at position `{}` in the original value:
  `{:?}`
was not expected to be present
",
                actual_value, self.elements_to_consist_of, pos, x
            );
        }
    }

    fn check_if_there_are_values_in_expected_not_present_in_actual<T>(&self, actual_value: &T)
    where
        T: IntoIterator<Item = V> + Clone + Debug,
        V: PartialEq,
    {
        'outer: for (pos, x) in self.elements_to_consist_of.iter().enumerate() {
            for y in actual_value.clone() {
                if x == &y {
                    continue 'outer;
                }
            }
            debug_assert!(
                false,
                "\
expected:
  `{:?}`
to consist of:
  `{:?}`
but the element at position `{}` in the expectation:
  `{:?}`
 was missing in the original value",
                actual_value, self.elements_to_consist_of, pos, x
            );
        }
    }
}

impl<T, V> Matcher<T> for ConsistOfMatcher<V>
where
    T: IntoIterator<Item = V> + Clone + Debug,
    V: PartialEq + Debug,
{
    fn matches(&self, actual_value: &T) {
        self.check_if_there_are_values_in_actual_not_present_in_expected(actual_value);
        self.check_if_there_are_values_in_expected_not_present_in_actual(actual_value);
    }
}

/// Expects the provided value to consist of the specified elements, in an unspecified order.
/// ```
/// # use expects::matcher::consist_of;
/// # use expects::Subject;
///  vec![1, 2, 3].should(consist_of(&[3, 1, 2]));
///  (1..3).should(consist_of(&[2, 1]));
/// ```
///
/// The following snippet will panic because there is an extra element in the original value:
/// ```should_panic
/// # use expects::matcher::consist_of;
/// # use expects::Subject;
/// let v = vec![1, 2, 3];
/// v.should(consist_of(&[3, 1]))
/// ```
///
/// The following snippet will panic because there is a missing element in the original value:
/// ```should_panic
/// # use expects::matcher::consist_of;
/// # use expects::Subject;
/// let v = vec![1, 2, 3];
/// v.should(consist_of(&[3, 1, 2, 4]))
/// ```
pub fn consist_of<T>(elements: &[T]) -> ConsistOfMatcher<T>
where
    T: Clone + Debug,
{
    ConsistOfMatcher {
        elements_to_consist_of: elements.to_vec(),
    }
}
