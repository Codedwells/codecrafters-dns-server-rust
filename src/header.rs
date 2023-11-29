pub struct DNSHeader {
     pub packet_id:u16,
     pub qr_indicator:u8,
     pub opcode:u8,
     pub authoritative_answer:u8,
     pub truncation:u8,
     pub recursion_desired:u8,
     pub recursion_available:u8,
     pub reserved:u8,
     pub response_code:u8,
     pub question_count:u16,
     pub answer_record_count:u16,
     pub authority_record_count:u16,
     pub additional_record_count:u16,
 }

// Header is 12 bytes long and should be encoded using big endian.
impl DNSHeader {
    pub fn new() -> Self {
        DNSHeader {
            packet_id: 1234, // Takes up 16 bits -> 2 bytes
            qr_indicator: 1,
            opcode: 0,
            authoritative_answer: 0,
            truncation: 0,
            recursion_desired: 0,
            recursion_available: 0,
            reserved: 0,
            response_code: 0,
            question_count: 0,
            answer_record_count: 0,
            authority_record_count: 0,
            additional_record_count: 0,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // Our packet_id is 2 bytes long (16 bits), and since our vec! is of u8 type we need two indexes in our vec! to store it
        buf.extend_from_slice(&self.packet_id.to_be_bytes());
        // Takes up 1 byte -> (1 bit, 4 bit, 1 bit, 1 bit, 1 bit)
        buf.push((self.qr_indicator << 7) | (self.opcode << 3) | (self.authoritative_answer << 2) | (self.truncation << 1) | self.recursion_desired); // pushes e.g [141]
        // Takes up 1 byte -> (1 bit, 3 bit, 4 bit)
        buf.push((self.recursion_available  << 7) | (self.reserved << 4) | (self.response_code));
        // Takes up 2 bytes -> 16 bits
        buf.extend_from_slice(&self.question_count.to_be_bytes());
        // Takes up 2 bytes -> 16 bits
        buf.extend_from_slice(&self.answer_record_count.to_be_bytes());
        // Takes up 2 bytes -> 16 bits
        buf.extend_from_slice(&self.authority_record_count.to_be_bytes());
        // Takes up 2 bytes -> 16 bits
        buf.extend_from_slice(&self.additional_record_count.to_be_bytes());

        buf
    }
}
