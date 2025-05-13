use crate::{and, or, xor};

/// 半加算器
/// 
/// 2つの1ビット入力（a, b）を受け取り、和（sum）と桁上げ（carry）を返す
/// 
/// * `a` - 1ビット目の入力
/// * `b` - 2ビット目の入力
/// 
/// 戻り値は (sum, carry) のタプル
pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    let sum = xor(a, b);    // 和は XOR
    let carry = and(a, b);  // 桁上げは AND
    (sum, carry)
}

/// 全加算器
/// 
/// 3つの1ビット入力（a, b, carry_in）を受け取り、和（sum）と桁上げ（carry_out）を返す
/// 
/// * `a` - 1ビット目の入力
/// * `b` - 2ビット目の入力
/// * `carry_in` - 前の桁からの桁上げ
/// 
/// 戻り値は (sum, carry_out) のタプル
pub fn full_adder(a: bool, b: bool, carry_in: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum, carry2) = half_adder(sum1, carry_in);
    let carry_out = or(carry1, carry2);
    (sum, carry_out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(false, false), (false, false)); // 0 + 0 = 0, carry 0
        assert_eq!(half_adder(false, true), (true, false));   // 0 + 1 = 1, carry 0
        assert_eq!(half_adder(true, false), (true, false));   // 1 + 0 = 1, carry 0
        assert_eq!(half_adder(true, true), (false, true));    // 1 + 1 = 0, carry 1
    }

    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(false, false, false), (false, false)); // 0 + 0 + 0 = 0, carry 0
        assert_eq!(full_adder(false, false, true), (true, false));   // 0 + 0 + 1 = 1, carry 0
        assert_eq!(full_adder(false, true, false), (true, false));   // 0 + 1 + 0 = 1, carry 0
        assert_eq!(full_adder(false, true, true), (false, true));    // 0 + 1 + 1 = 0, carry 1
        assert_eq!(full_adder(true, false, false), (true, false));   // 1 + 0 + 0 = 1, carry 0
        assert_eq!(full_adder(true, false, true), (false, true));    // 1 + 0 + 1 = 0, carry 1
        assert_eq!(full_adder(true, true, false), (false, true));    // 1 + 1 + 0 = 0, carry 1
        assert_eq!(full_adder(true, true, true), (true, true));      // 1 + 1 + 1 = 1, carry 1
    }
} 