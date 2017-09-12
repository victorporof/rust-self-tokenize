/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

#![cfg_attr(feature = "cargo-clippy", allow(implicit_hasher, match_ref_pats))]
#![feature(specialization)]

extern crate num_traits;
extern crate ordered_float;
extern crate quote;
extern crate smallvec;

use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;

use num_traits::Float;
use ordered_float::OrderedFloat;
use smallvec::{Array, SmallVec};

pub use quote::{ToTokens, Tokens};

pub trait ToCustomTokens {
    fn to_custom_tokens(&self, &mut Tokens);
}

macro_rules! default_impls {
    ( $( $ty:ty )+ ) => { $(
        impl ToCustomTokens for $ty where Self: ToTokens {
            fn to_custom_tokens(&self, tokens: &mut Tokens) {
                self.to_tokens(tokens);
            }
        }
    )+ };
}

macro_rules! slice_impls {
    ( $( $ty:ty )+ ) => { $(
        impl ToCustomTokens for [$ty] {
            fn to_custom_tokens(&self, tokens: &mut Tokens) {
                tokens.append(format!("{:?}", self));
            }
        }
    )+ }
}

macro_rules! array_impls {
    ( $( [$T:ident; $N:expr] )+ ) => { $(
        impl<$T> ToCustomTokens for [$T; $N]
        where
            T: ToCustomTokens
        {
            fn to_custom_tokens(&self, tokens: &mut Tokens) {
                self[..].to_custom_tokens(tokens);
            }
        }
    )+ }
}

macro_rules! tuple_impls {
    ( $( ( $( $idx:tt: $T:ident ),* ) )+ ) => { $(
        impl<$( $T ),*> ToCustomTokens for ($( $T, )*)
        where
            $( $T: ToCustomTokens ),*
        {
            fn to_custom_tokens(&self, tokens: &mut Tokens) {
                tokens.append("(");
                $(
                    self.$idx.to_custom_tokens(tokens);
                    tokens.append(",");
                )*
                tokens.append(")");
            }
        }
    )+ }
}

default_impls!(
    bool i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 isize usize char str
);

slice_impls!(
    bool i8 u8 i16 u16 i32 u32 i64 u64 isize usize char
);

array_impls! {
    [T; 0]  [T; 1]  [T; 2]  [T; 3]  [T; 4]  [T; 5]  [T; 6]  [T; 7]
    [T; 8]  [T; 9]  [T; 10] [T; 11] [T; 12] [T; 13] [T; 14] [T; 15]
    [T; 16] [T; 17] [T; 18] [T; 19] [T; 20] [T; 21] [T; 22] [T; 23]
    [T; 24] [T; 25] [T; 26] [T; 27] [T; 28] [T; 29] [T; 30] [T; 31]
    [T; 32]
}

tuple_impls! {
    ()
    (0: A)
    (0: A, 1: B)
    (0: A, 1: B, 2: C)
    (0: A, 1: B, 2: C, 3: D)
    (0: A, 1: B, 2: C, 3: D, 4: E)
    (0: A, 1: B, 2: C, 3: D, 4: E, 5: F)
    (0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G)
    (0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H)
    (0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I)
    (0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J)
    (0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K)
    (0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L)
    (0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L, 12: M)
}

impl<'a, T> ToCustomTokens for &'a T
where
    T: ?Sized + ToCustomTokens
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        (*self).to_custom_tokens(tokens);
    }
}

impl ToCustomTokens for String
where
    Self: ToTokens
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        self.to_tokens(tokens);
        tokens.append(".");
        tokens.append("to_string");
        tokens.append("(");
        tokens.append(")");
    }
}

impl<T> ToCustomTokens for Option<T>
where
    T: ToCustomTokens
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &Some(ref value) => {
                tokens.append("Some");
                tokens.append("(");
                value.to_custom_tokens(tokens);
                tokens.append(")");
            }
            &None => {
                tokens.append("None");
            }
        }
    }
}

impl<T> ToCustomTokens for Vec<T>
where
    T: ToCustomTokens
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        tokens.append("vec!");
        self[..].to_custom_tokens(tokens);
    }
}

impl<T> ToCustomTokens for [T]
where
    T: ToCustomTokens
{
    default fn to_custom_tokens(&self, tokens: &mut Tokens) {
        tokens.append("[");
        self.iter().for_each(|item| {
            item.to_custom_tokens(tokens);
            tokens.append(",");
        });
        tokens.append("]");
    }
}

impl<'a, T> ToCustomTokens for Cow<'a, T>
where
    T: ?Sized + ToOwned + ToCustomTokens
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        tokens.append("::std::borrow::Cow");
        tokens.append("::");
        tokens.append("from");
        tokens.append("(");
        self.as_ref().to_custom_tokens(tokens);
        tokens.append(")");
    }
}

impl<T> ToCustomTokens for Box<T>
where
    T: ?Sized + ToCustomTokens
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        tokens.append("box");
        (**self).to_custom_tokens(tokens);
    }
}

impl<T> ToCustomTokens for Rc<T>
where
    T: ?Sized + ToCustomTokens
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        assert_eq!(Rc::strong_count(self), 1, "Unsound operation");
        tokens.append("::std::rc::Rc");
        tokens.append("::");
        tokens.append("new");
        tokens.append("(");
        self.as_ref().to_custom_tokens(tokens);
        tokens.append(")");
    }
}

impl<T> ToCustomTokens for Arc<T>
where
    T: ?Sized + ToCustomTokens
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        assert_eq!(Arc::strong_count(self), 1, "Unsound operation");
        tokens.append("::std::sync::Arc");
        tokens.append("::");
        tokens.append("new");
        tokens.append("(");
        self.as_ref().to_custom_tokens(tokens);
        tokens.append(")");
    }
}

impl<T, U> ToCustomTokens for HashMap<T, U>
where
    T: Eq + Hash + ToCustomTokens,
    U: ToCustomTokens
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        tokens.append("::std::collections::HashMap");
        tokens.append("::");
        tokens.append("from_iter");
        tokens.append("(");
        tokens.append("vec!");
        tokens.append("[");
        self.iter().for_each(|entry| {
            entry.to_custom_tokens(tokens);
            tokens.append(",");
        });
        tokens.append("]");
        tokens.append(")");
    }
}

impl<T, U> ToCustomTokens for BTreeMap<T, U>
where
    T: ToCustomTokens,
    U: ToCustomTokens
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        tokens.append("::std::collections::BTreeMap");
        tokens.append("::");
        tokens.append("from_iter");
        tokens.append("(");
        tokens.append("vec!");
        tokens.append("[");
        self.iter().for_each(|entry| {
            entry.to_custom_tokens(tokens);
            tokens.append(",");
        });
        tokens.append("]");
        tokens.append(")");
    }
}

// Third party

impl<T> ToCustomTokens for OrderedFloat<T>
where
    T: ?Sized + ToCustomTokens + Float
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        self.as_ref().to_custom_tokens(tokens);
        tokens.append(".");
        tokens.append("into");
        tokens.append("(");
        tokens.append(")");
    }
}

impl<T: Array> ToCustomTokens for SmallVec<T>
where
    T::Item: ToCustomTokens
{
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        if self.is_empty() {
            tokens.append("SmallVec");
            tokens.append("::");
            tokens.append("new");
            tokens.append("(");
            tokens.append(")");
        } else if self.inline_size() == self.len() {
            tokens.append("SmallVec");
            tokens.append("::");
            tokens.append("from_buf");
            tokens.append("(");
            self.as_ref().to_custom_tokens(tokens);
            tokens.append(")");
        } else {
            tokens.append("SmallVec");
            tokens.append("::");
            tokens.append("from_vec");
            tokens.append("(");
            tokens.append("vec!");
            self.as_ref().to_custom_tokens(tokens);
            tokens.append(")");
        }
    }
}
