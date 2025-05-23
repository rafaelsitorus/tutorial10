use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    ws_stream: WebSocketStream<TcpStream>, // 'mut' dihapus karena tidak diperlukan
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("Received from {addr}: {text}");
                            // Tambahkan alamat pengirim ke pesan yang akan disiarkan
                            bcast_tx.send(format!("{addr}: {}", text))?;
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("WebSocket error for {addr}: {e}");
                        break;
                    }
                    None => {
                        println!("Client {addr} disconnected.");
                        break;
                    }
                }
            }
            msg = bcast_rx.recv() => {
                match msg {
                    Ok(msg) => {
                        // Kirim pesan yang sudah diformat dari broadcast channel ke klien ini
                        ws_sender.send(Message::text(msg)).await?;
                    }
                    Err(e) => {
                        eprintln!("Broadcast receive error for {addr}: {e}");
                        // Dalam kasus error ini (misal: channel lagged), kita bisa putuskan koneksi
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    // Mengubah port ke 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            let ws_stream_result = ServerBuilder::new().accept(socket).await;

            match ws_stream_result {
                Ok((_req, ws_stream)) => {
                    if let Err(e) = handle_connection(addr, ws_stream, bcast_tx).await {
                        eprintln!("Error handling connection for {addr:?}: {e}");
                    }
                }
                Err(e) => {
                    eprintln!("WebSocket handshake error for {addr:?}: {e}");
                }
            }
        });
    }
}