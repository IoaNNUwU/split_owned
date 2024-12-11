### This crate provides simple way to split array in 2 owned arrays with compile-time bounds checks.

- Works with `Non-Copy` & `Non-Clone` types

Common usage:
```rust
use split_owned::SplitOwned;

let arr: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];

let (arr1, arr2) = arr.split_owned::<3, 4>();

assert_eq!(arr1, [0, 1, 2]);
assert_eq!(arr2, [3, 4, 5, 6]);
```

```rust
use split_owned::SplitOwned;

let arr: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];

let arr1: [i32; 3];
let arr2: [i32; 4];
(arr1, arr2) = arr.split_owned();

assert_eq!(arr1, [0, 1, 2]);
assert_eq!(arr2, [3, 4, 5, 6]);
```
Does not compile
```rust
use split_owned::SplitOwned;

let arr: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];

// Compile error: 
// Length of original array has to be equal to sum of lengths of resulting arrays N == K + L
let (arr1, arr2) = arr.split_owned::<2, 4>();
```
`Non-Clone` type
```rust
use split_owned::SplitOwned;

#[derive(Debug, PartialEq)]
struct Num(f64);

let arr: [Num; 7] = [Num(0.), Num(1.), Num(2.), Num(3.), Num(4.), Num(5.), Num(6.)];

let (arr1, arr2) = arr.split_owned::<3, 4>();

assert_eq!(arr1, [Num(0.), Num(1.), Num(2.)]);
assert_eq!(arr2, [Num(3.), Num(4.), Num(5.), Num(6.)]);
```