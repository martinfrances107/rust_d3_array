use std::cmp::PartialOrd;

pub fn extent<T>(
  values: Vec<Option<T>>,
  valueof: Option<Box<dyn Fn(Option<T>) -> Option<T>>>,
) -> [Option<T>; 2]
where T: PartialOrd + Copy{
  let mut min: Option<T> = None;
  let mut max: Option<T> = None;
  match valueof {
    None => {
      for value in values {
        match value {
          Some(value_val) => {
            match min {
              None => {
                min = value;
              }
              Some(min_val) => {
                if min_val > value_val {
                  min = value;
                }
              }
            }
            match max {
              None => {
                max = value;
              }
              Some(max_val) => {
                if max_val < value_val {
                  max = value
                }
              }
            }
          }
          None => {}
        }
      }
    }

    Some(valueof) => {
      // let _index = -1;
      for value in values {
        let value_of_out = valueof(value);
        match value_of_out {
          Some(value_of_out) => {
            match min {
              Some(min_val) => {
                if min_val > value_of_out {
                  min = Some(value_of_out);
                }
              }
              None => {
                min = Some(value_of_out);
              }
            }
            match max {
              Some(max_val) => {
                if max_val < value_of_out {
                  max = Some(value_of_out);
                }
              }
              None => {
                max = Some(value_of_out);
              }
            }
          }
          None => {}
        }
      }
    }
  }
  return [min, max];
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
