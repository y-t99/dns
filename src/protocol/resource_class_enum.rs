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
