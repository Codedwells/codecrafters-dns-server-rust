use crate::answer::DNSAnswer;
use crate::header::DNSHeader;
use crate::question::DNSQuestion;

pub struct DNSResponse {
    pub header: DNSHeader,
    pub question: DNSQuestion,
    pub answer: DNSAnswer,
}

impl DNSResponse {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.extend_from_slice(&self.header.serialize());
        buf.extend_from_slice(&self.question.serialize());
        buf.extend_from_slice(&self.answer.serialize());

        buf
    }
}
