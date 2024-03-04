#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ResourceType {
    /// A host address
    A = 1u16,
    /// An authoritative name server
    NS = 2u16,
    /// A mail destination (Obsolete - use MX)
    MD = 3u16,
    /// A mail forwarder (Obsolete - use MX)
    MF = 4u16,
    /// The canonical name for an alias
    CName = 5u16,
    /// Marks the start of a zone of authority
    SOA = 6u16,
    /// A mailbox domain name (EXPERIMENTAL)
    MB = 7u16,
    /// A mail group member (EXPERIMENTAL)
    MG = 8u16,
    /// A mail rename domain name (EXPERIMENTAL)
    MR = 9u16,
    /// A null RR (EXPERIMENTAL)
    Null = 10u16,
    /// A well known service description
    WKS = 11u16,
    /// A domain name pointer
    PTR = 12u16,
    /// Host information
    HInfo = 13u16,
    /// Mailbox or mail list information
    MInfo = 14u16,
    /// Mail exchange
    MX = 15u16,
    /// Text strings
    Txt = 16u16,
}
