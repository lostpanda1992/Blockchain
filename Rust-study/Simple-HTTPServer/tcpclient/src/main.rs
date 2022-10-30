use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
fn main() {
    let mut stream = TcpStream::connect("localhost:3001").unwrap(); //连接3001端口
    stream.write("Hello, world!".as_bytes()).unwrap(); 

    //从服务器接受消息
    let mut buffer = [0; 15];
    stream.read(&mut buffer).unwrap(); 

    println!("response from server{:?}", str::from_utf8(&buffer[..]).unwrap());
}
