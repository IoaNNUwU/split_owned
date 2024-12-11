//! ### This crate provides simple way to split array in 2 owned arrays with compile-time bounds checks.
//! 
//! - Works with `Non-Copy` & `Non-Clone` types
//! 
//! Common usage:
//! ```
//! use split_owned::SplitOwned;
//! 
//! let arr: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
//! 
//! let (arr1, arr2) = arr.split_owned::<3, 4>();
//! 
//! assert_eq!(arr1, [0, 1, 2]);
//! assert_eq!(arr2, [3, 4, 5, 6]);
//! ```
//! 
//! ```
//! use split_owned::SplitOwned;
//! 
//! let arr: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
//! 
//! let arr1: [i32; 3];
//! let arr2: [i32; 4];
//! (arr1, arr2) = arr.split_owned();
//! 
//! assert_eq!(arr1, [0, 1, 2]);
//! assert_eq!(arr2, [3, 4, 5, 6]);
//! ```
//! Does not compile
//! ```compile_fail
//! use split_owned::SplitOwned;
//! 
//! let arr: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
//! 
//! // Compile error: 
//! // Length of original array has to be equal to sum of lengths of resulting arrays N == K + L
//! let (arr1, arr2) = arr.split_owned::<2, 4>();
//! ```
//! `Non-Clone` type
//! ```
//! use split_owned::SplitOwned;
//! 
//! #[derive(Debug, PartialEq)]
//! struct Num(f64);
//! 
//! let arr: [Num; 7] = [Num(0.), Num(1.), Num(2.), Num(3.), Num(4.), Num(5.), Num(6.)];
//! 
//! let (arr1, arr2) = arr.split_owned::<3, 4>();
//! 
//! assert_eq!(arr1, [Num(0.), Num(1.), Num(2.)]);
//! assert_eq!(arr2, [Num(3.), Num(4.), Num(5.), Num(6.)]);
//! ```

use std::mem::MaybeUninit;

/// Extention trait which provides [SplitOwned::split_owned] function.
pub trait SplitOwned<T> {
    fn split_owned<const K: usize, const L: usize>(self) -> ([T; K], [T; L]);
}

impl<T, const N: usize> SplitOwned<T> for [T; N] {
    
    /// Common usage:
    /// ```
    /// use split_owned::SplitOwned;
    /// 
    /// let arr: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
    /// 
    /// let (arr1, arr2) = arr.split_owned::<3, 4>();
    /// 
    /// assert_eq!(arr1, [0, 1, 2]);
    /// assert_eq!(arr2, [3, 4, 5, 6]);
    /// ```
    /// 
    /// ```
    /// use split_owned::SplitOwned;
    /// 
    /// let arr: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
    /// 
    /// let arr1: [i32; 3];
    /// let arr2: [i32; 4];
    /// (arr1, arr2) = arr.split_owned();
    /// 
    /// assert_eq!(arr1, [0, 1, 2]);
    /// assert_eq!(arr2, [3, 4, 5, 6]);
    /// ```

    fn split_owned<const K: usize, const L: usize>(self) -> ([T; K], [T; L]) {
        
        const { assert!(N == K + L, 
            "Length of original array has to be equal to sum of lengths of resulting arrays N == K + L"
        )};

        // Wrap each element of original array in MaybeUninit for ease of use
        let mut arr: [MaybeUninit<T>; N] = self.map(|el| MaybeUninit::new(el));

        let mut arr_k: [MaybeUninit<T>; K] = std::array::from_fn(|_| MaybeUninit::uninit());
        let mut arr_l: [MaybeUninit<T>; L] = std::array::from_fn(|_| MaybeUninit::uninit());

        for i in 0..K {
            std::mem::swap(&mut arr_k[i], &mut arr[i]);
        }
        for i in 0..L {
            std::mem::swap(&mut arr_l[i], &mut arr[i + K]);
        }

        // SAFETY: Both arrays are initialized with elements from initial array
        let arr_k: [T; K] = arr_k.map(|el: MaybeUninit<T> | unsafe { el.assume_init() });
        let arr_l: [T; L] = arr_l.map(|el: MaybeUninit<T> | unsafe { el.assume_init() });

        (arr_k, arr_l)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn split_easy() {

        let arr: [f64; 19] = std::array::from_fn(|n| n as f64);

        let (arr1, arr2) = arr.split_owned::<10, 9>();

        assert_eq!(arr1, [0., 1., 2., 3., 4., 5., 6., 7., 8., 9.]);
        assert_eq!(arr2, [10., 11., 12., 13., 14., 15., 16., 17., 18.]);
    }

    #[test]
    fn split_zero() {
        let arr: [f64; 6] = std::array::from_fn(|n| n as f64);

        let (arr1, arr2) = arr.split_owned::<0, 6>();

        assert_eq!(arr1, []);
        assert_eq!(arr2, [0., 1., 2., 3., 4., 5.]);
        
        let (arr1, arr2) = arr2.split_owned::<6, 0>();
        
        assert_eq!(arr1, [0., 1., 2., 3., 4., 5.]);
        assert_eq!(arr2, []);
    }
    
    #[test]
    fn split_ref() {
        let arr: [f64; 6] = std::array::from_fn(|n| n as f64);

        let refs: [&f64; 6] = [&arr[0], &arr[1], &arr[2], &arr[3], &arr[4], &arr[4]];

        let (arr1, _) = refs.split_owned::<1, 5>();

        assert_eq!(*arr1[0], 0.);
    }
}
