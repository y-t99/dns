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
