//! An implementation of detailed type information and reflection.
//!
//! This library provides simple access to type information at runtime, as well as the ability to
//! manipulate data whose type is not statically known.
//!
//! # Type information
//!
//! By deriving the `TypeInfo` trait, you get access to type information about the type at runtime.
//!
//! ```
//! #![feature(const_type_id)]
//!
//! extern crate type_info;
//! #[macro_use]
//! extern crate type_info_derive;
//!
//! use type_info::TypeInfo;
//!
//! #[derive(TypeInfo)]
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//!
//! fn main() {
//!     let ty = Person::TYPE;
//!
//!     assert_eq!("Person", ty.ident);
//!     assert_eq!(vec!["name", "age"], ty.fields().iter().map(|f| f.ident.unwrap()).collect::<Vec<_>>());
//! }
//! ```
//!
//! # Static dispatch
//!
//! This example shows how to use the main `TypeInfo` trait with generic parameters (i.e. not trait
//! objects):
//!
//! ```
//! # #![feature(const_type_id)]
//! #
//! # extern crate type_info;
//! # #[macro_use]
//! # extern crate type_info_derive;
//! #
//! # #[derive(TypeInfo)]
//! # struct Person {
//! #     name: String,
//! #     age: u32,
//! # }
//! // Person is defined like in the above example...
//!
//! // A function that can take any type that has a field called "name" of type String.
//! fn add_smith_to_name<A>(anything: &mut A) where A: type_info::TypeInfo {
//!     let name = anything.field_mut::<String>(type_info::FieldId::Named("name")).unwrap();
//!     name.push_str(" Smith");
//! }
//!
//! fn main() {
//!     let mut person = Person {
//!         name: "Lisa".to_owned(),
//!         age: 23,
//!     };
//!
//!     add_smith_to_name(&mut person);
//!
//!     assert_eq!("Lisa Smith", person.name.as_str());
//! }
//! ```
//!
//! # Dynamic dispatch
//!
//! This example shows how to use the `DynamicTypeInfo` trait when you don't want to introduce a
//! type parameter to specialize a function, but instead prefer to use a trait object:
//!
//! ```
//! # #![feature(const_type_id)]
//! #
//! # extern crate type_info;
//! # #[macro_use]
//! # extern crate type_info_derive;
//! #
//! # #[derive(TypeInfo)]
//! # struct Person {
//! #     name: String,
//! #     age: u32,
//! # }
//! // Person is defined like in the above example...
//!
//! // A function that can take any type that has a field called "name" of type String.
//! fn add_smith_to_name(anything: &mut type_info::DynamicTypeInfo) {
//!     let field = anything.field_any_mut(type_info::FieldId::Named("name")).unwrap();
//!     let name = field.downcast_mut::<String>().unwrap();
//!     name.push_str(" Smith");
//! }
//!
//! fn main() {
//!     let mut person = Person {
//!         name: "Lisa".to_owned(),
//!         age: 23,
//!     };
//!
//!     add_smith_to_name(&mut person);
//!
//!     assert_eq!("Lisa Smith", person.name.as_str());
//! }
//! ```
#![feature(const_type_id)]
#![feature(specialization)]
#![deny(
    missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unsafe_code, unused_import_braces, unused_qualifications
)]

use std::any;
use std::fmt;

/// A globally unique identifier for a type.
pub type TypeId = any::TypeId;

/// A locally unique identifier for a field within a certain type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum FieldId<'a> {
    /// An unnamed field with the specified index in a tuple-like struct; e.g. the `0` in `foo.0`.
    Unnamed(usize),
    /// A named field in a struct, e.g. the `name` in `foo.name`.
    Named(&'a str),
}

/// A type that has compile-time static type information associated with it.
pub trait TypeInfo: DynamicTypeInfo {
    /// The constant statically known type information for this type.
    const TYPE: Type;

    /// Get a reference to the value of a field on this type with the given field id.
    ///
    /// This method will return the current value of the given field if possible, or `None` if the
    /// given field does not exist or does not have a type matching the supplied type.
    fn field<A>(&self, _id: FieldId) -> Option<&A>
    where
        A: any::Any,
    {
        None
    }

    /// Get a mutable reference to the value of a field on this type with the given field id.
    ///
    /// This method will return the current value of the given field if possible, or `None` if the
    /// given field does not exist or does not have a type matching the supplied type.
    fn field_mut<A>(&mut self, _id: FieldId) -> Option<&mut A>
    where
        A: any::Any,
    {
        None
    }
}

/// A type that has compile-time dynamic type information associated with it.
///
/// This trait is built to be compatible with being a trait object.
pub trait DynamicTypeInfo {
    /// The dynamic statically known type information for this type.
    fn type_ref(&self) -> &'static Type;

    /// Get the id of the currently active variant of this type, or `None` if the type is not
    /// an `enum`.
    fn variant(&self) -> Option<&str> {
        None
    }

    /// Get a dynamic reference to the value of a field on this type with the given field id.
    ///
    /// This method will return the current value of the given field if possible, or `None` if the
    /// given field does not exist or does not have a type matching the supplied type.
    fn field_any(&self, _id: FieldId) -> Option<&any::Any> {
        None
    }

    /// Get a mutable dynamic reference to the value of a field on this type with the given field id.
    ///
    /// This method will return the current value of the given field if possible, or `None` if the
    /// given field does not exist or does not have a type matching the supplied type.
    fn field_any_mut(&mut self, _id: FieldId) -> Option<&mut any::Any> {
        None
    }
}

/// A trait that is implemented for every type to conditionally determine whether it exposes type
/// information.
pub trait TryTypeInfo {
    /// The constant statically known type information for this type, or `None` if the type does not
    /// implement `TypeInfo`.
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

/// Type information for a type that implements `TypeInfo`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Type {
    /// The globally unique identifier for this type.
    pub id: TypeId,

    /// The module in which this type was defined.
    /// This is using unrooted syntax à la `foo::bar`.
    pub module: &'static str,

    /// The identifier of this type within its module.
    pub ident: &'static str,

    /// Additional data about this type definition.
    pub data: Data,
}

/// Data associated with type information.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Data {
    /// The associated type is a primitive type.
    Primitive,
    /// The associated type is a `struct`.
    Struct(DataStruct),
    /// The associated type is an `enum`.
    Enum(DataEnum),
    /// The associated type is an `union`.
    Union(DataUnion),
}

/// Data associated with `struct` type information.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataStruct {
    /// The fields that this `struct` consists of.
    pub fields: Fields,
}

/// Data associated with `enum` type information.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataEnum {
    /// The variants that this `enum` consists of.
    pub variants: &'static [Variant],
}

/// Data associated with `union` type information.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataUnion {
    /// The fields that this `union` consists of.
    pub fields: FieldsNamed,
}

/// A specific `enum` variant.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variant {
    /// The identifier of the enum variant.
    pub ident: &'static str,
    /// The fields that are associated with a particular `enum` variant.
    pub fields: Fields,
}

/// A set of fields associated with a type or `enum` variant.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Fields {
    /// A set of named fields.
    Named(FieldsNamed),
    /// A set of index-addressed fields
    Unnamed(FieldsUnnamed),
    /// The empty set of fields, applicable to unit structs or enum variants.
    Unit,
}

/// A set of named fields associated with a type or `enum` variant.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FieldsNamed {
    /// The related set of named fields.
    pub named: &'static [Field],
}

/// A set of unnamed fields associated with a type or `enum` variant.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FieldsUnnamed {
    /// The related set of unnamed fields.
    pub unnamed: &'static [Field],
}

/// A field that is associated with a type or `enum` variant.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Field {
    /// The type or variant local unique identifier for the field.
    pub id: FieldId<'static>,
    /// The field's identifier, if it is named.
    pub ident: Option<&'static str>,
    /// The type of the field, if it has any associated `TypeInfo`.
    pub ty: Option<&'static Type>,
}

impl Type {
    /// Convenience method for getting all of the struct fields of this type.
    pub fn fields(&self) -> &'static [Field] {
        match self.data {
            Data::Struct(DataStruct { ref fields, .. }) => fields.fields(),
            _ => &[],
        }
    }

    /// Convenience method for getting all of the enum variants of this type.
    pub fn variants(&self) -> &'static [Variant] {
        match self.data {
            Data::Enum(DataEnum { ref variants, .. }) => variants,
            _ => &[],
        }
    }
}

impl Fields {
    /// Convenience method for getting all of the fields, ignoring whether they are named or
    /// unnamed.
    pub fn fields(&self) -> &'static [Field] {
        match *self {
            Fields::Unit => &[],
            Fields::Named(FieldsNamed { named, .. }) => named,
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
        }
    }
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
