 use std::net::UdpSocket;

 pub struct DNSHeader {
     id:u16,
     qr:u8,
     opcode:u8,
     aa:u8,
     tc:u8,
     rd:u8,
     ra:u8,
     z:u8,
     rcode:u8,
     qdcount:u16,
     ancount:u16,
     nscount:u16,
     arcount:u16,
 }

impl DNSHeader {
    pub fn new() -> Self {
        DNSHeader {
            id: 1234,
            qr: 0,
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 0,
            ra: 0,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.extend_from_slice(&self.id.to_be_bytes());
        buf.push((self.qr << 7) | (self.opcode << 3) | (self.aa << 2) | (self.tc << 1) | self.rd);


    }
}

fn main() {

     let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
     let mut buf = [0; 512];

     loop {
         match udp_socket.recv_from(&mut buf) {
             Ok((size, source)) => {
                 let _received_data = String::from_utf8_lossy(&buf[0..size]);
                 println!("Received {} bytes from {}", size, source);
                 let response = [];
                 udp_socket
                     .send_to(&response, source)
                     .expect("Failed to send response");
             }
             Err(e) => {
                 eprintln!("Error receiving data: {}", e);
                 break;
             }
         }
     }
}
