use tokio::net::TcpListener;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use async_tungstenite::tokio::accept_async;
use tracing::info;
use ws_stream_tungstenite::*;

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "runtime-agnostic")]
    use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

    tracing_subscriber::fmt().init();

    let listener = TcpListener::bind("127.0.0.1:9257").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    let (stream, _) = listener.accept().await.unwrap();
    let (read, write) = tokio::io::split(WsStream::new(accept_async(stream).await.unwrap()));
    #[cfg(feature = "runtime-agnostic")]
        let (read, write) = (read.compat(), write.compat_write());

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(read, write, socket).serve(service).await;
}