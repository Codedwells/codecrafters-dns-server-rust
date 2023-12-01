mod header;
mod question;
mod response;

use header::DNSHeader;
use question::DNSQuestion;
use response::DNSResponse;
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, source);

                let response = DNSResponse {
                    header: DNSHeader {
                        packet_id: 1234, // Takes up 16 bits -> 2 bytes
                        qr_indicator: 1,
                        opcode: 0,
                        authoritative_answer: 0,
                        truncation: 0,
                        recursion_desired: 0,
                        recursion_available: 0,
                        reserved: 0,
                        response_code: 0,
                        question_count: 1,
                        answer_record_count: 0,
                        authority_record_count: 0,
                        additional_record_count: 0,
                    },
                    question: DNSQuestion {
                        domain_name: vec![
                            "codecrafters".to_string(),
                            "io".to_string(),
                        ],
                        query_type: question::DNSQueryType::A,
                        query_class: question::DNSQueryClass::IN,
                    },
                }
                .serialize();

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
