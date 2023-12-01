use crate::header::DNSHeader;
use crate::question::DNSQuestion;

pub struct DNSResponse {
    pub header: DNSHeader,
    pub question: DNSQuestion,
}

impl DNSResponse {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.extend_from_slice(&self.header.serialize());
        buf.extend_from_slice(&self.question.serialize());

        buf
    }
}
