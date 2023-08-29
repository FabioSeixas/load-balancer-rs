use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

struct ApiConnectionManager {
    next_server_port: u8,
}

impl ApiConnectionManager {
    fn get_next_server_port(&mut self) -> String {
        let mut port = String::from("300");
        port.push_str(&self.next_server_port.to_string());
        if self.next_server_port == 9 {
            self.next_server_port = 0;
        } else {
            self.next_server_port += 1;
        }
        port
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:7878").await.unwrap();
    let mut manager = ApiConnectionManager {
        next_server_port: 0,
    };

    loop {
        let mut socket = listener.accept().await.unwrap().0;
        let server_port = manager.get_next_server_port();

        println!("port {server_port}");

        tokio::spawn(async move {
            process(&mut socket, server_port).await;
        });
    }
}

async fn process(stream: &mut TcpStream, server_port: String) {
    let data = reqwest::get(format!("http://localhost:{server_port}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let length = data.len();
    let header = format!("HTTP/1.1 200 Ok\r\nContent-Length: {length}\r\n\r\n{data}");

    stream.write_all(header.as_bytes()).await.unwrap();

    println!("{:?}", stream.peer_addr());
}
