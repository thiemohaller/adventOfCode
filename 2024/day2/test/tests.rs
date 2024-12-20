#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_is_safe() {
        assert!(report_is_safe("10 9 8 7")); // strictly descending, within diff 3
        assert!(report_is_safe("1 2 3"));    // ascending and safe
        assert!(!report_is_safe("10 10"));   // equal numbers (not safe)
        assert!(!report_is_safe("1 5"));     // difference more than 3 (not safe)
        assert!(!report_is_safe("2 1 2"));   // changes trend from descend to ascend (not safe)
    }
}
