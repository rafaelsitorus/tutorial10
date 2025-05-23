use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    // Mengubah port ke 8080
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    println!("Connected to WebSocket server at ws://127.0.0.1:8080");
    println!("Type messages and press Enter to send. Ctrl+D to disconnect.");

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    loop {
        tokio::select! {
            // Membaca baris dari stdin dan mengirimnya ke server WebSocket
            line = stdin.next_line() => {
                match line {
                    Ok(Some(text)) => {
                        ws_sender.send(Message::text(text)).await?;
                    }
                    Ok(None) => {
                        // EOF pada stdin, klien selesai mengirim pesan
                        break;
                    }
                    Err(e) => {
                        eprintln!("Stdin read error: {}", e);
                        break;
                    }
                }
            }
            // Menerima pesan dari server WebSocket dan mencetaknya
            msg = ws_receiver.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            // Menambahkan awalan "From server: " sebelum mencetak
                            println!("From server: {}", text);
                        } else if msg.is_binary() {
                            println!("Received binary message (not displayed).");
                        } else if msg.is_ping() {
                            println!("Received PING.");
                        } else if msg.is_pong() {
                            println!("Received PONG.");
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                    None => {
                        // Server menutup koneksi
                        println!("Server disconnected.");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}