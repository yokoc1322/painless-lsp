use crate::antlr::type_converter::SymbolName;
use crate::antlr_semantic_tree::{AntlrLeafRule, AntlrTreeNode};
use crate::lsp_token::{SemanticTokenConvertible, LEGEND_TYPE_MAP};
use crate::semantic_tree_type::{Leaf, TreeNode};
use tower_lsp::lsp_types::{SemanticToken, SemanticTokenType, SemanticTokens};

#[derive(Debug, Clone)]
struct Position {
    line: usize,
    col: usize,
}

impl SemanticTokenConvertible for AntlrTreeNode {
    fn to_semantic_tokens(&self) -> SemanticTokens {
        let leaf_nodes = self.pick_leaf_node();
        SemanticTokens {
            result_id: None,
            data: convert_semantic_tokens(leaf_nodes),
        }
    }
}

impl AntlrTreeNode {
    fn pick_leaf_node(&self) -> Vec<&Leaf<AntlrLeafRule>> {
        match &self {
            TreeNode::Leaf(leaf) => match leaf.rule.to_semantic_token_type() {
                Some(_) => vec![leaf],
                None => vec![],
            },
            TreeNode::Branch(branch) => branch
                .children
                .iter()
                .flat_map(|child| child.pick_leaf_node())
                .collect(),
        }
    }
}

fn convert_semantic_tokens(leafs: Vec<&Leaf<AntlrLeafRule>>) -> Vec<SemanticToken> {
    let mut prev_position = Position { line: 1, col: 0 };
    leafs
        .iter()
        .map(|token| {
            let (semantic_token, position) = token.to_semantic_token(&prev_position);
            prev_position = position;
            semantic_token
        })
        .filter(|token| token.is_some())
        .map(|token| token.unwrap())
        .collect()
}

impl Leaf<AntlrLeafRule> {
    fn to_semantic_token(&self, prev_position: &Position) -> (Option<SemanticToken>, Position) {
        let token_type = match self.rule.to_semantic_token_type() {
            Some(t) => t,
            None => return (None, prev_position.clone()),
        };
        let token_type_index = match LEGEND_TYPE_MAP.get(&token_type) {
            Some(i) => *i as u32,
            None => return (None, prev_position.clone()),
        };

        let delta_line = (self.line - prev_position.line) as u32;
        let delta_start = match delta_line {
            0 => (self.column_start - prev_position.col) as u32,
            _ => self.column_start as u32,
        };

        (
            Some(SemanticToken {
                delta_line,
                delta_start,
                length: self.token.len() as u32,
                token_type: token_type_index,
                token_modifiers_bitset: 0,
            }),
            Position {
                line: self.line,
                col: self.column_start,
            },
        )
    }
}

impl AntlrLeafRule {
    fn to_semantic_token_type(&self) -> Option<SemanticTokenType> {
        match self.0 {
            SymbolName::EOF => Some(SemanticTokenType::KEYWORD),
            SymbolName::IF => Some(SemanticTokenType::KEYWORD),
            SymbolName::IN => Some(SemanticTokenType::KEYWORD),
            SymbolName::ELSE => Some(SemanticTokenType::KEYWORD),
            SymbolName::WHILE => Some(SemanticTokenType::KEYWORD),
            SymbolName::DO => Some(SemanticTokenType::KEYWORD),
            SymbolName::FOR => Some(SemanticTokenType::KEYWORD),
            SymbolName::CONTINUE => Some(SemanticTokenType::KEYWORD),
            SymbolName::BREAK => Some(SemanticTokenType::KEYWORD),
            SymbolName::RETURN => Some(SemanticTokenType::KEYWORD),
            SymbolName::NEW => Some(SemanticTokenType::KEYWORD),
            SymbolName::TRY => Some(SemanticTokenType::KEYWORD),
            SymbolName::CATCH => Some(SemanticTokenType::KEYWORD),
            SymbolName::THROW => Some(SemanticTokenType::KEYWORD),
            SymbolName::THIS => Some(SemanticTokenType::KEYWORD),
            SymbolName::INSTANCEOF => Some(SemanticTokenType::KEYWORD),
            SymbolName::OCTAL => Some(SemanticTokenType::NUMBER),
            SymbolName::HEX => Some(SemanticTokenType::NUMBER),
            SymbolName::INTEGER => Some(SemanticTokenType::NUMBER),
            SymbolName::DECIMAL => Some(SemanticTokenType::NUMBER),
            SymbolName::STRING => Some(SemanticTokenType::STRING),
            SymbolName::REGEX => Some(SemanticTokenType::REGEXP),
            SymbolName::TRUE => Some(SemanticTokenType::KEYWORD),
            SymbolName::FALSE => Some(SemanticTokenType::KEYWORD),
            SymbolName::NULL => Some(SemanticTokenType::KEYWORD),
            SymbolName::PRIMITIVE => Some(SemanticTokenType::TYPE),
            SymbolName::DEF => Some(SemanticTokenType::TYPE),
            //
            SymbolName::VariableId => Some(SemanticTokenType::VARIABLE),
            SymbolName::FunctionId => Some(SemanticTokenType::FUNCTION),
            SymbolName::ClassId => Some(SemanticTokenType::CLASS),
            SymbolName::ParameterId => Some(SemanticTokenType::PARAMETER),
            //
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = include_str!("../assets/test_data/easy_sample.painless");
        // let input = include_str!("../assets/easy_sample.painless");
        let tree = crate::antlr_semantic_tree::generate_semantic_tree(input);
        println!("===debug====");
        println!("{:?}", tree);
        let tokens = tree.unwrap().to_semantic_tokens();
        println!("===result====");
        println!("{:?}", tokens);
    }

    #[test]
    fn test_for_parse() {
        let input = include_str!("../assets/test_data/for.painless");
        let tree = crate::antlr_semantic_tree::generate_semantic_tree(input).unwrap();
        println!("===debug====");
        println!("{}", serde_json::to_string(&tree).unwrap());
        // let tokens = tree.unwrap().to_semantic_tokens();
        // println!("===result====");
        // println!("{:?}", tokens);
    }
}
