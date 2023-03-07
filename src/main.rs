use std::io;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::Message::Text;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::{Error, Router};
use axum::routing::get;
use serde_json::Value;
use tower_lsp::{LanguageServer, LspService};
use tower_lsp::jsonrpc::{Request, RequestBuilder, Response, Result};
use tower_lsp::lsp_types::{CompletionItem, CompletionOptions, CompletionParams, CompletionResponse, DidChangeConfigurationParams, DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidChangeWorkspaceFoldersParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, ExecuteCommandOptions, ExecuteCommandParams, InitializedParams, InitializeParams, InitializeResult, MessageType, OneOf, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, WorkspaceEdit, WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities};
use tower_service::Service;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(handler));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

struct Backend;

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_string()],
                    ..Default::default()
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    ..Default::default()
                }),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        // self.client
        //     .log_message(MessageType::INFO, "initialized!")
        //     .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        // self.client
        //     .log_message(MessageType::INFO, "workspace folders changed!")
        //     .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        // self.client
        //     .log_message(MessageType::INFO, "configuration changed!")
        //     .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        // self.client
        //     .log_message(MessageType::INFO, "watched files have changed!")
        //     .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        // self.client
        //     .log_message(MessageType::INFO, "command executed!")
        //     .await;

        // match self.client.apply_edit(WorkspaceEdit::default()).await {
        //     Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
        //     Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
        //     Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        // }

        Ok(None)
    }

    async fn did_open(&self, _: DidOpenTextDocumentParams) {
        // self.client
        //     .log_message(MessageType::INFO, "file opened!")
        //     .await;
    }

    async fn did_change(&self, _: DidChangeTextDocumentParams) {
        // self.client
        //     .log_message(MessageType::INFO, "file changed!")
        //     .await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        // self.client
        //     .log_message(MessageType::INFO, "file saved!")
        //     .await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        // self.client
        //     .log_message(MessageType::INFO, "file closed!")
        //     .await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }
}

async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // const language_server: LspService<S> = create_language_server();
    //TODO construct the server here, call it with .call()
    let (mut service, _) = LspService::new(|_| Backend);

    while let Some(msg) = socket.recv().await {
        let res = if let Ok(Text(msg)) = msg {
            let deserialized = serde_json::from_value::<Request>(msg.clone().parse().unwrap());
            if let Ok(deserialized) = deserialized {
                // println!("{:?}", deserialized);
                let res = service.call(deserialized).await.unwrap();
                println!("{:?}", res);
                res
            } else {
                return;
            }
            // println!("{:?}", deserialized);
            // if let Text(text) = msg.clone() {
            //     println!("{:?}", text);
            // }
            //     .await
            //     .unwrap();
            // Text(msg)
        } else {
            return;
        };

        println!("{:?}", String::from(res.unwrap()));

        // if let Some(Text(res)) = res {
        //     if socket.send(Message::from(res)).await.is_err() {
        //         return;
        //     }
        // }
    }
}

