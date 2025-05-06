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

/// AND  (A · B) = NOT (NAND(A, B))
pub fn and(a: bool, b: bool) -> bool {
    let n = nand(a, b);
    nand(n, n)
}

/// OR   (A + B) = NAND(NOT A, NOT B)
pub fn or(a: bool, b: bool) -> bool {
    let na = nand(a, a);
    let nb = nand(b, b);
    nand(na, nb)
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

    #[test]
    fn and_gate() {
        assert_eq!(and(false, false), false);
        assert_eq!(and(false, true),  false);
        assert_eq!(and(true,  false), false);
        assert_eq!(and(true,  true),  true);
    }

    #[test]
    fn or_gate() {
        assert_eq!(or(false, false), false);
        assert_eq!(or(false, true),  true);
        assert_eq!(or(true,  false), true);
        assert_eq!(or(true,  true),  true);
    }
}
