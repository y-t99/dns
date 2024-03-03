#[derive(Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ResourceType {
    A = 1u16,               // A host address
    NS = 2u16,              // An authoritative name server
    MD = 3u16,              // A mail destination (Obsolete - use MX)
    MF = 4u16,              // A mail forwarder (Obsolete - use MX)
    CName = 5u16,           // The canonical name for an alias
    SOA = 6u16,             // Marks the start of a zone of authority
    MB = 7u16,              // A mailbox domain name (EXPERIMENTAL)
    MG = 8u16,              // A mail group member (EXPERIMENTAL)
    MR = 9u16,              // A mail rename domain name (EXPERIMENTAL)
    Null = 10u16,           // A null RR (EXPERIMENTAL)
    WKS = 11u16,            // A well known service description
    PTR = 12u16,            // A domain name pointer
    HInfo = 13u16,          // Host information
    MInfo = 14u16,          // Mailbox or mail list information
    MX = 15u16,             // Mail exchange
    Txt = 16u16,            // Text strings
}