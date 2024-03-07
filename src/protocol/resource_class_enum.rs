#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ResourceClass {
    /// the Internet
    IN = 1u16,
    /// the CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS = 2u16,
    /// the CHAOS class
    CH = 3u16,
    /// Hesiod [Dyer 87]
    HS = 4u16,
}

impl TryFrom<u16> for ResourceClass {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == ResourceClass::IN.into() => Ok(ResourceClass::IN),
            x if x == ResourceClass::CS.into() => Ok(ResourceClass::CS),
            x if x == ResourceClass::CH.into() => Ok(ResourceClass::CH),
            x if x == ResourceClass::HS.into() => Ok(ResourceClass::HS),
            _ => Err("ResourceClass No Exist"),
        }
    }
}

impl From<ResourceClass> for u16 {
    fn from(value: ResourceClass) -> Self {
        match value {
            ResourceClass::IN => 1u16,
            ResourceClass::CS => 2u16,
            ResourceClass::CH => 3u16,
            ResourceClass::HS => 4u16,
        }
    }
}
