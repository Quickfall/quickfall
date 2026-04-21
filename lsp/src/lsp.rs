use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use tower_lsp::{
    Client, LanguageServer,
    lsp_types::{
        CompletionOptions, HoverProviderCapability, InitializeParams, InitializeResult,
        InitializedParams, ServerCapabilities, ServerInfo, TextDocumentSyncCapability,
        TextDocumentSyncKind,
    },
};

#[derive(Debug)]
pub struct LSPBackend {
    lsp_client: Client,
    documents: Arc<Mutex<HashSet<String>>>,
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

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
}
