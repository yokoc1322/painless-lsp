use log::debug;
use tower_lsp::lsp_types::{SemanticToken, SemanticTokenType, SemanticTokens};

use crate::lsp_token::{SemanticTokenConvertible, LEGEND_TYPE_MAP};
use crate::pest_semantic_tree::{PestTokenTree, Rule};

struct Position {
    line: usize,
    col: usize,
}

impl SemanticTokenConvertible for PestTokenTree {
    fn to_semantic_tokens(&self) -> SemanticTokens {
        debug!("tree: {:?}", self);
        SemanticTokens {
            result_id: None,
            data: self.to_semantic_tokens_rec(Position { line: 1, col: 0 }).0,
        }
    }
}

impl PestTokenTree {
    fn to_semantic_tokens_rec(&self, prev_position: Position) -> (Vec<SemanticToken>, Position) {
        let (current_token, parent_position) = self.to_semantic_token(&prev_position);
        let child_tokens = self
            .children
            .iter()
            .fold(
                (vec![], prev_position),
                |(mut acc, prev_position), child| {
                    let (tokens, next_position) = child.to_semantic_tokens_rec(prev_position);
                    acc.extend(tokens);
                    (acc, next_position)
                },
            )
            .0;

        (
            current_token
                .into_iter()
                .chain(child_tokens.into_iter())
                .collect(),
            parent_position,
        )
    }

    fn to_semantic_token(&self, prev_position: &Position) -> (Option<SemanticToken>, Position) {
        let token_type = match self.rule.to_semantic_token_type() {
            Some(t) => t,
            None => return (None, self.to_position()),
        };

        let token_type_index = match LEGEND_TYPE_MAP.get(&token_type) {
            Some(i) => *i as u32,
            None => {
                debug!("unknown semantic token type: {:?}", token_type);
                return (None, self.to_position()); // TODO: 型で制限をいれるべき
            }
        };

        let delta_line = (self.line - prev_position.line) as u32;
        let delta_start = match delta_line {
            0 => (self.start - prev_position.col) as u32,
            _ => self.start as u32,
        };

        (
            Some(SemanticToken {
                delta_line,
                delta_start,
                length: (self.end - self.start) as u32,
                token_type: token_type_index,
                token_modifiers_bitset: 0,
            }),
            self.to_position(),
        )
    }

    fn to_position(&self) -> Position {
        Position {
            line: self.line,
            col: self.start,
        }
    }
}

impl Rule {
    fn to_semantic_token_type(&self) -> Option<SemanticTokenType> {
        match self {
            Rule::r#type => Some(SemanticTokenType::TYPE),
            // Rule::class => Some(SemanticTokenType::CLASS),
            // Rule::parameter => Some(SemanticTokenType::PARAMETER),
            Rule::identifier => Some(SemanticTokenType::VARIABLE),
            // Rule::property => Some(SemanticTokenType::PROPERTY),
            // Rule::function => Some(SemanticTokenType::FUNCTION),
            // Rule::method => Some(SemanticTokenType::METHOD),
            // Rule::keyword => Some(SemanticTokenType::KEYWORD),
            // Rule::comment => Some(SemanticTokenType::COMMENT),
            Rule::string => Some(SemanticTokenType::STRING),
            Rule::integer => Some(SemanticTokenType::NUMBER),
            Rule::float => Some(SemanticTokenType::NUMBER),
            // Rule::regexp => Some(SemanticTokenType::REGEXP),
            // Rule::operator => Some(SemanticTokenType::OPERATOR),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_one_line() {
        let tree = PestTokenTree {
            rule: Rule::program,
            value: "int i = 10".to_string(),
            start: 0,
            end: 10,
            line: 1,
            children: vec![
                PestTokenTree {
                    rule: Rule::expr,
                    value: "int i = 10".to_string(),
                    start: 0,
                    end: 10,
                    line: 1,
                    children: vec![PestTokenTree {
                        rule: Rule::declare_with_assign_expr,
                        value: "int i = 10".to_string(),
                        start: 0,
                        end: 10,
                        line: 1,
                        children: vec![
                            PestTokenTree {
                                rule: Rule::r#type,
                                value: "int".to_string(),
                                start: 0,
                                end: 3,
                                line: 1,
                                children: vec![],
                            },
                            PestTokenTree {
                                rule: Rule::identifier,
                                value: "i".to_string(),
                                start: 4,
                                end: 5,
                                line: 1,
                                children: vec![],
                            },
                            PestTokenTree {
                                rule: Rule::expr,
                                value: "10".to_string(),
                                start: 8,
                                end: 10,
                                line: 1,
                                children: vec![PestTokenTree {
                                    rule: Rule::terms,
                                    value: "10".to_string(),
                                    start: 8,
                                    end: 10,
                                    line: 1,
                                    children: vec![PestTokenTree {
                                        rule: Rule::integer,
                                        value: "10".to_string(),
                                        start: 8,
                                        end: 10,
                                        line: 1,
                                        children: vec![],
                                    }],
                                }],
                            },
                        ],
                    }],
                },
                PestTokenTree {
                    rule: Rule::EOI,
                    value: "".to_string(),
                    start: 10,
                    end: 10,
                    line: 1,
                    children: vec![],
                },
            ],
        };

        let tokens = tree.to_semantic_tokens();
        assert_eq!(
            tokens.data,
            vec![
                SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: 3,
                    token_type: SemanticTokenType::TYPE.to_index(),
                    token_modifiers_bitset: 0
                },
                SemanticToken {
                    delta_line: 0,
                    delta_start: 4,
                    length: 1,
                    token_type: SemanticTokenType::VARIABLE.to_index(),
                    token_modifiers_bitset: 0
                },
                SemanticToken {
                    delta_line: 0,
                    delta_start: 4,
                    length: 2,
                    token_type: SemanticTokenType::NUMBER.to_index(),
                    token_modifiers_bitset: 0
                }
            ]
        );
    }

    trait ToIndex {
        fn to_index(&self) -> u32;
    }

    impl ToIndex for SemanticTokenType {
        fn to_index(&self) -> u32 {
            LEGEND_TYPE_MAP.get(self).unwrap().clone() as u32
        }
    }
}
