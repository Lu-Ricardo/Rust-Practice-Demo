use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
fn main() {
    let listener = TcpListener::bind("0.0.0.0:8899").unwrap();
    match wait_connect(&listener) {
        Ok(stream) => {
            loop {
                let data = recive_data(&stream);
                if data.is_empty() {
                    println!("客户端断开连接...");
                    break;
                }
                println!("接收到客户端数据：{}", data);
            }
        },
        Err(e) => println!("出现错误，{}", e.to_string())
    }
}

fn recive_data(stream: &TcpStream) -> String {
    let mut stream = stream;
    let mut buffer = [0; 128];
    let size = stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&mut buffer[..size]).to_string()
}

fn send_data(stream: &TcpStream, data: &[u8]) -> bool{
    let mut stream = stream;
    match stream.write(&data){
        Ok(_) => true,
        Err(e) => {
            println!("发送失败，{}", e.to_string());
            false
        }
    }
}

fn wait_connect(listener: &TcpListener) -> Result<TcpStream, std::io::Error> {
    println!("等待客户端连接");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let mut data = recive_data(&stream);
        println!("接收到数据: {}", data);
        if data == "Hello! It's from Client" {
            send_data(&stream, b"Ok, It's from Server!");
            data = recive_data(&stream);
            if data == "Ok" {
                println!("连接成功，目标地址:{}", stream.peer_addr().unwrap());
                return Ok(stream);
            }
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "无可用连接"))
}
