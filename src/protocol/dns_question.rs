use crate::protocol::question_class_enum::QuestionClass;
use crate::protocol::question_type_enum::QuestionType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub name: String,
    pub q_type: QuestionType,
    pub q_class: QuestionClass,
}

impl DnsQuestion {
    pub fn new(name: String, q_type: QuestionType, q_class: QuestionClass) -> DnsQuestion {
        DnsQuestion {
            name,
            q_type,
            q_class,
        }
    }
}