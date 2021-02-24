/*!
    Derive mathematical operators.

    This is a crate written for practice and will not be published.
*/

#![allow(unused)]
#![cfg_attr(doctest, allow(non_camel_case_types))]

use proc_macro2::TokenStream;
use quote::*;
use syn::{spanned::Spanned, *};

#[macro_use]
mod ops;
#[macro_use]
mod assign;

ops! {
    Add add
    Sub sub
    Mul mul
    Div div
    Rem rem
}

assign! {
    AddAssign add_assign
    SubAssign sub_assign
    MulAssign mul_assign
    DivAssign div_assign
    RemAssign rem_assign
}

// derives everything
#[proc_macro_attribute]
pub fn num(_: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    match &input.data {
        Data::Struct(_) => quote!(
            #[derive(Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign)]
            #input
        ).into(),

        _ => quote!(
            compile_error!("#[num] can only be used on structs");
        ).into(),
    }
}

// Rust doesn't support tests that fail to compile, but rustdoc does.

/**```compile_fail
use ops_derive::*;

struct NoAdd;

#[derive(Add)]
struct Foo(NoAdd);
```*/
#[cfg(doctest)] struct op_all_fields_need_trait;

/**```compile_fail
use ops_derive::*;

struct NoAddAssign;

#[derive(AddAssign)]
struct Foo(NoAddAssign);
```*/
#[cfg(doctest)] struct assign_all_fields_need_trait;

/**```compile_fail
use ops_derive::*;

#[derive(Add)]
enum NotAStruct {}
```*/
#[cfg(doctest)] struct op_must_be_struct;

/**```compile_fail
use ops_derive::*;

#[derive(AddAssign)]
enum NotAStruct {}
*/
#[cfg(doctest)] struct assign_must_be_struct;

/**```compile_fail
use ops_derive::*;

#[derive(Add)]
struct Foo<T>(T);

fn main() {
    let one = Foo("heck");
    let two = Foo("crap");

    assert_eq!(one + two, Foo("can't add strings together"));
}
```*/
#[cfg(doctest)] struct op_just_in_case;

/**```compile_fail
use ops_derive::*;

#[derive(AddAssign)]
struct Foo<T>(T);

fn main() {
    let one = Foo("heck");
    one += Foo("crap");

    assert_eq!(one, Foo("can't add strings together"));
}
```*/
#[cfg(doctest)] struct assign_just_in_case;
