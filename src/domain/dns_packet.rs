use crate::domain::dns_packet_buffer::DnsPacketBuffer;
use crate::protocol::dns_header::DnsHeader;
use crate::protocol::dns_question::DnsQuestion;
use crate::protocol::dns_resource_record::DnsRecord;
use crate::protocol::opcode_enum::OpCode;
use crate::protocol::rcode_enum::RCode;

/**
```
+---------------------+
|        Header       |
+---------------------+
|       Question      | the question for the name server
+---------------------+
|        Answer       | RRs answering the question
+---------------------+
|      Authority      | RRs pointing toward an authority
+---------------------+
|      Additional     | RRs holding additional information
+---------------------+
```
*/
#[derive(Clone, Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn new() -> DnsPacket {
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            resources: Vec::new(),
        }
    }

    fn decode_header(buffer: &mut DnsPacketBuffer) -> Result<DnsHeader, &'static str> {
        let id = buffer.read_u16()?;

        let flags = buffer.read_u16()?;
        let h = (flags >> 8) as u8;
        let l = (flags & 0xFF) as u8;

        let response = (h & (1 << 7)) == 1;
        let opcode: OpCode = ((h >> 3) & 0x0F).try_into()?;
        let authoritative_answer = (h & (1 << 2)) == 1;
        let truncated_message = (h & (1 << 1)) == 1;
        let recursion_desired = (h & (1 << 0)) == 1;

        let recursion_available = (l & (1 << 7)) == 1;
        let z = (l & (1 << 6)) == 1;
        let authed_data = (l & (1 << 5)) == 1;
        let checking_disabled = (l & (1 << 4)) == 1;
        let r_code: RCode = (l & 0x0F).try_into()?;

        let questions = buffer.read_u16()?;
        let answers = buffer.read_u16()?;
        let authoritative_entries = buffer.read_u16()?;
        let resource_entries = buffer.read_u16()?;

        Ok(DnsHeader {
            id,

            response,
            opcode,
            authoritative_answer,
            truncated_message,
            recursion_desired,
            recursion_available,
            z,
            authed_data,
            checking_disabled,
            r_code,

            questions,
            answers,
            authoritative_entries,
            resource_entries,
        })
    }
}
