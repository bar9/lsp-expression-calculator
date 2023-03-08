extern crate core;

use std::io;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::Message::Text;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::{Error, Router};
use axum::routing::get;
use serde_json::{from_str, Map, Value};
use tower_lsp::{LanguageServer, LspService};
use tower_lsp::jsonrpc::{Request, RequestBuilder, Response, Result};
use tower_lsp::lsp_types::{CompletionItem, CompletionOptions, CompletionParams, CompletionResponse, DidChangeConfigurationParams, DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidChangeWorkspaceFoldersParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, ExecuteCommandOptions, ExecuteCommandParams, Hover, HoverContents, HoverParams, HoverProviderCapability, InitializedParams, InitializeParams, InitializeResult, MarkedString, MessageType, OneOf, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, WorkspaceEdit, WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities};
use tower_lsp::lsp_types::notification::Initialized;
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
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions::default()),
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
        // Ok(Some(Initialized))
        // Ok(Some(Initialized)
        // self.client
        //     .log_message(MessageType::INFO, "initialized!")
        //     .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(
                MarkedString::String("You're hovering!".to_string())
            ),
            range: None
        }))
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
        let mut method = String::from("");
        let res = if let Ok(Text(msg)) = msg {
            let deserialized = serde_json::from_value::<Request>(msg.clone().parse().unwrap());
            if let Ok(deserialized) = deserialized {
                method = deserialized.method().to_string();
                // println!("{:?}", deserialized);
                if let Ok(res) = service.call(deserialized).await {
                    println!("{:?}", res);
                    res
                } else {
                //     println!("{}", "ERROR: LSP call failed");
                    return;
                }
            } else {
            //     println!("{}", "ERROR: recv failed");
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
        //     println!("{}", "ERROR: recv failed");
            return;
        };


        if let Some(res) = res {
            println!("{:?}", res);
            if let Some(result) = res.result() {
                let result = result.to_string();
                let mut map = Map::new();
                map.insert(String::from("id"), Value::Number(res.id().to_string().parse().unwrap()));
                map.insert(String::from("jsonrpc"), Value::String(String::from("2.0")));
                map.insert(String::from("result"), from_str(&result[..]).unwrap());
                // map.insert(String::from("method"), Value::String(method));
                let obj = Value::Object(map);
                // map.insert("method", Value::String(String::from(res)));
                if socket.send(Message::Text(obj.to_string())).await.is_err() {
                    println!("{}", "ERROR: send failed");
                    // return
                }
            }
            // println!("{:?}", res);
            // if socket.send(Message::from(res)).await.is_err() {
            //     return;
            // }
        } else {
            // return;
        }
    }
}

