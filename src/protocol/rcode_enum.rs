#[derive(Copy, Debug, PartialEq, Eq)]
pub enum RCode {
    /*
        No error condition
     */
    NoError = 0u32,
    /*
        Format error - The name server was
        unable to interpret the query.
     */
    FormErr = 1u32,
    /*
        Server failure - The name server was
        unable to process this query due to a
        problem with the name server.
     */
    ServFail = 2u32,
    /*
        Name Error - Meaningful only for
        responses from an authoritative name
        server, this code signifies that the
        domain name referenced in the query does
        not exist.
     */
    NameErr = 3u32,
    /*
        Not Implemented - The name server does
        not support the requested kind of query.
     */
    NotImp = 4u32,
    /*
        Refused - The name server refuses to
        perform the specified operation for
        policy reasons.  For example, a name
        server may not wish to provide the
        information to the particular requester,
        or a name server may not wish to perform
        a particular operation (e.g., zone
     */
    Refused = 5u32,
}