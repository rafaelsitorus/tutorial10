use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    // Split the WebSocket stream into a sender and receiver
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    loop {
        tokio::select! {
            // Read line from stdin and send it to the WebSocket server
            line = stdin.next_line() => {
                match line {
                    Ok(Some(text)) => {
                        ws_sender.send(Message::text(text)).await?;
                    }
                    Ok(None) => {
                        // EOF on stdin, client is done sending messages
                        break;
                    }
                    Err(e) => {
                        eprintln!("Stdin read error: {e}");
                        break;
                    }
                }
            }
            // Receive message from the WebSocket server and print it
            msg = ws_receiver.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("{}", text);
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("WebSocket error: {e}");
                        break;
                    }
                    None => {
                        // Server closed the connection
                        println!("Server disconnected.");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}