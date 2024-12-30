use std::collections::HashMap;
use std::sync::RwLock;
use std::vec;

use log::debug;
use log::error;
use tower_lsp::jsonrpc::Result;
use tower_lsp::{lsp_types::*, Client, LanguageServer};

use crate::antlr_semantic_tree::generate_semantic_tree;
use crate::lsp_token::{SemanticTokenConvertible, LEGEND_TYPE_ARRAY};

type Path = String;

pub struct Backend {
    pub client: Client,
    trees: RwLock<HashMap<Path, Box<dyn SemanticTokenConvertible>>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        // Ok(InitializeResult::default())
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                        SemanticTokensRegistrationOptions {
                            text_document_registration_options: {
                                TextDocumentRegistrationOptions {
                                    document_selector: Some(vec![DocumentFilter {
                                        language: Some("painless".to_string()),
                                        scheme: Some("file".to_string()),
                                        pattern: None,
                                    }]),
                                }
                            },
                            semantic_tokens_options: SemanticTokensOptions {
                                work_done_progress_options: WorkDoneProgressOptions::default(),
                                legend: SemanticTokensLegend {
                                    token_types: LEGEND_TYPE_ARRAY.into(),
                                    token_modifiers: vec![],
                                },
                                range: Some(false),
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                            },
                            static_registration_options: StaticRegistrationOptions::default(),
                        },
                    ),
                ),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        debug!("debug initialized");
        self.client
            .log_message(MessageType::INFO, "server initialized!!!!!")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        debug!("file opened: {:?}", params.text_document.uri.to_path());
        self.on_change(
            &params.text_document.text,
            params.text_document.uri,
            Some(params.text_document.version),
        )
        .await
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        debug!("file changed: {:?}", params.text_document.uri.to_path());
        self.on_change(
            &params.content_changes[0].text,
            params.text_document.uri,
            Some(params.text_document.version),
        )
        .await
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        debug!(
            "semantic_tokens_full: {:?}",
            params.text_document.uri.to_path()
        );
        let trees_ref = match self.trees.read() {
            Ok(reader) => reader,
            Err(_) => return Ok(None), // TODO: error handling
        };

        let ret = match trees_ref.get(&params.text_document.uri.to_path()) {
            Some(tree) => Some(SemanticTokensResult::Tokens(tree.to_semantic_tokens())),
            None => None,
        };
        Ok(ret)
    }

    async fn semantic_tokens_full_delta(
        &self,
        params: SemanticTokensDeltaParams,
    ) -> Result<Option<SemanticTokensFullDeltaResult>> {
        debug!(
            "semantic_tokens_full_delta: {:?}",
            params.text_document.uri.to_path()
        );
        Ok(None)
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        debug!(
            "semantic_tokens_range: {:?}",
            params.text_document.uri.to_path()
        );
        Ok(None)
    }
}

impl Backend {
    pub fn new(client: Client) -> Backend {
        Backend {
            client,
            trees: RwLock::new(HashMap::new()),
        }
    }

    async fn on_change(&self, text: &str, uri: Url, _: Option<i32>) {
        debug!("on_change: {:?}", uri.to_path());
        let mut trees_writable_ref = match self.trees.write() {
            Ok(writer) => writer,
            Err(error) => {
                error!("error catching lock: {:?}", error);
                return;
            } // TODO: error handling
        };

        let tree = match generate_semantic_tree(text) {
            Ok(tree) => tree,
            Err(error) => {
                error!("error generating tree: {:?}", error);
                return;
            }
        };

        trees_writable_ref.insert(uri.to_path(), Box::new(tree));
        // log::debug!("tree: {:?}", *tree_writable_ref);

        debug!(
            "on_change:map_keys: {:?}",
            trees_writable_ref.keys().collect::<Vec<&Path>>()
        );
    }
}

trait UrlToPath {
    fn to_path(&self) -> Path;
}

impl UrlToPath for Url {
    fn to_path(&self) -> Path {
        self.path().to_string()
    }
}
