#![feature(const_fn)]
#![feature(const_type_id)]
#![feature(specialization)]

use std::any;

pub type TypeId = any::TypeId;

pub enum FieldId {
    Unnamed(usize),
    Named(&'static str),
}

pub trait TypeInfo: DynamicTypeInfo {
    const TYPE: Type;
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

pub struct Type {
    pub id: TypeId,
    pub module: &'static str,
    pub ident: &'static str,
    pub data: Data,
}

pub enum Data {
    Primitive,
    Struct(DataStruct),
    Enum(DataEnum),
    Union(DataUnion),
}

pub struct DataStruct {
    pub fields: Fields,
}

pub struct DataEnum {
    pub variants: &'static [Variant],
}

pub struct DataUnion {
    pub fields: FieldsNamed,
}

pub struct Variant {
    pub ident: &'static str,
    pub fields: Fields,
}

pub enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}

pub struct FieldsNamed {
    pub named: &'static [Field],
}

pub struct FieldsUnnamed {
    pub unnamed: &'static [Field],
}

pub struct Field {
    pub id: FieldId,
    pub ident: Option<&'static str>,
    pub ty: Option<&'static Type>,
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
