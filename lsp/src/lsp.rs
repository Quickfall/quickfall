use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use tower_lsp::{
    Client, LanguageServer,
    lsp_types::{
        CompletionOptions, DidOpenTextDocumentParams, HoverProviderCapability, InitializeParams,
        InitializeResult, InitializedParams, MessageType, ServerCapabilities, ServerInfo,
        TextDocumentSyncCapability, TextDocumentSyncKind,
    },
};

use crate::{bridge::check_for_file, diags::to_tower_diag};

#[derive(Debug)]
pub struct LSPBackend {
    pub lsp_client: Client,
    pub documents: Arc<Mutex<HashSet<String>>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for LSPBackend {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "quickfall-lsp".to_string(),
                version: Some("0.1".to_string()),
            }),
        })
    }

    async fn did_open(&self, param: DidOpenTextDocumentParams) {
        let uri = param.text_document.uri.to_string();
        let text = param.text_document.text;

        self.documents.lock().unwrap().insert(uri.clone());

        let mut diags = vec![];

        self.lsp_client
            .log_message(MessageType::ERROR, format!("Path: {}", uri))
            .await;

        for diag in check_for_file(uri.clone().replace("file://", "")) {
            diags.push(to_tower_diag(diag))
        }

        self.lsp_client
            .publish_diagnostics(uri.parse().unwrap(), diags, None)
            .await;
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
}
