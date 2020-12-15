#[cfg(test)]
mod polygon_contains_test {
    extern crate pretty_assertions;

    use rust_d3_array::extent::extent;
    use rust_d3_array::extent::extent_f64;

    use pretty_assertions::assert_eq;

    #[test]
    fn returns_the_least_and_greatest_numeric() {
        println!("extent(array) returns the least and greatest numeric values for numbers");
        assert_eq!(extent(vec![1], None), [1, 1]);
        assert_eq!(extent(vec![1], None), [1, 1]);
        assert_eq!(extent(vec![5, 1, 2, 3, 4], None), [1, 5]);
        assert_eq!(extent(vec![20, 3], None), [3, 20]);
        assert_eq!(extent(vec![3, 20], None), [3, 20]);
    }

    #[test]
    fn test_ignores_nan() {
        println!("extent(array) ignores null, undefined and NaN");
        assert_eq!(
            extent_f64(vec![f64::NAN, 1f64, 2f64, 3f64, 4f64, 5f64], None),
            [1f64, 5f64]
        );
    }
}
