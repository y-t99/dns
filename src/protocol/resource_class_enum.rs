#[derive(Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ResourceClass {
    IN = 1u16, // the Internet
    CS = 2u16, // the CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CH = 3u16, // the CHAOS class
    HS = 4u16, // Hesiod [Dyer 87]
}