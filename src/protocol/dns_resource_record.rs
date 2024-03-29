use crate::protocol::resource_class_enum::ResourceClass;
use crate::protocol::resource_type_enum::ResourceType;

/**
```
  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                                               |
/                                               /
/                      NAME                     /
|                                               |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                      TYPE                     |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                     CLASS                     |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                      TTL                      |
|                                               |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                   RDLENGTH                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
/                     RDATA                     /
/                                               /
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
```
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsRecord {
    /// a domain name to which this resource record pertains.
    pub name: String,
    /**
        two octets containing one of the RR type codes.  This
        field specifies the meaning of the data in the RDATA
        field.
    */
    pub r_type: ResourceType,
    /**
        two octets which specify the class of the data in the
        RDATA field.
    */
    pub r_class: ResourceClass,
    /**
       a 32 bit unsigned integer that specifies the time
       interval (in seconds) that the resource record may be
       cached before it should be discarded.  Zero values are
       interpreted to mean that the RR can only be used for the
       transaction in progress, and should not be cached.
    */
    pub ttl: u32,
    /**
        an unsigned 16 bit integer that specifies the length in
        octets of the RDATA field.
    */
    pub rd_length: u16,
    /**
        a variable length string of octets that describes the
        resource.  The format of this information varies
        according to the TYPE and CLASS of the resource record.
        For example, the if the TYPE is A and the CLASS is IN,
        the RDATA field is a 4 octet ARPA Internet address.
    */
    pub r_data: String,
}

impl DnsRecord {
    pub fn new() -> DnsRecord {
        DnsRecord {
            name: String::new(),
            r_type: ResourceType::A,
            r_class: ResourceClass::IN,
            ttl: 0,
            rd_length: 0,
            r_data: String::new(),
        }
    }
}
