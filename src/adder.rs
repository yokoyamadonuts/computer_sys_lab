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

/// nビット加算器
/// 
/// 2つのnビット入力（a, b）を受け取り、和（sum）と最終桁上げ（carry）を返す
/// 
/// * `a` - 1つ目のnビット入力（LSB→MSB順）
/// * `b` - 2つ目のnビット入力（LSB→MSB順）
/// 
/// 戻り値は (sum, carry) のタプル
/// - sum: nビットの和（LSB→MSB順）
/// - carry: 最終桁上げ
pub fn n_bit_adder(a: &[bool], b: &[bool]) -> (Vec<bool>, bool) {
    // aとbの長さが異なる場合は、短い方を0で埋める
    let n = a.len().max(b.len());
    
    let mut sum = Vec::with_capacity(n);
    let mut carry = false;
    
    for i in 0..n {
        let bit_a = if i < a.len() { a[i] } else { false };
        let bit_b = if i < b.len() { b[i] } else { false };
        
        let (bit_sum, bit_carry) = full_adder(bit_a, bit_b, carry);
        sum.push(bit_sum);
        carry = bit_carry;
    }
    
    (sum, carry)
}

/// インクリメンタ
/// 
/// nビット入力に1を加算する
/// 
/// * `a` - nビット入力（LSB→MSB順）
/// 
/// 戻り値は (result, overflow) のタプル
/// - result: 加算結果（LSB→MSB順）
/// - overflow: オーバーフロー発生フラグ（すべてのビットが1の場合にtrueになる）
pub fn incrementer(a: &[bool]) -> (Vec<bool>, bool) {
    // 加算する1をビット配列として表現: [true]（LSBのみ1）
    let increment = [true];
    
    // n_bit_adderを利用して加算
    n_bit_adder(a, &increment)
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
    
    #[test]
    fn test_n_bit_adder() {
        // 0 + 0 = 0
        assert_eq!(
            n_bit_adder(&[false, false], &[false, false]),
            (vec![false, false], false)
        );
        
        // 1 + 1 = 2 (10 in binary)
        assert_eq!(
            n_bit_adder(&[true], &[true]),
            (vec![false], true)
        );
        
        // 01 + 01 = 02 (10 in binary)
        assert_eq!(
            n_bit_adder(&[true, false], &[true, false]),
            (vec![false, true], false)
        );
        
        // 11 + 01 = 100 (4 in binary)
        assert_eq!(
            n_bit_adder(&[true, true], &[true, false]),
            (vec![false, false], true)
        );
        
        // 異なる長さの入力: 101 + 11 = 1000 (8 in binary)
        assert_eq!(
            n_bit_adder(&[true, false, true], &[true, true]),
            (vec![false, false, false], true)
        );
    }
    
    #[test]
    fn test_incrementer() {
        // 0 + 1 = 1
        assert_eq!(
            incrementer(&[false]),
            (vec![true], false)
        );
        
        // 1 + 1 = 2 (10 in binary)
        assert_eq!(
            incrementer(&[true]),
            (vec![false], true)
        );
        
        // 10 + 1 = 11 (3 in binary)
        assert_eq!(
            incrementer(&[false, true]),
            (vec![true, true], false)
        );
        
        // 11 + 1 = 100 (4 in binary)
        assert_eq!(
            incrementer(&[true, true]),
            (vec![false, false], true)
        );
        
        // 1111 + 1 = 10000 (16 in binary、オーバーフロー)
        assert_eq!(
            incrementer(&[true, true, true, true]),
            (vec![false, false, false, false], true)
        );
    }
} 