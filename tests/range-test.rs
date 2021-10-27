#[cfg(test)]
mod range_test {
    extern crate pretty_assertions;

    use rust_d3_array::range::range;

    use pretty_assertions::assert_eq;

    // tape("range(start, stop, step) returns [start, start + step, start + 2 * step, … stop - step]", (test) => {
    //     test.deepEqual(d3.range(0, 5, 1), [0, 1, 2, 3, 4]);
    //     test.deepEqual(d3.range(0, 5, 2), [0, 2, 4]);
    //     test.deepEqual(d3.range(2, 5, 2), [2, 4]);
    //     test.deepEqual(d3.range(-1, 3, 2), [-1, 1]);
    //   });

    #[test]
    fn returns_start_stop_step() {
        println!("range(start, stop, step) returns [start, start + step, start + 2 * step, … stop - step]");
        assert_eq!(
            range(0_f64, 5_f64, 1_f64),
            vec![0_f64, 1_f64, 2_f64, 3_f64, 4_f64]
        );
        assert_eq!(range(0_f64, 5_f64, 2_f64), vec![0_f64, 2_f64, 4_f64]);
        assert_eq!(range(2_f64, 5_f64, 2_f64), vec![2_f64, 4_f64]);
        assert_eq!(range(-1_f64, 3_f64, 2_f64), vec![-1_f64, 1_f64]);
    }

    // tape("range(start, stop, step) returns an empty array if start >= stop and step > 0", (test) => {
    //     test.deepEqual(d3.range(5, 5, 2), []);
    //     test.deepEqual(d3.range(6, 5, 2), []);
    //     test.deepEqual(d3.range(10, 10, 1), []);
    //     test.deepEqual(d3.range(10, 10, 0.5), []);
    //     test.deepEqual(d3.range(0, 0, 1), []);
    //     test.deepEqual(d3.range(0, 0, 0.5), []);
    //     test.deepEqual(d3.range(20, 10, 2), []);
    //     test.deepEqual(d3.range(20, 10, 1), []);
    //     test.deepEqual(d3.range(20, 10, 0.5), []);
    //   });
    #[test]
    fn returns_empty() {
        println!("range(start, stop, step) returns an empty array if start >= stop and step > 0");
        assert_eq!(range(5_f64, 5_f64, 2_f64), vec![]);
        assert_eq!(range(6_f64, 5_f64, 2_f64), vec![]);
        assert_eq!(range(10_f64, 10_f64, 1_f64), vec![]);
        assert_eq!(range(10_f64, 10_f64, 0.5_f64), vec![]);

        assert_eq!(range(0_f64, 0_f64, 1_f64), vec![]);
        assert_eq!(range(0_f64, 0_f64, 0.5_f64), vec![]);

        assert_eq!(range(20_f64, 10_f64, 2_f64), vec![]);
        assert_eq!(range(20_f64, 10_f64, 1_f64), vec![]);
        assert_eq!(range(20_f64, 10_f64, 0.5_f64), vec![]);
    }
}
