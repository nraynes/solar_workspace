use crate::sorted::sorted;

/// Compares two vector slices for equality, regardless of element order.
pub fn check_vec_eq_unord<T>(vec_one: &[T], vec_two: &[T]) -> bool
where
    T: Ord + Clone,
{
    sorted(vec_one.to_owned()) == sorted(vec_two.to_owned())
}
