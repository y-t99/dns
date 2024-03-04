use crate::protocol::resource_class_enum::ResourceClass;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum QuestionClass {
    Base(ResourceClass),
    /// any class
    All = 255u16,
}
