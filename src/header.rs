use byteorder::{BigEndian, ByteOrder};

#[repr(C, packed)]
pub struct DNSHeader {
    pub packet_id: u16,               // 16 bits
    pub qr_indicator: u8,             // 1 bit
    pub opcode: u8,                   // 4 bits
    pub authoritative_answer: u8,     // 1 bit
    pub truncation: u8,               // 1 bit
    pub recursion_desired: u8,        // 1 bit
    pub recursion_available: u8,      // 1 bit
    pub reserved: u8,                 // 3 bits
    pub response_code: u8,            // 4 bits
    pub question_count: u16,          // 16 bits
    pub answer_record_count: u16,     // 16 bits
    pub authority_record_count: u16,  // 16 bits
    pub additional_record_count: u16, // 16 bits
}

// Header is 12 bytes long and should be encoded using big endian.
impl DNSHeader {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // Our packet_id is 2 bytes long (16 bits), and since our vec! is of u8 type we need two indexes in our vec! to store it
        buf.extend_from_slice(&self.packet_id.to_be_bytes());
        // Takes up 1 byte -> (1 bit, 4 bit, 1 bit, 1 bit, 1 bit)
        buf.push(
            (self.qr_indicator << 7)
                | (self.opcode << 3)
                | (self.authoritative_answer << 2)
                | (self.truncation << 1)
                | self.recursion_desired,
        ); // pushes e.g [141]
           // Takes up 1 byte -> (1 bit, 3 bit, 4 bit)
        buf.push((self.recursion_available << 7) | (self.reserved << 4) | (self.response_code));
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

    pub fn deserialize(buf: &mut [u8; 512]) -> DNSHeader {
        unsafe {
            let mut buffer: DNSHeader = std::mem::zeroed();

            let header_bytes = std::slice::from_raw_parts_mut(&mut buffer as *mut _ as *mut u8, 12);
            header_bytes.copy_from_slice(&buf[0..12]);

            // Byte 0..2 are packet_id -> 16 bits
            buffer.packet_id = BigEndian::read_u16(&buf[0..2]);

            // Byte 2..4  are (qr_indicator -> 1bit, opcode -> 4bits, authoritative_answer -> 1 bit, truncation -> 1 bit, recursion_desired -> 1 bit)
            buffer.qr_indicator = buf[2] >> 7;
            buffer.opcode = (buf[2] >> 3) & 0b0000_1111;
            buffer.authoritative_answer = (buf[2] >> 2) & 0b0000_0001;
            buffer.truncation = (buf[2] >> 1) & 0b0000_0001;
            buffer.recursion_desired = buf[2] & 0b0000_0001;

            // Byte 3..4 are (recursion_available -> 1 bit, reserved -> 3 bits, response_code -> 4 bits)
            buffer.recursion_available = buf[3] >> 7;
            buffer.reserved = (buf[3] >> 4) & 0b00000111;
            buffer.response_code = buf[3] & 0b0000_1111;

            // Byte 4..6 are question_count -> 16 bits
            buffer.question_count = BigEndian::read_u16(&buf[4..6]);
            // Byte 6..8 are answer_record_count -> 16 bits
            buffer.answer_record_count = BigEndian::read_u16(&buf[6..8]);
            // Byte 8..10 are authority_record_count -> 16 bits
            buffer.authority_record_count = BigEndian::read_u16(&buf[8..10]);
            // Byte 10..12 are additional_record_count -> 16 bits
            buffer.additional_record_count = BigEndian::read_u16(&buf[10..12]);

            buffer
        }
    }
}
