use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use tower_lsp::{LspService, Server};

use crate::lsp::LSPBackend;

pub mod bridge;
pub mod diags;
pub mod lsp;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::build(|client| LSPBackend {
        lsp_client: client,
        documents: Arc::new(Mutex::new(HashSet::new())),
    })
    .finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}
