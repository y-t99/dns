use crate::protocol::opcode_enum::OpCode;
use crate::protocol::rcode_enum::RCode;

/**
```
  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                      ID                       |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    QDCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    ANCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    NSCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    ARCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
```
*/
#[derive(Clone, Debug)]
pub struct DnsHeader {
    /**
       A 16 bit identifier assigned by the program that
       generates any kind of query. This identifier is copied
       the corresponding reply and can be used by the requester
       to match up replies to outstanding queries.
    */
    pub id: u16, // 16 bits

    /**
       A one bit field that specifies whether this message is a
       query (0), or a response (1).
    */
    pub response: bool, // 1 bit
    /**
       A four bit field that specifies kind of query in this
       message.  This value is set by the originator of a query
       and copied into the response.  The values are:
    */
    pub opcode: OpCode, // 4 bits
    /**
       Authoritative Answer - this bit is valid in responses,
       and specifies that the responding name server is an
       authority for the domain name in question section.

       Note that the contents of the answer section may have
       multiple owner names because of aliases.  The AA bit
       corresponds to the name which matches the query name, or
       the first owner name in the answer section.
    */
    pub authoritative_answer: bool, // 1 bit
    /**
       TrunCation - specifies that this message was truncated
       due to length greater than that permitted on the
       transmission channel.
    */
    pub truncated_message: bool, // 1 bit
    /**
       Recursion Desired - this bit may be set in a query and
       is copied into the response.  If RD is set, it directs
       the name server to pursue the query recursively.
       Recursive query support is optional.
    */
    pub recursion_desired: bool, // 1 bit
    /**
       Recursion Available - this be is set or cleared in a
       response, and denotes whether recursive query support is
       available in the name server.
    */
    pub recursion_available: bool, // 1 bit
    /**
       Reserved for future use.  Must be zero in all queries
       and responses.
    */
    pub z: bool, // 1 bit
    pub checking_disabled: bool, // 1 bit
    pub authed_data: bool,       // 1 bit
    /**
       Response code - this 4 bit field is set as part of
       responses.  The values have the following
       interpretation:
    */
    pub r_code: RCode, // 4 bits

    /**
       an unsigned 16 bit integer specifying the number of
       entries in the question section.
    */
    pub questions: u16, // 16 bits

    /**
       an unsigned 16 bit integer specifying the number of
       resource records in the answer section.
    */
    pub answers: u16, // 16 bits

    /**
       an unsigned 16 bit integer specifying the number of name
       server resource records in the authority records
       section.
    */
    pub authoritative_entries: u16, // 16 bits

    /**
       an unsigned 16 bit integer specifying the number of
       resource records in the additional records section.
    */
    pub resource_entries: u16, // 16 bits
}

impl DnsHeader {
    pub fn new() -> DnsHeader {
        DnsHeader {
            id: 0,

            response: false,
            opcode: OpCode::Query,
            authoritative_answer: false,
            truncated_message: false,
            recursion_desired: false,
            recursion_available: false,
            z: false,
            checking_disabled: false,
            authed_data: false,
            r_code: RCode::NoError,

            questions: 0,
            answers: 0,
            authoritative_entries: 0,
            resource_entries: 0,
        }
    }
}
