use std::collections::HashMap;
use std::{cell::LazyCell, fmt::Debug};
use tower_lsp::lsp_types::{SemanticTokenType, SemanticTokens};

pub const LEGEND_TYPE_ARRAY: &[SemanticTokenType] = &[
    SemanticTokenType::NAMESPACE,
    SemanticTokenType::TYPE,
    SemanticTokenType::CLASS,
    SemanticTokenType::ENUM,
    SemanticTokenType::INTERFACE,
    SemanticTokenType::STRUCT,
    SemanticTokenType::TYPE_PARAMETER,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::ENUM_MEMBER,
    SemanticTokenType::EVENT,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::METHOD,
    SemanticTokenType::MACRO,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::MODIFIER,
    SemanticTokenType::COMMENT,
    SemanticTokenType::STRING,
    SemanticTokenType::NUMBER,
    SemanticTokenType::REGEXP,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::DECORATOR,
];

pub const LEGEND_TYPE_MAP: LazyCell<HashMap<SemanticTokenType, usize>> = LazyCell::new(|| {
    LEGEND_TYPE_ARRAY
        .iter()
        .enumerate()
        .map(|(i, token)| (token.clone(), i))
        .collect()
});

pub trait SemanticTokenConvertible: Send + Sync + Debug {
    fn to_semantic_tokens(&self) -> SemanticTokens;
}
