mod answer;
mod header;
mod question;
mod response;

use answer::DNSAnswer;
use header::DNSHeader;
use question::{DNSQueryClass, DNSQueryType, DNSQuestion};
use response::DNSResponse;
use std::net::UdpSocket;
use std::net::{Ipv4Addr, SocketAddrV4};

fn main() {
    let ip_addr = Ipv4Addr::LOCALHOST;
    let udp_socket =
        UdpSocket::bind(SocketAddrV4::new(ip_addr, 2053)).expect("Failed to bind to address");

    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, source);

                // Handle received data
                let parsed_header_bytes = DNSHeader::deserialize(&mut buf);
                let parsed_question_bytes =
                    DNSQuestion::deserialize(&mut buf, parsed_header_bytes.question_count);

                // Construct DNS response
                let response = DNSResponse {
                    header: DNSHeader {
                        packet_id: parsed_header_bytes.packet_id,
                        qr_indicator: 1,
                        opcode: parsed_header_bytes.opcode,
                        authoritative_answer: 0,
                        truncation: 0,
                        recursion_desired: parsed_header_bytes.recursion_desired,
                        recursion_available: 0,
                        reserved: 0,
                        response_code: if parsed_header_bytes.opcode == 0 {
                            0
                        } else {
                            4
                        },
                        question_count: 1,
                        answer_record_count: 1,
                        authority_record_count: 0,
                        additional_record_count: 0,
                    },
                    question: DNSQuestion {
                        domain_name: parsed_question_bytes.domain_name.clone(),
                        query_type: DNSQueryType::A,
                        query_class: DNSQueryClass::IN,
                    },
                    answer: DNSAnswer {
                        name: parsed_question_bytes.domain_name.clone(),
                        typ: DNSQueryType::A,
                        class: DNSQueryClass::IN,
                        ttl: 60,
                        rdlength: 4,
                        rdata: [8, 8, 8, 8].to_vec(),
                    },
                }
                .serialize();

                // Send DNS response
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
