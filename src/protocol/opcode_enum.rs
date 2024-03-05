use crate::protocol::rcode_enum::RCode;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    /**
       a standard query
    */
    Query = 0u8,

    /**
       an inverse query
    */
    IQuery = 1u8,

    /**
       a server status request
    */
    Status = 2u8,
}

impl TryFrom<u8> for OpCode {
    type Error = &'static str;

    fn try_from(v: u8) -> Result<Self, &'static str> {
        match v {
            x if x == OpCode::Query.into() => Ok(OpCode::Query),
            x if x == OpCode::IQuery.into() => Ok(OpCode::IQuery),
            x if x == OpCode::Status.into() => Ok(OpCode::Status),
            _ => Err("OpCode No Exist"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> Self {
        match code {
            OpCode::Query => 0,
            OpCode::IQuery => 1,
            OpCode::Status => 2,
        }
    }
}
