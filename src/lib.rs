//! LIX
//!
//! Lix Readability Test: <https://en.wikipedia.org/wiki/Lix_(readability_test)>
pub fn lix(a: f64, b: f64, c: f64) -> f64 {
    let lix_return: f64 = a / b + (c * 100f64) / a;
    lix_return
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = lix(400_000.0, 2_500.0, 50_000.0);
        assert_eq!(result, 172.5);
    }
}
