#![feature(const_fn)]
#![feature(const_type_id)]
#![feature(specialization)]

use std::any;
use std::fmt;

pub type TypeId = any::TypeId;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum FieldId<'a> {
    Unnamed(usize),
    Named(&'a str),
}

pub trait TypeInfo: DynamicTypeInfo {
    const TYPE: Type;

    fn field<A>(&self, id: FieldId) -> Option<&A>
    where
        A: any::Any,
    {
        panic!("no such field id: {}", id)
    }

    fn field_mut<A>(&mut self, id: FieldId) -> Option<&mut A>
    where
        A: any::Any,
    {
        panic!("no such field id: {}", id)
    }
}

pub trait DynamicTypeInfo {
    fn type_ref(&self) -> &'static Type;
}

pub trait TryTypeInfo {
    const TRY_TYPE: Option<&'static Type>;
}

impl<T> TryTypeInfo for T {
    default const TRY_TYPE: Option<&'static Type> = None;
}

impl<T> TryTypeInfo for T
where
    T: TypeInfo,
{
    const TRY_TYPE: Option<&'static Type> = Some(&T::TYPE);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Type {
    pub id: TypeId,
    pub module: &'static str,
    pub ident: &'static str,
    pub data: Data,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Data {
    Primitive,
    Struct(DataStruct),
    Enum(DataEnum),
    Union(DataUnion),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataStruct {
    pub fields: Fields,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataEnum {
    pub variants: &'static [Variant],
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataUnion {
    pub fields: FieldsNamed,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variant {
    pub ident: &'static str,
    pub fields: Fields,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FieldsNamed {
    pub named: &'static [Field],
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FieldsUnnamed {
    pub unnamed: &'static [Field],
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Field {
    pub id: FieldId<'static>,
    pub ident: Option<&'static str>,
    pub ty: Option<&'static Type>,
}

impl<'a> fmt::Display for FieldId<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldId::Unnamed(idx) => idx.fmt(f),
            FieldId::Named(name) => name.fmt(f),
        }
    }
}

macro_rules! impl_primitive {
    ($t:ty) => {
        impl TypeInfo for $t {
            const TYPE: Type = Type {
                id: TypeId::of::<$t>(),
                module: "",
                ident: stringify!($t),
                data: Data::Primitive,
            };
        }

        impl DynamicTypeInfo for $t {
            fn type_ref(&self) -> &'static Type {
                &<Self as TypeInfo>::TYPE
            }
        }
    };
}

impl_primitive!(u8);
impl_primitive!(u16);
impl_primitive!(u32);
impl_primitive!(u64);
impl_primitive!(usize);

impl_primitive!(i8);
impl_primitive!(i16);
impl_primitive!(i32);
impl_primitive!(i64);
impl_primitive!(isize);

impl_primitive!(f32);
impl_primitive!(f64);

impl_primitive!(bool);

impl_primitive!(char);
