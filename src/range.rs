use geo::CoordFloat;

// use num_traits::ceil;
// export default function(start, stop, step) {
// 	start = +start, stop = +stop, step = (n = arguments.length) < 2 ? (stop = start, start = 0, 1) : n < 3 ? 1 : +step;

// 	var i = -1,
// 		n = Math.max(0, Math.ceil((stop - start) / step)) | 0,
// 		range = new Array(n);

// 	while (++i < n) {
// 	  range[i] = start + i * step;
// 	}

// 	return range;
//   }

/// Generates a numeric sequence starting from the given start and stop values.
///
/// The output range does not include 'stop'.
pub fn range<T>(start: T, stop: T, step: T) -> Vec<T>
where
    T: CoordFloat,
{
    // TODO must find a way to pre-calculate n
    // let delta = ((stop - start) / step).ceil();
    // let n = match delta.try_into() {
    //     Ok(n) => n,
    //     Err(_) => 0usize,
    // };
    // let mut v = Vec::with_capacity(n);
    let mut v = Vec::new();

    let mut value = start;
    while value < stop {
        v.push(value);
        value = value + step;
    }
    v
}
