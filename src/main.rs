mod antlr;
mod antlr_semantic_tree;
mod antlr_token_mapping;
mod lsp_token;
mod semantic_tree_type;
mod server;
mod utils;

use server::Backend;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    env_logger::init();
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
