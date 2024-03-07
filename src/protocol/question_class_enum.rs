use crate::protocol::question_class_enum::QuestionClass::Base;
use crate::protocol::resource_class_enum::ResourceClass;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum QuestionClass {
    Base(ResourceClass),
    /// any class
    All = 255u16,
}

impl TryFrom<u16> for QuestionClass {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let resource_bound: [u16; 2] = [ResourceClass::IN.into(), ResourceClass::HS.into()];
        match value {
            x if resource_bound[0] <= x && x <= resource_bound[1] => {
                let resource_class: ResourceClass = x.try_into()?;
                Ok(Base(resource_class))
            }
            x if x == QuestionClass::All.into() => Ok(QuestionClass::All),
            _ => Err("ResourceClass No Exist"),
        }
    }
}

impl From<QuestionClass> for u16 {
    fn from(value: QuestionClass) -> Self {
        match value {
            Base(resource_class) => resource_class.into(),
            QuestionClass::All => 255u16,
        }
    }
}
