//! LIX
//!
//! Lix Readability Test: https://en.wikipedia.org/wiki/Lix_(readability_test)
pub fn lix(a: usize, b: usize, c: usize) -> usize {
    let lix_return: usize = a / b + (c * 100) / a;
    lix_return
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = lix(400_000, 2_500, 50_000);
        assert_eq!(result, 172);
    }
}
