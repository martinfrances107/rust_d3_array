/// Given a array of arrays flattern.
#[inline]
pub fn merge<T>(arrays: Vec<Vec<T>>) -> Vec<T> {
    arrays.into_iter().flatten().collect()
}
