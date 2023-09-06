use bytes::{Bytes, BufMut};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    oneshot,
};

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

struct ChannelMessage {
    server_port: String,
    responder: oneshot::Sender<Bytes>,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();

    let http_client = reqwest::Client::new();

    let (sender, mut receiver): (Sender<ChannelMessage>, Receiver<_>) = mpsc::channel(100);

    let mut manager = ApiConnectionManager {
        next_server_port: 0,
    };

    loop {
        tokio::select! {
            maybe_socket = listener.accept() => {
                let current_time = std::time::Instant::now();
                let (mut socket, _) = maybe_socket.unwrap();
                let (oneshot_sender, oneshot_receiver): (oneshot::Sender<Bytes>, oneshot::Receiver<Bytes>) = oneshot::channel();
                sender.send(ChannelMessage { server_port: manager.get_next_server_port(), responder: oneshot_sender }).await.unwrap();
                tokio::spawn(async move {
                    process(&mut socket, oneshot_receiver).await;
                    let elapsed = current_time.elapsed().as_secs_f64();
                    println!("elapsed: {} ms", elapsed * 1000.0);
                });
            },
            maybe_message = receiver.recv() => {
                if let Some(message) = maybe_message {
                    let respose_bytes = http_client.get(format!("http://node-server:{}", message.server_port))
                        .send()
                        .await
                        .unwrap()
                        .bytes()
                        .await
                        .unwrap();
                    message.responder.send(respose_bytes).unwrap();
                }
            }
        };
    }
}

async fn process(stream: &mut TcpStream, receiver: oneshot::Receiver<Bytes>) {
    match receiver.await {
        Ok(value) => {
            let length = value.len();
            let header = format!("HTTP/1.1 200 Ok\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n", length);

            let mut buffer = vec![];
            buffer.put(header.as_bytes());

            for byte in value {
                buffer.put_u8(byte);
            }
            println!("{:?}", String::from_utf8(buffer.clone()).unwrap());
            stream.write_all(&buffer).await.unwrap();
        }
        Err(_) => {
            let header = format!("HTTP/1.1 500 Ok\r\n\r\n");
            stream.write_all(header.as_bytes()).await.unwrap();
        }
    };
    stream.shutdown().await.unwrap();
}
