//! Fibonacci Numbers are defined by
//! ```text
//!   F(0) = 1
//!   F(1) = 1
//!   F(n+2) = F(n+1) + F(n)
//! ```
//! This crate contains various functions computing F(N).

/// This function computes `F(N)` recursively.
pub fn recursive(n: u64) -> u64 {
    //    let fibn = if n == 0 || n == 1 {
    //        1
    //    } else {
    //        recursive(n - 1) + recursive(n - 2)
    //    };
    match n {
        0 | 1 => 1,
        n => recursive(n - 1) + recursive(n - 2),
    }
}

#[test]
fn test_recursive() {
    assert_eq!(1, recursive(0));
    assert_eq!(1, recursive(1));
    assert_eq!(2, recursive(2));
    assert_eq!(3, recursive(3));
    assert_eq!(5, recursive(4));
    assert_eq!(317_811, recursive(27));
}

/// This function computes `F(N)` iteratively.
///
/// # Panics
///
/// Panics when `N` greater than 92 due to overflow.
pub fn iterative(n: u64) -> u64 {
    let mut i = 1u64;
    let mut j = 1u64;

    for x in 1..n {
        let tmp = j;
        j = match j.checked_add(i) {
            Some(j) => j,
            None => panic!("fibonacci::iterative: overflowed at {}", x + 1),
        };
        i = tmp;
    }

    j
}

#[test]
fn test_iterative() {
    assert_eq!(1, iterative(0));
    assert_eq!(1, iterative(1));
    assert_eq!(2, iterative(2));
    assert_eq!(3, iterative(3));
    assert_eq!(5, iterative(4));
    assert_eq!(317_811, iterative(27));
    assert_eq!(3_416_454_622_906_707, iterative(75));
    let _ = iterative(92);
}

#[test]
#[should_panic]
fn test_iterative_overflow() {
    let _ = iterative(93);
}
