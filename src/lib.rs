pub mod algo;
pub mod gear;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    use algo::utilities::compare_version_string;
    #[test]
    fn is_compare_version_string_correctly() {
        assert_eq!(compare_version_string("1", "1"), 0);
        assert_eq!(compare_version_string("1", "0"), 1);
        assert_eq!(compare_version_string("0", "1"), -1);
        assert_eq!(compare_version_string("1.01", "1.001"), 0);
        assert_eq!(compare_version_string("1", "1.0.0"), 0);
        assert_eq!(compare_version_string("123", "3"), 1);
        assert_eq!(compare_version_string("1.10.1.rs", "1.10.1"), 0);
        assert_eq!(compare_version_string("1.10.1rs", "1.10.1"), 0);
        assert_eq!(compare_version_string("1.10.1-rs", "1.10c.1"), 0);
        assert_eq!(compare_version_string("2024-02-05", "0.40"), 1);
    }
}
