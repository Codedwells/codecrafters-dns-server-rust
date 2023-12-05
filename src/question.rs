use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use std::io::Cursor;

#[allow(dead_code)]
#[derive(Copy, Clone)]
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

    pub fn deserialize(buf: &mut [u8; 512]) -> Self {
        // 12 bytes are reserved for the header
        let mut offset = 12;

        let mut domain_name = Vec::new();

        loop {
            let label_length = buf[offset] as usize;

            if label_length == 0 {
                break;
            }

            if (label_length & 0b1100_0000) == 0b1100_0000 {
                // Compression pointer
                let mut cursor = Cursor::new(&buf[offset..offset + 2]);
                let pointer = (cursor.read_u16::<BigEndian>().unwrap() & 0x3FFF) as usize;

                // Follow the pointer and continue parsing
                offset = pointer;
                continue;
            }

            offset += 1;

            let label = String::from_utf8_lossy(&buf[offset..offset + label_length]);
            domain_name.push(label.to_string());
            offset += label_length;
        }

        let query_type = BigEndian::read_u16(&buf[offset..offset + 2]);
        offset += 2;
        let query_class = BigEndian::read_u16(&buf[offset..offset + 2]);

        DNSQuestion {
            domain_name,
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

fn split_u16_to_u8(input: u16) -> [u8; 2] {
    [(input >> 8) as u8, input as u8]
}
