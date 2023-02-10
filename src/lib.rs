pub mod equation;
pub mod math;

pub fn get_version() -> &'static str {
    "0.0.1"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version() {
        let result = get_version();
        assert_eq!(result, "0.0.1");
    }
}
