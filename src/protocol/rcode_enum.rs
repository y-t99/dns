#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RCode {
    /**
       No error condition
    */
    NoError = 0u8,
    /**
       Format error - The name server was
       unable to interpret the query.
    */
    FormErr = 1u8,
    /**
       Server failure - The name server was
       unable to process this query due to a
       problem with the name server.
    */
    ServFail = 2u8,
    /**
       Name Error - Meaningful only for
       responses from an authoritative name
       server, this code signifies that the
       domain name referenced in the query does
       not exist.
    */
    NameErr = 3u8,
    /**
       Not Implemented - The name server does
       not support the requested kind of query.
    */
    NotImp = 4u8,
    /**
       Refused - The name server refuses to
       perform the specified operation for
       policy reasons.  For example, a name
       server may not wish to provide the
       information to the particular requester,
       or a name server may not wish to perform
       a particular operation (e.g., zone
    */
    Refused = 5u8,
}

impl TryFrom<u8> for RCode {
    type Error = &'static str;

    fn try_from(v: u8) -> Result<Self, &'static str> {
        match v {
            x if x == RCode::NoError.into() => Ok(RCode::NoError),
            x if x == RCode::FormErr.into() => Ok(RCode::FormErr),
            x if x == RCode::ServFail.into() => Ok(RCode::ServFail),
            x if x == RCode::NameErr.into() => Ok(RCode::NameErr),
            x if x == RCode::NotImp.into() => Ok(RCode::NotImp),
            x if x == RCode::Refused.into() => Ok(RCode::Refused),
            _ => Err("RCode No Exist"),
        }
    }
}

impl From<RCode> for u8 {
    fn from(code: RCode) -> Self {
        match code {
            RCode::NoError => 0,
            RCode::FormErr => 1,
            RCode::ServFail => 2,
            RCode::NameErr => 3,
            RCode::NotImp => 4,
            RCode::Refused => 5,
        }
    }
}
