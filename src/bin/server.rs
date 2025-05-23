use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Subscribe to the broadcast channel
    let mut bcast_rx = bcast_tx.subscribe();

    // Split the WebSocket stream into a sender and receiver
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    loop {
        tokio::select! {
            // Handle incoming messages from the WebSocket client
            msg = ws_receiver.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("Received from {addr}: {text}");
                            // Broadcast the message to all connected clients
                            bcast_tx.send(format!("{addr}: {text}"))?;
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
            // Handle messages from the broadcast channel (other clients)
            msg = bcast_rx.recv() => {
                match msg {
                    Ok(msg) => {
                        // Send the broadcast message to this client
                        ws_sender.send(Message::text(msg)).await?;
                    }
                    Err(e) => {
                        eprintln!("Broadcast receive error for {addr}: {e}");
                        // If the channel is lagged, we might want to just continue or handle it
                        // more robustly. For simplicity, we'll break here.
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

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
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