#![allow(unused)]
#![allow(non_snake_case)]

use core::ops::*;
use ops_derive::*;
use rand::distributions::{Distribution, Uniform};

#[num]
#[derive(Debug, PartialEq)]
struct Foo(isize);

macro_rules! test {
    ($($op:tt $op_assign:tt $trait:ident $trait_assign:ident $method:ident)*) => {$(
        #[test]
        fn $trait() {
            let mut rng = rand::thread_rng();
            let range = Uniform::new(0, isize::MAX);

            for _ in 0..10_000 {
                let one = range.sample(&mut rng);
                let two = range.sample(&mut rng);
                if let Some(num) = one.$method(two) {
                    assert_eq!(Foo(one) $op Foo(two), Foo(num));
                }
            }
        }

        #[test]
        fn $trait_assign() {
            let mut rng = rand::thread_rng();
            let range = Uniform::new(0, isize::MAX);

            for _ in 0..10_000 {
                let one = range.sample(&mut rng);
                let two = range.sample(&mut rng);

                if let Some(num) = one.$method(two) {
                    let mut foo_obj = Foo(one);
                    foo_obj $op_assign Foo(two);
                    assert_eq!(foo_obj, Foo(one $op two));
                }
            }
        }
    )*}
}

test! {
    + += Add AddAssign checked_add
    - -= Sub SubAssign checked_sub
    * *= Mul MulAssign checked_mul
    / /= Div DivAssign checked_div
    % %= Rem RemAssign checked_rem
}
