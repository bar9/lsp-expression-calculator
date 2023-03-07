use axum::extract::WebSocketUpgrade;
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(handler));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
   while let Some(msg) = socket.recv().await {
       let msg = if let Ok(msg) = msg {
           msg
       } else {
           return;
       };

       if socket.send(msg).await.is_err() {
           return;
       }
   }
}