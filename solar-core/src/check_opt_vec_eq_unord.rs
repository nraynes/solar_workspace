use crate::check_vec_eq_unord::check_vec_eq_unord;

/// Compares two optional vectors for equality, regardless of element order.
pub fn check_opt_vec_eq_unord<T>(opt_one: &Option<Vec<T>>, opt_two: &Option<Vec<T>>) -> bool
where
    T: Ord + Clone,
{
    if let Some(vec_one) = opt_one
        && let Some(vec_two) = opt_two
    {
        return check_vec_eq_unord(vec_one, vec_two);
    }
    opt_one == opt_two
}
