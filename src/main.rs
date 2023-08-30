use bytes::Bytes;
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
    let listener = TcpListener::bind("localhost:7878").await.unwrap();

    let http_client = reqwest::Client::new();

    let (sender, mut receiver): (Sender<ChannelMessage>, Receiver<_>) = mpsc::channel(100);

    let mut manager = ApiConnectionManager {
        next_server_port: 0,
    };

    loop {
        tokio::select! {
            maybe_socket = listener.accept() => {
                let (mut socket, _) = maybe_socket.unwrap();
                let (oneshot_sender, oneshot_receiver): (oneshot::Sender<Bytes>, oneshot::Receiver<Bytes>) = oneshot::channel();
                sender.send(ChannelMessage { server_port: manager.get_next_server_port(), responder: oneshot_sender }).await.unwrap();
                tokio::spawn(async move {
                    process(&mut socket, oneshot_receiver).await
                });
            },
            maybe_message = receiver.recv() => {
                if let Some(message) = maybe_message {
                    let respose_bytes = http_client.get(format!("http://localhost:{}", message.server_port))
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
            println!("{:?}", value);
            let length = value.len();
            let header = format!("HTTP/1.1 200 Ok\r\nContent-Length: {}\r\n\r\n", length);

            stream.write_all(header.as_bytes()).await.unwrap();

            for byte in value {
                stream.write_u8(byte).await.unwrap();
            }
        }
        Err(_) => {
            let header = format!("HTTP/1.1 500 Ok\r\n\r\n");
            stream.write_all(header.as_bytes()).await.unwrap();
        }
    };

    //
    // println!("{:?}", stream.peer_addr());
}
