use crate::domain::dns_packet_buffer::DnsPacketBuffer;
use crate::protocol::dns_header::DnsHeader;
use crate::protocol::dns_question::DnsQuestion;
use crate::protocol::dns_resource_record::DnsRecord;
use crate::protocol::opcode_enum::OpCode;
use crate::protocol::question_class_enum::QuestionClass;
use crate::protocol::question_type_enum::QuestionType;
use crate::protocol::rcode_enum::RCode;
use crate::protocol::resource_class_enum::ResourceClass;
use crate::protocol::resource_type_enum::ResourceType;

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

    pub fn decode(buffer: &mut DnsPacketBuffer) -> Result<DnsPacket, &'static str> {
        let header = Self::decode_header(buffer).unwrap();

        let mut questions = Vec::new();
        for _ in 0..header.questions {
            let question = Self::decode_question(buffer)?;
            questions.push(question);
        }

        let mut answers = Vec::new();
        for _ in 0..header.answers {
            let answer = Self::decode_record(buffer)?;
            answers.push(answer);
        }

        let mut authorities = Vec::new();
        for _ in 0..header.authoritative_entries {
            let authority = Self::decode_record(buffer)?;
            authorities.push(authority);
        }

        let mut resources = Vec::new();
        for _ in 0..header.resource_entries {
            let resource = Self::decode_record(buffer)?;
            resources.push(resource);
        }

        Ok(DnsPacket {
            header,
            questions,
            answers,
            authorities,
            resources,
        })
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

    /**
        Name space definitions

        Domain names in messages are expressed in terms of a sequence of labels.
        Each label is represented as a one octet length field followed by that
        number of octets.  Since every domain name ends with the null label of
        the root, a domain name is terminated by a length byte of zero.  The
        high order two bits of every length octet must be zero, and the
        remaining six bits of the length field limit the label to 63 octets or
        less. => **sequence limit 1 byte & labels limit 63 bytes**

        To simplify implementations, the total length of a domain name (i.e.,
        label octets and label length octets) is restricted to 255 octets or
        less. => **sequences plus labels limit 255 bytes**

        Although labels can contain any 8 bit values in octets that make up a
        label, it is strongly recommended that labels follow the preferred
        syntax described elsewhere in this memo, which is compatible with
        existing host naming conventions. Name servers and resolvers must
        compare labels in a case-insensitive manner (i.e., A=a), assuming ASCII
        with zero parity. Non-alphabetic codes must match exactly.

        The tricky part: Reading domain names, taking labels into consideration.
        Will take something like [3]www[6]google[3]com[0] and append
        www.google.com

        In order to reduce the size of messages, the domain system utilizes a
        compression scheme which eliminates the repetition of domain names in a
        message. In this scheme, an entire domain name or a list of labels at
        the end of a domain name is replaced with a pointer to a prior occurance
        of the same name.
    */
    fn decode_name(buffer: &mut DnsPacketBuffer) -> Result<String, &'static str> {
        let mut domain_name = String::new();

        let mut pos = buffer.pos();

        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;

        let mut delimiter = "";

        loop {
            let len = buffer.get(pos)?;

            /*
            The pointer takes the form of a two octet sequence:
                +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
                | 1  1|                OFFSET                   |
                +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
             */
            if (len & 0xC0) == 0xC0 {
                if jumps_performed > max_jumps {
                    return Err("Limit of {} jumps exceeded");
                }

                if !jumped {
                    buffer.seek(pos + 2)?;
                }

                let l = buffer.get(pos + 1)? as u16;
                let h = ((len as u16) ^ 0xC0) << 8;
                let offset = h | l;
                pos = offset as usize;

                jumped = true;
                jumps_performed += 1;

                continue;
            } else {
                pos += 1;

                if len == 0 {
                    break;
                }

                domain_name.push_str(delimiter);

                let str_buffer = buffer.get_range(pos, len as usize)?;
                domain_name.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                delimiter = ".";

                pos += len as usize;
            }
        }

        if !jumped {
            buffer.seek(pos)?;
        }

        Ok(domain_name)
    }

    fn decode_question(buffer: &mut DnsPacketBuffer) -> Result<DnsQuestion, &'static str> {
        let name = Self::decode_name(buffer)?;
        let q_type: QuestionType = buffer.read_u16()?.try_into()?;
        let q_class: QuestionClass = buffer.read_u16()?.try_into()?;
        Ok(DnsQuestion {
            name,
            q_type,
            q_class,
        })
    }

    fn decode_record(buffer: &mut DnsPacketBuffer) -> Result<DnsRecord, &'static str> {
        let domain = Self::decode_name(buffer)?;

        let resource_type: ResourceType = buffer.read_u16()?.try_into()?;
        let resource_class: ResourceClass = buffer.read_u16()?.try_into()?;

        let ttl = buffer.read_u32()?;
        let rd_length = buffer.read_u16()?;

        let str_buffer = buffer.get_range(buffer.pos(), rd_length as usize)?;
        let r_data = String::from_utf8_lossy(str_buffer).to_string();
        buffer.step(rd_length as usize)?;

        Ok(DnsRecord {
            name: domain,
            r_type: resource_type,
            r_class: resource_class,
            ttl,
            rd_length,
            r_data,
        })
    }
}
