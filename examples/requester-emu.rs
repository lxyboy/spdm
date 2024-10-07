use spdm::requester::Requester;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:2345").unwrap();
    let send_buffer = &mut [0u8; 4096][..];
    let recv_buffer = &mut [0u8; 4096][..];

    let mut requester = Requester::new();

    loop {
        let (need_write, need_read) = requester.vca().unwrap();
        
    }

    let ret = stream.write(&[1]).unwrap();
    let ret = stream.read(buffer);
}
