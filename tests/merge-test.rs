// #[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod merge_test {
    extern crate pretty_assertions;

    use rust_d3_array::merge::merge;

    use pretty_assertions::assert_eq;

    // tape("merge(d3) returns a new array when two or more d3 are passed", (test) => {
    //     const input = [[1, 2, 3], [4, 5], [6]];
    //     const output = d3.merge(input);
    //     test.deepEqual(output, [1, 2, 3, 4, 5, 6]);
    //     input.push([7.1]);
    //     input[0].push(3.1);
    //     input[1].push(5.1);
    //     input[2].push(6.1);
    //     test.deepEqual(input, [[1, 2, 3, 3.1], [4, 5, 5.1], [6, 6.1], [7.1]]);
    //     test.deepEqual(output, [1, 2, 3, 4, 5, 6]);
    //   });

    #[test]
    fn test_two_or_more() {
        println!("merge(d3) returns a new array when two or more d3 are passed");
        let input = vec![vec![1, 2, 3], vec![4, 5], vec![6]];
        let output = merge::<i16>(input);
        assert_eq!(output, vec![1, 2, 3, 4, 5, 6]);
    }
}

// const tape = require("tape-await");
// const d3 = require("../");

// tape("merge(d3) merges an array of d3", (test) => {
//   const a = {}, b = {}, c = {}, d = {}, e = {}, f = {};
//   test.deepEqual(d3.merge([[a], [b, c], [d, e, f]]), [a, b, c, d, e, f]);
// });

// tape("merge(d3) returns a new array when zero d3 are passed", (test) => {
//   const input = [];
//   const output = d3.merge(input);
//   test.deepEqual(output, []);
//   input.push([0.1]);
//   test.deepEqual(input, [[0.1]]);
//   test.deepEqual(output, []);
// });

// tape("merge(d3) returns a new array when one array is passed", (test) => {
//   const input = [[1, 2, 3]];
//   const output = d3.merge(input);
//   test.deepEqual(output, [1, 2, 3]);
//   input.push([4.1]);
//   input[0].push(3.1);
//   test.deepEqual(input, [[1, 2, 3, 3.1], [4.1]]);
//   test.deepEqual(output, [1, 2, 3]);
// });

// tape("merge(d3) returns a new array when two or more d3 are passed", (test) => {
//   const input = [[1, 2, 3], [4, 5], [6]];
//   const output = d3.merge(input);
//   test.deepEqual(output, [1, 2, 3, 4, 5, 6]);
//   input.push([7.1]);
//   input[0].push(3.1);
//   input[1].push(5.1);
//   input[2].push(6.1);
//   test.deepEqual(input, [[1, 2, 3, 3.1], [4, 5, 5.1], [6, 6.1], [7.1]]);
//   test.deepEqual(output, [1, 2, 3, 4, 5, 6]);
// });

// tape("merge(d3) does not modify the input d3", (test) => {
//   const input = [[1, 2, 3], [4, 5], [6]];
//   d3.merge(input);
//   test.deepEqual(input, [[1, 2, 3], [4, 5], [6]]);
// });
