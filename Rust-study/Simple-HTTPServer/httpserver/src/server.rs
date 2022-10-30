//主要功能程序
use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        for stream in connection_listener.incoming(){
            let mut stream = stream.unwrap();
            println!("Connecting established");
            
            let mut read_buffer = [0;200];
            stream.read(&mut read_buffer).unwrap();
            //HttpRequest实现了from方法就可以调用into来进行类型转换
            let req:HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::router(req, &mut stream);
        }
    }
}