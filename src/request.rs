use crate::header::DNSHeader;
use crate::question::DNSQuestion;

#[derive(Debug)]
pub struct DNSRequest {
    pub header: DNSHeader,
    pub question: DNSQuestion,
    pub questions: Vec<DNSQuestion>,
}

impl DNSRequest {
    pub fn deserialize(buf: &mut [u8; 512]) -> Self {
        let header = DNSHeader::deserialize(buf);

        let questions:Vec<DNSQuestion> = (0..header.question_count)
            .map(|_| {
                let question = DNSQuestion::deserialize(buf);
                question
            })
            .collect();

            Self {
                header,
                question: questions[0].clone(),
                questions,
            }

    }
}
