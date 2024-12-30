use antlr_rust::token::Token;
use antlr_rust::tree::TerminalNode;
use better_any::TidExt;
use serde::Serialize;

use crate::semantic_tree_type::{Branch, Leaf, TreeNode};
use std::rc::Rc;

use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};

use crate::antlr::generated::painlessparser::{PainlessParserContext, PainlessParserContextType};
use crate::antlr::generated::{painlesslexer::PainlessLexer, painlessparser::PainlessParser};
use crate::antlr::type_converter::{get_rule, get_symbol_by_index, RuleName, SymbolName};

#[derive(Debug, PartialEq, Serialize)]
pub struct AntlrLeafRule(pub SymbolName);
#[derive(Debug, PartialEq, Serialize)]
pub struct AntlrBranchRule(pub RuleName);
pub type AntlrTreeNode = TreeNode<AntlrLeafRule, AntlrBranchRule>;

struct ParseContext<'a> {
    parent_rule: Option<&'a RuleName>,
}

pub fn generate_semantic_tree(text: &str) -> Result<AntlrTreeNode, String> {
    let input = text.as_bytes();
    let input_stream = InputStream::new(input);
    let lexer = PainlessLexer::new(input_stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = PainlessParser::new(token_stream);

    let tree = match parser.source() {
        Ok(tree) => tree,
        Err(e) => return Err(format!("Error: {:?}", e)),
    };

    let root = generate_semantic_tree_recursive(tree, ParseContext { parent_rule: None })?;
    return Ok(root);
}

fn generate_semantic_tree_recursive<'a>(
    tree: Rc<dyn PainlessParserContext<'a> + 'a>,
    context: ParseContext,
) -> Result<AntlrTreeNode, String> {
    if let Ok(leaf) = tree
        .clone()
        .downcast_rc::<TerminalNode<'a, PainlessParserContextType>>()
    {
        let symbol = leaf.symbol.clone();
        let token_text = symbol.get_text();
        let token_type_index = symbol.get_token_type();
        let rule = get_symbol_by_index(token_type_index, context.parent_rule)?;

        return Ok(TreeNode::Leaf(Leaf {
            rule: AntlrLeafRule(rule),
            column_start: symbol.get_column() as usize,
            column_end: (symbol.get_column() as usize) + token_text.len() as usize,
            line: symbol.get_line() as usize,
            token: token_text.to_string(),
        }));
    }

    let rule = get_rule(tree.clone())?;
    let children = tree
        .get_children()
        .map(|child| {
            generate_semantic_tree_recursive(
                child,
                ParseContext {
                    parent_rule: Some(&rule),
                },
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(TreeNode::Branch(Branch {
        rule: AntlrBranchRule(rule),
        text: tree.get_text().to_string(),
        text_start: tree.start().start as usize,
        text_end: tree.stop().stop as usize,
        children,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_semantic_tree_valid_input_easy() {
        let input = include_str!("../assets/test_data/easy_sample.painless");
        println!("{}", input);

        let result = generate_semantic_tree(input);
        println!("{:?}", result);
        // assert!(result.is_ok());
        // let tree = result.unwrap();
        // assert_eq!(tree.rule, "expected_rule");
        // assert_eq!(tree.value, "expected_value");
        // assert_eq!(tree.start, 0);
        // assert_eq!(tree.end, input.len());
        // assert_eq!(tree.line, 1);
        // assert!(tree.children.is_empty());
    }

    #[test]
    fn test_generate_semantic_tree_valid_input() {
        let input = include_str!("../assets/test_data/sample.painless");
        println!("{}", input);

        let result = generate_semantic_tree(input);
        // println!("{:?}", result);
        // assert!(result.is_ok());
        // let tree = result.unwrap();
        // assert_eq!(tree.rule, "expected_rule");
        // assert_eq!(tree.value, "expected_value");
        // assert_eq!(tree.start, 0);
        // assert_eq!(tree.end, input.len());
        // assert_eq!(tree.line, 1);
        // assert!(tree.children.is_empty());
    }
}
