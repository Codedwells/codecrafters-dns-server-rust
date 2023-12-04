use std::net::Ipv4Addr;

use crate::question::{DNSQueryClass, DNSQueryType};

#[derive(Clone)]
pub struct DNSAnswer {
    pub name: Vec<String>,    // Label sequence for domain name
    pub typ: DNSQueryType,    // 2 bytes
    pub class: DNSQueryClass, // 2 bytes
    pub ttl: u32,             // 4 bytes
    pub rdlength: u16,        // 2 bytes
    pub rdata: Vec<u8>,       // variable size
}

impl DNSAnswer {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        for label in self.name.iter() {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes())
        }

        // Null terminator
        buf.push(0);

        buf.extend_from_slice(dns_type_to_bytes(self.typ).as_slice());
        buf.extend_from_slice(dns_class_to_bytes(self.class).as_slice());
        buf.extend_from_slice(&self.ttl.to_be_bytes());
        buf.extend_from_slice(&self.rdlength.to_be_bytes());
        buf.extend_from_slice(&self.rdata);

        buf
    }
}

pub fn _ipv4_to_bytes(ip: Ipv4Addr) -> Vec<u8> {
    ip.octets().to_vec()
}

pub fn dns_class_to_bytes(class: DNSQueryClass) -> Vec<u8> {
    match class {
        DNSQueryClass::IN => 1u16.to_be_bytes().to_vec(),
        DNSQueryClass::CS => 2u16.to_be_bytes().to_vec(),
        DNSQueryClass::CH => 3u16.to_be_bytes().to_vec(),
        DNSQueryClass::HS => 4u16.to_be_bytes().to_vec(),
    }
}

pub fn dns_type_to_bytes(typ: DNSQueryType) -> Vec<u8> {
    match typ {
        DNSQueryType::A => 1u16.to_be_bytes().to_vec(),
        DNSQueryType::NS => 2u16.to_be_bytes().to_vec(),
        DNSQueryType::MD => 3u16.to_be_bytes().to_vec(),
        DNSQueryType::MF => 4u16.to_be_bytes().to_vec(),
        DNSQueryType::CNAME => 5u16.to_be_bytes().to_vec(),
        DNSQueryType::SOA => 6u16.to_be_bytes().to_vec(),
        DNSQueryType::MB => 7u16.to_be_bytes().to_vec(),
        DNSQueryType::MR => 8u16.to_be_bytes().to_vec(),
        DNSQueryType::NULL => 10u16.to_be_bytes().to_vec(),
        DNSQueryType::WKS => 11u16.to_be_bytes().to_vec(),
        DNSQueryType::PTR => 12u16.to_be_bytes().to_vec(),
        DNSQueryType::HINFO => 13u16.to_be_bytes().to_vec(),
        DNSQueryType::MINFO => 14u16.to_be_bytes().to_vec(),
        DNSQueryType::MX => 15u16.to_be_bytes().to_vec(),
        DNSQueryType::TXT => 16u16.to_be_bytes().to_vec(),
    }
}
