use std::ops::Index;
use crate::domain::dns_packet::DnsPacket;
use crate::domain::dns_packet_buffer::DnsPacketBuffer;

mod domain;
mod protocol;

fn hex_str_to_bits(hex_str: &str) -> Result<Vec<u8>, &'static str> {
    if hex_str.len() % 2 != 0 {
        return Err("Hex String Formatter Error")
    }
    let mut result = vec![];
    for i in 0..hex_str.len() / 2 {
        let pos = i * 2;
        let h = u8::from_str_radix(&hex_str[pos..(pos+1)], 16).unwrap();
        let l = u8::from_str_radix(&hex_str[(pos+1)..(pos+2)], 16).unwrap();
        let bit = (h << 4) | l;
        result.push(bit);
    }
    Ok(result)
}

fn main() -> Result<(), &'static str> {

    let dns_response = "862a8180000100010000000006676f6f676c6503636f6d0000010001c00c00010001000001250004d83ad38e".to_string();
    let bits = hex_str_to_bits(&dns_response)?;
    let mut buf: [u8; 512] = [0; 512];

    for i in 0..bits.len() {
        buf[i] = bits[i];
    }

    let mut buffer = DnsPacketBuffer { pos: 0, buf };

    let packet = DnsPacket::decode(&mut buffer)?;

    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
