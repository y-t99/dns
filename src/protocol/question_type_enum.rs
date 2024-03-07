use crate::protocol::resource_type_enum::ResourceType;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum QuestionType {
    Base(ResourceType),
    /// A request for a transfer of an entire zone
    AxfR = 252u16,
    /// A request for mailbox-related records (MB, MG or MR)
    MailB = 253u16,
    /// A request for mail agent RRs (Obsolete - see MX)
    MailA = 254u16,
    /// A request for all records
    All = 255u16,
}

impl TryFrom<u16> for QuestionType {
    type Error = &'static str;

    fn try_from(v: u16) -> Result<Self, &'static str> {
        let resource_bound: [u16; 2] = [ResourceType::A.into(), ResourceType::Txt.into()];
        match v {
            x if resource_bound[0] <= x && x <= resource_bound[1] => {
                let resource_type = x.try_into()?;
                Ok(QuestionType::Base(resource_type))
            }
            x if x == QuestionType::AxfR.into() => Ok(QuestionType::AxfR),
            x if x == QuestionType::MailB.into() => Ok(QuestionType::MailB),
            x if x == QuestionType::MailA.into() => Ok(QuestionType::MailA),
            x if x == QuestionType::All.into() => Ok(QuestionType::All),
            _ => Err("QuestionType No Exist"),
        }
    }
}

impl From<QuestionType> for u16 {
    fn from(value: QuestionType) -> Self {
        match value {
            QuestionType::Base(resource_type) => resource_type.into(),
            QuestionType::AxfR => 252u16,
            QuestionType::MailB => 253u16,
            QuestionType::MailA => 254u16,
            QuestionType::All => 255u16,
        }
    }
}
