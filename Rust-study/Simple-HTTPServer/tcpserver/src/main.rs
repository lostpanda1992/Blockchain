use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3001").unwrap(); //绑定本地端口
    println!("Running on port 3001");

    // let result = listener.accept().unwrap(); //只接受一次
    // 持续监听端口
    for stream in listener.incoming() {
        let mut stream = stream.unwrap(); //可变流
        println!("Connecting established");
        let mut buffer = [0; 1024]; //1024 初始化全为 0

        stream.read(&mut buffer).unwrap(); //读取数据 可变取地址
        stream.write(&mut buffer).unwrap(); //直接返回

    }
}
