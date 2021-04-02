// function* flatten(arrays) {
//     for (const array of arrays) {
//       yield* array;
//     }
//   }

#[inline]
pub fn merge<T>(arrays: Vec<Vec<T>>) -> Vec<T> {
    arrays.into_iter().flatten().collect()
}

// function* flatten(arrays) {
//     for (const array of arrays) {
//       yield* array;
//     }
//   }

//   export default function merge(arrays) {
//     return Array.from(flatten(arrays));
//   }
