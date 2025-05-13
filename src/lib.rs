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

/// XOR  (A ⊕ B) = NAND(NAND(A, B), NAND(A, NAND(A, B)), NAND(B, NAND(A, B)))
pub fn xor(a: bool, b: bool) -> bool {
    let t1 = nand(a, b);
    let t2 = nand(a, t1);
    let t3 = nand(b, t1);
    nand(t2, t3)
}

/// 2:1 マルチプレクサ  
/// sel が 0 なら a、1 なら b を出力
pub fn mux(sel: bool, a: bool, b: bool) -> bool {
    let not_sel = not(sel);
    let a_and_not_sel = and(a, not_sel);
    let b_and_sel = and(b, sel);
    or(a_and_not_sel, b_and_sel)
}

/// 1:2 デマルチプレクサ  
/// 入力 d を sel=0→(d,0), sel=1→(0,d) へ分配
pub fn demux(sel: bool, d: bool) -> (bool, bool) {
    let not_sel = not(sel);
    let o0 = and(d, not_sel);
    let o1 = and(d, sel);
    (o0, o1)
}

// 加算器モジュールを追加
pub mod adder;

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

    #[test]
    fn xor_gate() {
        assert_eq!(xor(false, false), false);
        assert_eq!(xor(false, true),  true);
        assert_eq!(xor(true,  false), true);
        assert_eq!(xor(true,  true),  false);
    }

    #[test]
    fn mux_gate() {
        assert_eq!(mux(false, false, false), false);
        assert_eq!(mux(false, false, true),  false);
    }

    #[test]
    fn demux_gate() {
        assert_eq!(demux(false, false), (false, false));
        assert_eq!(demux(false, true),  (true, false));
        assert_eq!(demux(true,  false), (false, false));
        assert_eq!(demux(true,  true),  (false, true));
    }
}
