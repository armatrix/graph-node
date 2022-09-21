#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityChanges {
    #[prost(message, repeated, tag="5")]
    pub entity_changes: ::prost::alloc::vec::Vec<EntityChange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityChange {
    #[prost(string, tag="1")]
    pub entity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub ordinal: u64,
    #[prost(enumeration="entity_change::Operation", tag="4")]
    pub operation: i32,
    #[prost(message, repeated, tag="5")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
}
/// Nested message and enum types in `EntityChange`.
pub mod entity_change {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        /// Protobuf default should not be used, this is used so that the consume can ensure that the value was actually specified
        Unset = 0,
        Create = 1,
        Update = 2,
        Delete = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="field::Type", tag="2")]
    pub value_type: i32,
    #[prost(string, tag="3")]
    pub new_value: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub new_value_null: bool,
    #[prost(string, tag="5")]
    pub old_value: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub old_value_null: bool,
}
/// Nested message and enum types in `Field`.
pub mod field {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Protobuf default should not be used, this is used so that the consume can ensure that the value was actually specified
        Unset = 0,
        Bigdecimal = 1,
        Bigint = 2,
        Int32 = 3,
        /// Serialized as Base64 strings.
        Bytes = 4,
        String = 5,
    }
}
