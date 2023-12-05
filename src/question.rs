#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
#[repr(u16)]
pub enum DNSQueryClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

#[derive(Clone, Debug)]
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

    pub fn deserialize(buf: &mut [u8; 512]) -> Vec<DNSQuestion> {
        let mut res = Vec::new();
        let mut finished = false;
        let mut current_position: usize = (12) as usize;
        while !finished {
            let mut length = buf[current_position] as usize;
            let mut name: String = String::new();
            while length != 0 {
                let compressed = (buf[current_position] & 0b11000000) != 0;
                if compressed {
                    current_position += 1;
                    let pointer = buf[current_position] as usize;
                    let (returned_name, _) = &process_compressed_name(buf, pointer);
                    name += returned_name;
                    break;
                } else {
                    let (returned_name, updated_pos) =
                        process_compressed_name(buf, current_position);
                    name += &returned_name;
                    current_position = updated_pos;
                }
                length = buf[current_position] as usize;
            }
            current_position += 1;
            let _type = u16::from_be_bytes([buf[current_position], buf[current_position + 1]]);
            let class = u16::from_be_bytes([buf[current_position + 2], buf[current_position + 3]]);

            res.push(DNSQuestion {
                domain_name: name.split('.').map(|s| s.to_string()).collect(),
                query_type: match _type {
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
                query_class: match class {
                    1 => DNSQueryClass::IN,
                    2 => DNSQueryClass::CS,
                    3 => DNSQueryClass::CH,
                    4 => DNSQueryClass::HS,
                    _ => panic!("Unknown query class"),
                },
            });
            current_position += 4;
            if buf[current_position] as u8 == 0 {
                finished = true;
            }
        }
        res
    }
}

fn process_compressed_name(buf: &[u8], position: usize) -> (String, usize) {
    let mut length = buf[position] as usize;
    let mut name: String = String::new();
    let mut current_position = position;

    while length != 0 {
        current_position += 1;
        name +=
            &String::from_utf8(buf[current_position..current_position + length].to_vec()).unwrap();
        current_position += length;
        length = buf[current_position] as usize;

        let compressed = (buf[current_position] & 0b11000000) != 0;
        if compressed {
            name += ".";
            break;
        }
        if length != 0 {
            name += ".";
        }
    }

    (name, current_position)
}

fn split_u16_to_u8(input: u16) -> [u8; 2] {
    [(input >> 8) as u8, input as u8]
}
