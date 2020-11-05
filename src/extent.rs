use std::cmp::PartialOrd;

pub fn extent<T>(values: Vec<T>) -> [T; 2]
where
    T: PartialOrd + Copy,
{
    let mut min: Option<T> = None;
    let mut max: Option<T> = None;
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

    return [min.unwrap(), max.unwrap()];
}

// export default function(values, valueof) {
//   let min;
//   let max;
//   if (valueof === undefined) {
//     for (const value of values) {
//       if (value != null) {
//         if (min === undefined) {
//           if (value >= value) min = max = value;
//         } else {
//           if (min > value) min = value;
//           if (max < value) max = value;
//         }
//       }
//     }
//   } else {
//     let index = -1;
//     for (let value of values) {
//       if ((value = valueof(value, ++index, values)) != null) {
//         if (min === undefined) {
//           if (value >= value) min = max = value;
//         } else {
//           if (min > value) min = value;
//           if (max < value) max = value;
//         }
//       }
//     }
//   }
//   return [min, max];
// }
