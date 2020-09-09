mod connection;

use crate::client::connection::{ConnectionAddr, ConnectionInfo};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::ops::Add;

pub struct Client {
    connection_info: ConnectionInfo,
}

impl Client {
    pub fn new(host: String, port: u16) -> Self {
        return Client {
            connection_info: ConnectionInfo {
                addr: ConnectionAddr { host, port },
            },
        };
    }

    pub fn execute(&self, command: String) {
        let addr = self
            .connection_info
            .addr
            .host
            .to_string()
            .add(":")
            .add(self.connection_info.addr.port.to_string().as_str());
        println!("{}", addr);
        let stream = TcpStream::connect(addr).expect("Connect beanstalkd failed");
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        writer.write_all(command.as_bytes()).expect("Write error!");
        writer.flush().expect("Write flush error!");

        let mut response = String::new();
        reader.read_line(&mut response).expect("Read error");
        let header: Vec<&str> = response.trim().split(" ").collect();
        let code = header[0];
        let len = header[1];
        println!("header:\ncode: {}, len: {}", code, len);
    }
}
