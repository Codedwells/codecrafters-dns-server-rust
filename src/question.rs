use byteorder::{BigEndian, ByteOrder};
use std::str::from_utf8;

#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(u16)]
pub enum DNSQueryType {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MR = 8,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(u16)]
pub enum DNSQueryClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

#[derive(Clone)]
pub struct DNSQuestion {
    pub domain_name: Vec<String>,
    pub query_type: DNSQueryType,
    pub query_class: DNSQueryClass,
}

impl DNSQuestion {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        for label in self.domain_name.iter() {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes());
        }

        // Null terminator
        buf.push(0);

        buf.extend_from_slice(&split_u16_to_u8(self.query_type as u16));
        buf.extend_from_slice(&split_u16_to_u8(self.query_class as u16));

        buf
    }

    pub fn deserialize(buf: &mut [u8; 512],questions_count:u16) -> Self {
        // 12 bytes are reserved for the header
        let mut offset = 12;

        for _ in 0..questions_count {
            let mut length = buf[offset] as usize;
            while length != 0 {
                if length & 0xC0 == 0xC0 {
                    offset += 2;
                    break;
                }
                offset += length + 1;
                length = buf[offset] as usize;
            }
            offset += 1;
            offset += 4;
        }

        let (domain_name, new_offset) = process_compressed_name(buf, offset);
        offset = new_offset;

        let query_type = BigEndian::read_u16(&buf[offset..offset + 2]);
        offset += 2;
        let query_class = BigEndian::read_u16(&buf[offset..offset + 2]);

        DNSQuestion {
            domain_name: domain_name.split('.').map(|s| s.to_string()).collect(),
            query_type: match query_type {
                1 => DNSQueryType::A,
                2 => DNSQueryType::NS,
                3 => DNSQueryType::MD,
                4 => DNSQueryType::MF,
                5 => DNSQueryType::CNAME,
                6 => DNSQueryType::SOA,
                7 => DNSQueryType::MB,
                8 => DNSQueryType::MR,
                10 => DNSQueryType::NULL,
                11 => DNSQueryType::WKS,
                12 => DNSQueryType::PTR,
                13 => DNSQueryType::HINFO,
                14 => DNSQueryType::MINFO,
                15 => DNSQueryType::MX,
                16 => DNSQueryType::TXT,
                _ => panic!("Unknown query type"),
            },
            query_class: match query_class {
                1 => DNSQueryClass::IN,
                2 => DNSQueryClass::CS,
                3 => DNSQueryClass::CH,
                4 => DNSQueryClass::HS,
                _ => panic!("Unknown query class"),
            },
        }
    }
}

fn process_compressed_name(buffer: &[u8], position: usize) -> (String, usize) {
    let mut offset = position;
    let mut name = String::new();
    loop {
        let length = buffer[offset] as usize;
        if length == 0 {
            break;
        }
        if !name.is_empty() {
            name.push('.');
        }
        if length & 0xC0 == 0xC0 {
            let next = u16::from_be_bytes(buffer[offset..offset + 2].try_into().unwrap()) & 0x3FFF;
            let (next_name, _) = process_compressed_name(buffer, next as usize);
            name.push_str(&next_name);
            offset += 1;
            break;
        }
        let next = match from_utf8(&buffer[offset + 1..offset + 1 + length]) {
            Ok(s) => s,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        name.push_str(next);
        offset += length + 1;
    }
    (name, offset + 1)
}

fn split_u16_to_u8(input: u16) -> [u8; 2] {
    [(input >> 8) as u8, input as u8]
}
