use std::cmp::PartialOrd;

pub fn extent<T>(values: Vec<T>, value_of: Option<Box<dyn Fn(T, T, T) -> T>>) -> [T; 2]
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
            unimplemented!("Des not yet support valueof function.");
        }
        
        
}

    return [min.unwrap(), max.unwrap()];
}
