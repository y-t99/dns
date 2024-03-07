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

impl TryFrom<u16> for ResourceType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == ResourceType::A.into() => Ok(ResourceType::A),
            x if x == ResourceType::NS.into() => Ok(ResourceType::NS),
            x if x == ResourceType::MD.into() => Ok(ResourceType::MD),
            x if x == ResourceType::MF.into() => Ok(ResourceType::MF),
            x if x == ResourceType::CName.into() => Ok(ResourceType::CName),
            x if x == ResourceType::SOA.into() => Ok(ResourceType::SOA),
            x if x == ResourceType::MB.into() => Ok(ResourceType::MB),
            x if x == ResourceType::Null.into() => Ok(ResourceType::Null),
            x if x == ResourceType::WKS.into() => Ok(ResourceType::WKS),
            x if x == ResourceType::PTR.into() => Ok(ResourceType::PTR),
            x if x == ResourceType::HInfo.into() => Ok(ResourceType::HInfo),
            x if x == ResourceType::MInfo.into() => Ok(ResourceType::MInfo),
            x if x == ResourceType::MX.into() => Ok(ResourceType::MX),
            x if x == ResourceType::Txt.into() => Ok(ResourceType::Txt),
            _ => Err("ResourceType No Exist"),
        }
    }
}

impl From<ResourceType> for u16 {
    fn from(value: ResourceType) -> Self {
        match value {
            ResourceType::A => 1u16,
            ResourceType::NS => 2u16,
            ResourceType::MD => 3u16,
            ResourceType::MF => 4u16,
            ResourceType::CName => 5u16,
            ResourceType::SOA => 6u16,
            ResourceType::MB => 7u16,
            ResourceType::MG => 8u16,
            ResourceType::MR => 9u16,
            ResourceType::Null => 10u16,
            ResourceType::WKS => 11u16,
            ResourceType::PTR => 12u16,
            ResourceType::HInfo => 13u16,
            ResourceType::MInfo => 14u16,
            ResourceType::MX => 15u16,
            ResourceType::Txt => 16u16,
        }
    }
}
