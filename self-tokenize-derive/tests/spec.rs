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

#![cfg_attr(feature = "cargo-clippy", allow(blacklisted_name))]
#![feature(box_syntax)]
#![feature(proc_macro)]

extern crate num_traits;
extern crate ordered_float;
#[macro_use]
extern crate quote;
extern crate self_tokenize_macro;
extern crate self_tokenize_trait;
extern crate smallvec;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::rc::Rc;
use std::sync::Arc;

use ordered_float::OrderedFloat;
use quote::ToTokens;
use smallvec::SmallVec;

use self_tokenize_macro::SelfTokenize;
use self_tokenize_trait::ToCustomTokens;

#[derive(SelfTokenize)]
struct MyExampleUnit;

#[derive(SelfTokenize)]
struct MyExampleStruct {
    foo: String,
    bar: MyExampleTupleStruct,
    baz: Vec<MyExampleEnum>
}

#[derive(SelfTokenize)]
struct MyExampleTupleStruct(String);

#[derive(SelfTokenize)]
enum MyExampleEnum {
    FooVariant(MyExampleUnit),
    BarVariant { unit: MyExampleUnit }
}

#[derive(SelfTokenize)]
struct MyUnitStruct;

#[derive(SelfTokenize, Eq, PartialEq, Ord, PartialOrd)]
struct MyTupleStructA(i32);

#[derive(SelfTokenize)]
struct MyTupleStructB<'a>(&'a str);

#[derive(SelfTokenize)]
struct MyTupleStructC(String);

#[derive(SelfTokenize)]
struct MyTupleStructD<'a>(
    bool,
    i32,
    f64,
    &'a str,
    String,
    Vec<MyTupleStructA>,
    BTreeMap<MyTupleStructA, (MyTupleStructB<'a>, MyTupleStructC)>
);

#[derive(SelfTokenize)]
struct MyTupleStructE<'a>(
    bool,
    i32,
    f64,
    &'a str,
    String,
    Vec<MyStructA>,
    BTreeMap<MyStructA, (MyStructB<'a>, MyStructC)>
);

#[derive(SelfTokenize, Eq, PartialEq, Ord, PartialOrd)]
struct MyStructA {
    foo: i32
}

#[derive(SelfTokenize)]
struct MyStructB<'a> {
    foo: &'a str
}

#[derive(SelfTokenize)]
struct MyStructC {
    foo: String
}

#[derive(SelfTokenize)]
struct MyStructD<'a> {
    a: bool,
    b: i32,
    c: f64,
    d: &'a str,
    e: String,
    f: Vec<MyStructA>,
    g: BTreeMap<MyStructA, (MyStructB<'a>, MyStructC)>
}

#[derive(SelfTokenize)]
struct MyStructE<'a> {
    a: bool,
    b: i32,
    c: f64,
    d: &'a str,
    e: String,
    f: Vec<MyTupleStructA>,
    g: BTreeMap<MyTupleStructA, (MyTupleStructB<'a>, MyTupleStructC)>
}

#[derive(SelfTokenize, Eq, PartialEq, Ord, PartialOrd)]
enum MyEnumA {
    Foo(i32)
}

#[derive(SelfTokenize, Eq, PartialEq, Ord, PartialOrd)]
enum MyEnumNamedA {
    Foo { foo: i32 }
}

#[derive(SelfTokenize)]
enum MyEnumB<'a> {
    Foo(&'a str)
}

#[derive(SelfTokenize)]
enum MyEnumNamedB<'a> {
    Foo { foo: &'a str }
}

#[derive(SelfTokenize)]
enum MyEnumC {
    Foo(String)
}

#[derive(SelfTokenize)]
enum MyEnumNamedC {
    Foo { foo: String }
}

#[derive(SelfTokenize)]
enum MyEnumD<'a> {
    A(bool),
    B(i32),
    C(f64),
    D(&'a str),
    E(String),
    F(Vec<MyEnumA>),
    G(BTreeMap<MyEnumA, (MyEnumB<'a>, MyEnumC)>)
}

#[derive(SelfTokenize)]
enum MyEnumNamedD<'a> {
    A {
        foo: bool
    },
    B {
        foo: i32
    },
    C {
        foo: f64
    },
    D {
        foo: &'a str
    },
    E {
        foo: String
    },
    F {
        foo: Vec<MyEnumNamedA>
    },
    G {
        foo: BTreeMap<MyEnumNamedA, (MyEnumNamedB<'a>, MyEnumNamedC)>
    }
}

#[derive(SelfTokenize)]
struct MyOrderedFloat(OrderedFloat<f32>);

#[derive(SelfTokenize)]
struct MyCow<'a>(Cow<'a, str>);

#[derive(SelfTokenize)]
struct MyBox(Box<u8>);

#[derive(SelfTokenize)]
struct MyBoxedSlice(Box<[u8]>);

#[derive(SelfTokenize)]
#[allow(box_vec)]
struct MyBoxedVec(Box<Vec<u8>>);

#[derive(SelfTokenize)]
struct MyOptionStr<'a>(Option<&'a str>);

#[derive(SelfTokenize)]
struct MyOptionString(Option<String>);

#[derive(SelfTokenize)]
struct MyOptionCowStr<'a>(Option<Cow<'a, str>>);

#[derive(SelfTokenize)]
struct MyOptionCowNamedLifetime<'foo>(Option<Cow<'foo, str>>);

#[derive(SelfTokenize)]
struct MySmallVec(SmallVec<[u8; 4]>);

#[derive(SelfTokenize)]
struct MyRc(Rc<u8>);

#[derive(SelfTokenize)]
struct MyArc(Arc<u8>);

#[derive(SelfTokenize)]
struct MyRcSmallVec(Rc<SmallVec<[u8; 4]>>);

#[derive(SelfTokenize)]
struct MyArcSmallVec(Arc<SmallVec<[u8; 4]>>);

#[test]
fn test_example_1() {
    let value = MyExampleStruct {
        foo: "Hello".to_string(),
        bar: MyExampleTupleStruct("world!".to_string()),
        baz: vec![
            MyExampleEnum::FooVariant(MyExampleUnit),
            MyExampleEnum::BarVariant {
                unit: MyExampleUnit
            },
        ]
    };
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyExampleStruct { foo : \"Hello\" . to_string ( ) , bar : MyExampleTupleStruct ( \"world!\" . to_string ( ) ) , baz : vec! [ \
         MyExampleEnum :: FooVariant ( MyExampleUnit ) , MyExampleEnum :: BarVariant { unit : MyExampleUnit } , ] }"
    );
}

#[test]
fn test_example_2() {
    let value = MyExampleStruct {
        foo: "Hello".to_string(),
        bar: MyExampleTupleStruct("world!".to_string()),
        baz: vec![
            MyExampleEnum::FooVariant(MyExampleUnit),
            MyExampleEnum::BarVariant {
                unit: MyExampleUnit
            },
        ]
    };
    let mut t = quote::Tokens::new();
    value.to_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyExampleStruct { foo : \"Hello\" . to_string ( ) , bar : MyExampleTupleStruct ( \"world!\" . to_string ( ) ) , baz : vec! [ \
         MyExampleEnum :: FooVariant ( MyExampleUnit ) , MyExampleEnum :: BarVariant { unit : MyExampleUnit } , ] }"
    );
}

#[test]
fn test_example_3() {
    let value = MyExampleStruct {
        foo: "Hello".to_string(),
        bar: MyExampleTupleStruct("world!".to_string()),
        baz: vec![
            MyExampleEnum::FooVariant(MyExampleUnit),
            MyExampleEnum::BarVariant {
                unit: MyExampleUnit
            },
        ]
    };

    assert_eq!(
        quote! { #value }.to_string(),
        "MyExampleStruct { foo : \"Hello\" . to_string ( ) , bar : MyExampleTupleStruct ( \"world!\" . to_string ( ) ) , baz : vec! [ \
         MyExampleEnum :: FooVariant ( MyExampleUnit ) , MyExampleEnum :: BarVariant { unit : MyExampleUnit } , ] }"
    );
}

#[test]
fn test_derive_unit_structs() {
    let value = MyUnitStruct;
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyUnitStruct");
}

#[test]
fn test_derive_tuple_structs_a() {
    let value = MyTupleStructA(42);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyTupleStructA ( 42i32 )");
}

#[test]
fn test_derive_tuple_structs_b() {
    let foo = "foo".to_string();
    let value = MyTupleStructB(&foo);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyTupleStructB ( \"foo\" )");
}

#[test]
fn test_derive_tuple_structs_c() {
    let foo = "foo".to_string();
    let value = MyTupleStructC(foo);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyTupleStructC ( \"foo\" . to_string ( ) )");
}

#[test]
fn test_derive_tuple_structs_d() {
    let foo = "foo".to_string();
    let value = MyTupleStructD(
        true,
        42,
        43.0e-1,
        &foo,
        "bar".to_string(),
        vec![MyTupleStructA(1), MyTupleStructA(2), MyTupleStructA(3)],
        BTreeMap::from_iter(vec![
            (
                MyTupleStructA(42),
                (MyTupleStructB("hello"), MyTupleStructC("world".to_string()))
            ),
            (
                MyTupleStructA(43),
                (MyTupleStructB("hello"), MyTupleStructC("world".to_string()))
            ),
        ])
    );
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyTupleStructD ( true , 42i32 , 4.3f64 , \"foo\" , \"bar\" . to_string ( ) , vec! [ MyTupleStructA ( 1i32 ) , MyTupleStructA ( \
         2i32 ) , MyTupleStructA ( 3i32 ) , ] , ::std::collections::BTreeMap :: from_iter ( vec! [ ( MyTupleStructA ( 42i32 ) , ( \
         MyTupleStructB ( \"hello\" ) , MyTupleStructC ( \"world\" . to_string ( ) ) , ) , ) , ( MyTupleStructA ( 43i32 ) , ( \
         MyTupleStructB ( \"hello\" ) , MyTupleStructC ( \"world\" . to_string ( ) ) , ) , ) , ] ) )"
    );
}

#[test]
fn test_derive_tuple_structs_e() {
    let foo = "foo".to_string();
    let value = MyTupleStructE(
        true,
        42,
        43.0e-1,
        &foo,
        "bar".to_string(),
        vec![
            MyStructA { foo: 1 },
            MyStructA { foo: 2 },
            MyStructA { foo: 3 },
        ],
        BTreeMap::from_iter(vec![
            (
                MyStructA { foo: 42 },
                (
                    MyStructB { foo: "hello" },
                    MyStructC {
                        foo: "world".to_string()
                    }
                )
            ),
            (
                MyStructA { foo: 43 },
                (
                    MyStructB { foo: "hello" },
                    MyStructC {
                        foo: "world".to_string()
                    }
                )
            ),
        ])
    );
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyTupleStructE ( true , 42i32 , 4.3f64 , \"foo\" , \"bar\" . to_string ( ) , vec! [ MyStructA { foo : 1i32 } , MyStructA { foo : \
         2i32 } , MyStructA { foo : 3i32 } , ] , ::std::collections::BTreeMap :: from_iter ( vec! [ ( MyStructA { foo : 42i32 } , ( \
         MyStructB { foo : \"hello\" } , MyStructC { foo : \"world\" . to_string ( ) } , ) , ) , ( MyStructA { foo : 43i32 } , ( \
         MyStructB { foo : \"hello\" } , MyStructC { foo : \"world\" . to_string ( ) } , ) , ) , ] ) )"
    );
}

#[test]
fn test_derive_structs_a() {
    let value = MyStructA { foo: 42 };
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyStructA { foo : 42i32 }");
}

#[test]
fn test_derive_structs_b() {
    let foo = "foo".to_string();
    let value = MyStructB { foo: &foo };
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyStructB { foo : \"foo\" }");
}

#[test]
fn test_derive_structs_c() {
    let foo = "foo".to_string();
    let value = MyStructC { foo };
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyStructC { foo : \"foo\" . to_string ( ) }");
}

#[test]
fn test_derive_structs_d() {
    let foo = "foo".to_string();
    let value = MyStructD {
        a: true,
        b: 42,
        c: 43.0e-1,
        d: &foo,
        e: "bar".to_string(),
        f: vec![
            MyStructA { foo: 1 },
            MyStructA { foo: 2 },
            MyStructA { foo: 3 },
        ],
        g: BTreeMap::from_iter(vec![
            (
                MyStructA { foo: 42 },
                (
                    MyStructB { foo: "hello" },
                    MyStructC {
                        foo: "world".to_string()
                    }
                )
            ),
            (
                MyStructA { foo: 43 },
                (
                    MyStructB { foo: "hello" },
                    MyStructC {
                        foo: "world".to_string()
                    }
                )
            ),
        ])
    };
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyStructD { a : true , b : 42i32 , c : 4.3f64 , d : \"foo\" , e : \"bar\" . to_string ( ) , f : vec! [ MyStructA { foo : 1i32 } \
         , MyStructA { foo : 2i32 } , MyStructA { foo : 3i32 } , ] , g : ::std::collections::BTreeMap :: from_iter ( vec! [ ( MyStructA { \
         foo : 42i32 } , ( MyStructB { foo : \"hello\" } , MyStructC { foo : \"world\" . to_string ( ) } , ) , ) , ( MyStructA { foo : \
         43i32 } , ( MyStructB { foo : \"hello\" } , MyStructC { foo : \"world\" . to_string ( ) } , ) , ) , ] ) }"
    );
}

#[test]
fn test_derive_structs_e() {
    let foo = "foo".to_string();
    let value = MyStructE {
        a: true,
        b: 42,
        c: 43.0e-1,
        d: &foo,
        e: "bar".to_string(),
        f: vec![MyTupleStructA(1), MyTupleStructA(2), MyTupleStructA(3)],
        g: BTreeMap::from_iter(vec![
            (
                MyTupleStructA(42),
                (MyTupleStructB("hello"), MyTupleStructC("world".to_string()))
            ),
            (
                MyTupleStructA(43),
                (MyTupleStructB("hello"), MyTupleStructC("world".to_string()))
            ),
        ])
    };
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyStructE { a : true , b : 42i32 , c : 4.3f64 , d : \"foo\" , e : \"bar\" . to_string ( ) , f : vec! [ MyTupleStructA ( 1i32 ) , \
         MyTupleStructA ( 2i32 ) , MyTupleStructA ( 3i32 ) , ] , g : ::std::collections::BTreeMap :: from_iter ( vec! [ ( MyTupleStructA \
         ( 42i32 ) , ( MyTupleStructB ( \"hello\" ) , MyTupleStructC ( \"world\" . to_string ( ) ) , ) , ) , ( MyTupleStructA ( 43i32 ) , \
         ( MyTupleStructB ( \"hello\" ) , MyTupleStructC ( \"world\" . to_string ( ) ) , ) , ) , ] ) }"
    );
}

#[test]
fn test_derive_enums_a() {
    let value = MyEnumA::Foo(42);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyEnumA :: Foo ( 42i32 )");
}

#[test]
fn test_derive_enums_named_a() {
    let value = MyEnumNamedA::Foo { foo: 42 };
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyEnumNamedA :: Foo { foo : 42i32 }");
}

#[test]
fn test_derive_enums_b() {
    let foo = "foo".to_string();
    let value = MyEnumB::Foo(&foo);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyEnumB :: Foo ( \"foo\" )");
}

#[test]
fn test_derive_enums_named_b() {
    let foo = "foo".to_string();
    let value = MyEnumNamedB::Foo { foo: &foo };
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyEnumNamedB :: Foo { foo : \"foo\" }");
}

#[test]
fn test_derive_enums_c() {
    let foo = "foo".to_string();
    let value = MyEnumC::Foo(foo);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyEnumC :: Foo ( \"foo\" . to_string ( ) )");
}

#[test]
fn test_derive_enums_named_c() {
    let foo = "foo".to_string();
    let value = MyEnumNamedC::Foo { foo };
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyEnumNamedC :: Foo { foo : \"foo\" . to_string ( ) }"
    );
}

#[test]
fn test_derive_enums_d() {
    let foo = "foo".to_string();
    let value = vec![
        MyEnumD::A(true),
        MyEnumD::B(42),
        MyEnumD::C(43.0e-1),
        MyEnumD::D(&foo),
        MyEnumD::E("bar".to_string()),
        MyEnumD::F(vec![MyEnumA::Foo(1), MyEnumA::Foo(2), MyEnumA::Foo(3)]),
        MyEnumD::G(BTreeMap::from_iter(vec![
            (
                MyEnumA::Foo(42),
                (MyEnumB::Foo("hello"), MyEnumC::Foo("world".to_string()))
            ),
            (
                MyEnumA::Foo(43),
                (MyEnumB::Foo("hello"), MyEnumC::Foo("world".to_string()))
            ),
        ])),
    ];
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "vec! [ MyEnumD :: A ( true ) , MyEnumD :: B ( 42i32 ) , MyEnumD :: C ( 4.3f64 ) , MyEnumD :: D ( \"foo\" ) , MyEnumD :: E ( \
         \"bar\" . to_string ( ) ) , MyEnumD :: F ( vec! [ MyEnumA :: Foo ( 1i32 ) , MyEnumA :: Foo ( 2i32 ) , MyEnumA :: Foo ( 3i32 ) , \
         ] ) , MyEnumD :: G ( ::std::collections::BTreeMap :: from_iter ( vec! [ ( MyEnumA :: Foo ( 42i32 ) , ( MyEnumB :: Foo ( \
         \"hello\" ) , MyEnumC :: Foo ( \"world\" . to_string ( ) ) , ) , ) , ( MyEnumA :: Foo ( 43i32 ) , ( MyEnumB :: Foo ( \"hello\" ) \
         , MyEnumC :: Foo ( \"world\" . to_string ( ) ) , ) , ) , ] ) ) , ]"
    );
}

#[test]
fn test_derive_enums_named_d() {
    let foo = "foo".to_string();
    let value = vec![
        MyEnumNamedD::A { foo: true },
        MyEnumNamedD::B { foo: 42 },
        MyEnumNamedD::C { foo: 43.0e-1 },
        MyEnumNamedD::D { foo: &foo },
        MyEnumNamedD::E {
            foo: "bar".to_string()
        },
        MyEnumNamedD::F {
            foo: vec![
                MyEnumNamedA::Foo { foo: 1 },
                MyEnumNamedA::Foo { foo: 2 },
                MyEnumNamedA::Foo { foo: 3 },
            ]
        },
        MyEnumNamedD::G {
            foo: BTreeMap::from_iter(vec![
                (
                    MyEnumNamedA::Foo { foo: 42 },
                    (
                        MyEnumNamedB::Foo { foo: "hello" },
                        MyEnumNamedC::Foo {
                            foo: "world".to_string()
                        }
                    )
                ),
                (
                    MyEnumNamedA::Foo { foo: 43 },
                    (
                        MyEnumNamedB::Foo { foo: "hello" },
                        MyEnumNamedC::Foo {
                            foo: "world".to_string()
                        }
                    )
                ),
            ])
        },
    ];
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "vec! [ MyEnumNamedD :: A { foo : true } , MyEnumNamedD :: B { foo : 42i32 } , MyEnumNamedD :: C { foo : 4.3f64 } , MyEnumNamedD \
         :: D { foo : \"foo\" } , MyEnumNamedD :: E { foo : \"bar\" . to_string ( ) } , MyEnumNamedD :: F { foo : vec! [ MyEnumNamedA :: \
         Foo { foo : 1i32 } , MyEnumNamedA :: Foo { foo : 2i32 } , MyEnumNamedA :: Foo { foo : 3i32 } , ] } , MyEnumNamedD :: G { foo : \
         ::std::collections::BTreeMap :: from_iter ( vec! [ ( MyEnumNamedA :: Foo { foo : 42i32 } , ( MyEnumNamedB :: Foo { foo : \
         \"hello\" } , MyEnumNamedC :: Foo { foo : \"world\" . to_string ( ) } , ) , ) , ( MyEnumNamedA :: Foo { foo : 43i32 } , ( \
         MyEnumNamedB :: Foo { foo : \"hello\" } , MyEnumNamedC :: Foo { foo : \"world\" . to_string ( ) } , ) , ) , ] ) } , ]"
    );
}

#[test]
fn test_ordered_float() {
    let value = MyOrderedFloat(OrderedFloat(42.0));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyOrderedFloat ( 42f32 . into ( ) )");
}

#[test]
fn test_cow() {
    let value = MyCow(Cow::from("test"));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyCow ( ::std::borrow::Cow :: from ( \"test\" ) )"
    );
}

#[test]
fn test_box_1() {
    let value = MyBox(Box::new(1u8));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyBox ( box 1u8 )");
}

#[test]
fn test_box_2() {
    let value = MyBox(box 1u8);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyBox ( box 1u8 )");
}

#[test]
fn test_boxed_slice_1() {
    let value = MyBoxedSlice(vec![1u8].into_boxed_slice());
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyBoxedSlice ( box [1] )");
}

#[test]
fn test_boxed_slice_2() {
    let value = MyBoxedSlice(box [1u8]);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyBoxedSlice ( box [1] )");
}

#[test]
fn test_boxed_vec_1() {
    let value = MyBoxedVec(Box::new(vec![1u8]));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyBoxedVec ( box vec! [1] )");
}

#[test]
fn test_boxed_vec_2() {
    let value = MyBoxedVec(box vec![1u8]);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyBoxedVec ( box vec! [1] )");
}

#[test]
fn test_option_str() {
    let value = MyOptionStr(Some("test"));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyOptionStr ( Some ( \"test\" ) )");
}

#[test]
fn test_option_str_none() {
    let value = MyOptionStr(None);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyOptionStr ( None )");
}

#[test]
fn test_option_string() {
    let value = MyOptionString(Some("test".to_string()));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyOptionString ( Some ( \"test\" . to_string ( ) ) )"
    );
}

#[test]
fn test_option_string_none() {
    let value = MyOptionString(None);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyOptionString ( None )");
}

#[test]
fn test_option_cow_str() {
    let value = MyOptionCowStr(Some(Cow::from("test")));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyOptionCowStr ( Some ( ::std::borrow::Cow :: from ( \"test\" ) ) )"
    );
}

#[test]
fn test_option_cow_str_none() {
    let value = MyOptionCowStr(None);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyOptionCowStr ( None )");
}

#[test]
fn test_option_cow_str_named_lifetime() {
    let value = MyOptionCowNamedLifetime(Some(Cow::from("test")));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyOptionCowNamedLifetime ( Some ( ::std::borrow::Cow :: from ( \"test\" ) ) )"
    );
}

#[test]
fn test_option_cow_str_named_lifetime_none() {
    let value = MyOptionCowNamedLifetime(None);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyOptionCowNamedLifetime ( None )");
}

#[test]
fn test_smallvec_empty() {
    let value = MySmallVec(SmallVec::from_vec(vec![]));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MySmallVec ( SmallVec :: new ( ) )");
}

#[test]
fn test_smallvec_1() {
    let value = MySmallVec(SmallVec::from_vec(vec![1, 2, 3, 4]));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MySmallVec ( SmallVec :: from_buf ( [1, 2, 3, 4] ) )"
    );
}

#[test]
fn test_smallvec_2() {
    let value = MySmallVec(SmallVec::from_vec(vec![1]));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MySmallVec ( SmallVec :: from_vec ( vec! [1] ) )"
    );
}

#[test]
fn test_smallvec_3() {
    let value = MySmallVec(SmallVec::from_vec(vec![1, 2, 3, 4, 5]));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MySmallVec ( SmallVec :: from_vec ( vec! [1, 2, 3, 4, 5] ) )"
    );
}

#[test]
fn test_rc_1() {
    let value = MyRc(Rc::new(1));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyRc ( ::std::rc::Rc :: new ( 1u8 ) )");
}

#[test]
#[should_panic]
fn test_rc_2() {
    let src = Rc::new(1);
    let value = MyRc(Rc::clone(&src));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);
}

#[test]
fn test_arc_1() {
    let value = MyArc(Arc::new(1));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(t.to_string(), "MyArc ( ::std::sync::Arc :: new ( 1u8 ) )");
}

#[test]
#[should_panic]
fn test_arc_2() {
    let src = Arc::new(1);
    let value = MyArc(Arc::clone(&src));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);
}

#[test]
fn test_arc_3() {
    let value = Arc::new(include_bytes!("fixtures/Quantum.png").to_vec());
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);
}

#[test]
fn test_arc_4() {
    let value = Arc::new(include_bytes!("fixtures/FreeSans.ttf").to_vec());
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);
}

#[test]
fn test_arc_5() {
    let value = Arc::new(vec![0u8; 10]);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "::std::sync::Arc :: new ( vec! [0, 0, 0, 0, 0, 0, 0, 0, 0, 0] )"
    );
}

#[test]
fn test_arc_6() {
    let value = Arc::new(vec![0u8; 100_000_000]);
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);
}

#[test]
fn test_rc_smallvec() {
    let value = MyRcSmallVec(Rc::new(SmallVec::from_buf([1, 2, 3, 4])));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyRcSmallVec ( ::std::rc::Rc :: new ( SmallVec :: from_buf ( [1, 2, 3, 4] ) ) )"
    );
}

#[test]
fn test_arc_smallvec() {
    let value = MyArcSmallVec(Arc::new(SmallVec::from_buf([1, 2, 3, 4])));
    let mut t = quote::Tokens::new();
    value.to_custom_tokens(&mut t);

    assert_eq!(
        t.to_string(),
        "MyArcSmallVec ( ::std::sync::Arc :: new ( SmallVec :: from_buf ( [1, 2, 3, 4] ) ) )"
    );
}
