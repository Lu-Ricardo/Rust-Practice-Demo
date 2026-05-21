use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
fn main() {
    println!("开始连接服务器...");
    let stream = TcpStream::connect("127.0.0.1:8899").unwrap();
    send_data(&stream, b"Hello! It's from Client");
    let data = recive_data(&stream);
    if data == "Ok, It's from Server!" {
        send_data(&stream, b"Ok");
        println!("连接成功，目标地址: {}", &stream.peer_addr().unwrap());
        loop {
            let mut cmd = String::new();
            std::io::stdin().read_line(&mut cmd).expect("未检测到输入...");
            println!("发送结果：{}", send_data(&stream, &cmd.trim().as_bytes()));
        }
    }
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

fn recive_data(stream: &TcpStream) -> String {
    let mut stream = stream;
    let mut buffer = [0; 128];
    let size = stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&mut buffer[..size]).to_string()
}