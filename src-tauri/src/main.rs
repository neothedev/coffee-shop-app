use tauri::{generate_handler, Builder};
use tokio::runtime::Runtime;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use futures_util::stream::StreamExt; // Import StreamExt for the `split` method
use futures_util::sink::SinkExt; // Import SinkExt for the `send` method
use futures_util::stream::TryStreamExt; // Import TryStreamExt for the `try_next` method

fn main() {
    let runtime = Runtime::new().unwrap();
    let (tx, _rx) = broadcast::channel::<String>(100); // Broadcast channel for WebSocket messages
    let tx = Arc::new(Mutex::new(tx));

    // Start WebSocket server in a separate thread
    std::thread::spawn(move || {
        runtime.block_on(start_websocket_server(tx.clone()));
    });

    Builder::default()
        .invoke_handler(generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

async fn start_websocket_server(tx: Arc<Mutex<broadcast::Sender<String>>>) {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("WebSocket server running on ws://127.0.0.1:9001");

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let mut rx = tx.lock().unwrap().subscribe();

            let (mut write, mut read) = ws_stream.split(); // Use the split method

            // Handle incoming messages
            tokio::spawn(async move {
                while let Ok(msg) = rx.recv().await {
                    write.send(Message::Text(msg)).await.unwrap(); // Use SinkExt's `send` method
                }
            });

            // Handle outgoing messages
            while let Ok(Some(msg)) = read.try_next().await { // Use TryStreamExt's `try_next` method
                if let Message::Text(text) = msg {
                    tx.lock().unwrap().send(text).unwrap();
                }
            }
        });
    }
}