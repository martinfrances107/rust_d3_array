use std::cmp::PartialOrd;

type ValueOfFn<T> = Box<dyn Fn(T, T, T) -> T>;

/// Return the min and max simultaneously.
pub fn extent_f64(values_in: Vec<f64>, value_of: Option<ValueOfFn<f64>>) -> [f64; 2] {
    let mut values = values_in;
    match value_of {
        None => {
            // Drop NAN early.
            values.retain(|&x| !x.is_nan());
        }
        Some(_) => {
            unimplemented!("Not yet supported: extent_f64() valueof function parameter.");
        }
    }
    extent(values, value_of)
}

/// Return the min and max simultaneously.
pub fn extent<T>(values: Vec<T>, value_of: Option<ValueOfFn<T>>) -> [T; 2]
where
    T: PartialOrd + Copy,
{
    let mut min: Option<T> = None;
    let mut max: Option<T> = None;
    match value_of {
        None => {
            for value in values {
                match min {
                    None => {
                        min = Some(value);
                    }
                    Some(min_val) => {
                        if min_val > value {
                            min = Some(value);
                        }
                    }
                }
                match max {
                    None => {
                        max = Some(value);
                    }
                    Some(max_val) => {
                        if max_val < value {
                            max = Some(value);
                        }
                    }
                }
            }
        }
        Some(_) => {
            unimplemented!("Not yet supported: extent() valueof function parameter.");
        }
    }

    [min.unwrap(), max.unwrap()]
}
