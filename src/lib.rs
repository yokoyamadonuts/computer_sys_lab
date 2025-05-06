/// 2入力 NAND ゲート
///
/// *真理値表*  
/// A | B | NAND  
/// 0 | 0 | 1  
/// 0 | 1 | 1  
/// 1 | 0 | 1  
/// 1 | 1 | 0
pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

/// NOT  (￢A) = NAND(A, A)
pub fn not(a: bool) -> bool {
    nand(a, a)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nand_gate() {
        assert_eq!(nand(false, false), true);
        assert_eq!(nand(false, true),  true);
        assert_eq!(nand(true,  false), true);
        assert_eq!(nand(true,  true),  false);
    }

    #[test]
    fn not_gate() {
        assert_eq!(not(false), true);
        assert_eq!(not(true),  false);
    }
}
