/// Assert that the absolute difference between two quantities is small.
pub fn assert_close(a: f64, b: f64, delta: Option<f64>) -> bool {
    let delta = delta.unwrap_or(0.00000001_f64);
    (a - b).abs() <= delta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_close() {
        assert_eq!(assert_close(0.1 + 0.2, 0.3, None), true);
        assert_eq!(assert_close(0.1, 0.1, None), true);
        assert_eq!(assert_close(0.1, 0.12, None), false);
    }
}
