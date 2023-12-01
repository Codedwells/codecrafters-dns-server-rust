#[allow(dead_code)]
#[derive(Copy,Clone)]
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
#[derive(Copy,Clone)]
pub enum DNSQueryClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

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

}

fn split_u16_to_u8(input: u16) -> [u8; 2] {
    [(input >> 8) as u8, input as u8]
}
