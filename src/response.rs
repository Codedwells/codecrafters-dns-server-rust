use crate::answer::DNSAnswer;
use crate::header::DNSHeader;
use crate::question::DNSQuestion;

pub struct DNSResponse {
    pub header: DNSHeader,
    pub question: Vec<DNSQuestion>,
    pub answer: Vec<DNSAnswer>,
}

impl DNSResponse {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        let questions: Vec<DNSQuestion> = (0..self.header.question_count)
            .map(|_| {
                let question = DNSQuestion {
                    domain_name: self.question[0].domain_name.clone(),
                    query_type: self.question[0].query_type.clone(),
                    query_class: self.question[0].query_class.clone(),
                };
                question
            })
            .collect();

        let answers: Vec<DNSAnswer> = (0..self.header.answer_record_count)
            .map(|_| {
                let answer = DNSAnswer {
                    name: self.answer[0].name.clone(),
                    typ: self.answer[0].typ.clone(),
                    class: self.answer[0].class.clone(),
                    ttl: self.answer[0].ttl.clone(),
                    rdlength: self.answer[0].rdlength.clone(),
                    rdata: self.answer[0].rdata.clone(),
                };
                answer
            })
            .collect();

        todo!("Serialize DNSResponse");

        buf.extend_from_slice(&self.header.serialize());
        //buf.extend_from_slice();
        //buf.extend_from_slice(&self.answer.serialize());

        buf
    }
}
