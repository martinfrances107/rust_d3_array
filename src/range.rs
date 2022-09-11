use geo::CoordFloat;

/// Generates a numeric sequence starting from the given start and stop values.
///
/// The output range does not include 'stop'.
pub fn range<T>(start: T, stop: T, step: T) -> Vec<T>
where
    T: CoordFloat,
{
    let count = ((stop - start) / step).floor().to_usize();
    match count {
        Some(count) => {
            let mut v = Vec::with_capacity(count);

            let mut value = start;
            while value < stop {
                v.push(value);
                value = value + step;
            }
            v
        }
        None => {
            vec![]
        }
    }
}
