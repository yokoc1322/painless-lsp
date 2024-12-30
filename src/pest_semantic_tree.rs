use pest::error::Error;
use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use std::fmt::{Display, Formatter};

use crate::utils::ScopeFunc;

pub type PestTokenTree = TokenTree<Rule>;

pub fn generate_semantic_tree(text: &str) -> Result<PestTokenTree, String> {
    let pairs = parse_expression(text);
    match pairs {
        Ok(mut pairs) => {
            if let Some(program_token) = pairs.next() {
                let token = TokenTree::from_pair(program_token);
                return Ok(token);
            } else {
                return Err("No program token found".to_string());
            }
        }
        Err(error) => Err(error.to_string()),
    }
}

#[derive(Parser)]
#[grammar = "assets/painless_grammar.pest"]
struct ExpressionParser;

fn parse_expression(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    let pairs = ExpressionParser::parse(Rule::program, input);
    return pairs;
}

#[derive(Debug, PartialEq)]
pub struct TokenTree<R> {
    pub rule: R,
    pub value: String,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub children: Vec<TokenTree<R>>,
}

impl PestTokenTree {
    fn from_pair(pair: Pair<Rule>) -> PestTokenTree {
        let rule = pair.as_rule();
        let pos = pair.line_col().tap_owned(|(line, col)| (line, col - 1));
        let value = String::from(pair.as_span().as_str());

        let span = pair.as_span();
        let start = pos.1;
        let end = pos.1 + span.end() - span.start();
        let line = pos.0;

        let children: Vec<PestTokenTree> = pair
            .clone()
            .into_inner()
            .map(|pair| TokenTree::from_pair(pair))
            .collect();

        TokenTree {
            rule,
            value,
            start,
            end,
            line,
            children,
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use testutil::{assert_token_tree, RawTokenTree};

    mod tests_declaration {
        use super::*;

        #[test]
        fn test_declaration_can_parse_one_line() {
            let input = "int i = 10";
            let expected = vec![RawTokenTree {
                rule: Rule::program,
                value: "int i = 10",
                start: 0,
                end: 10,
                line: 1,
                children: vec![
                    RawTokenTree {
                        rule: Rule::expr,
                        value: "int i = 10",
                        start: 0,
                        end: 10,
                        line: 1,
                        children: vec![RawTokenTree {
                            rule: Rule::declare_with_assign_expr,
                            value: "int i = 10",
                            start: 0,
                            end: 10,
                            line: 1,
                            children: vec![
                                RawTokenTree {
                                    rule: Rule::r#type,
                                    value: "int",
                                    start: 0,
                                    end: 3,
                                    line: 1,
                                    children: vec![],
                                },
                                RawTokenTree {
                                    rule: Rule::identifier,
                                    value: "i",
                                    start: 4,
                                    end: 5,
                                    line: 1,
                                    children: vec![],
                                },
                                RawTokenTree {
                                    rule: Rule::expr,
                                    value: "10",
                                    start: 8,
                                    end: 10,
                                    line: 1,
                                    children: vec![RawTokenTree {
                                        rule: Rule::terms,
                                        value: "10",
                                        start: 8,
                                        end: 10,
                                        line: 1,
                                        children: vec![RawTokenTree {
                                            rule: Rule::integer,
                                            value: "10",
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
                    RawTokenTree {
                        rule: Rule::EOI,
                        value: "",
                        start: 10,
                        end: 10,
                        line: 1,
                        children: vec![],
                    },
                ],
            }];

            assert_token_tree(input, expected);
        }

        #[test]
        fn test_declaration_can_parse_multi_line() {
            let input = "int i = 10\nint abc = 2000";
            let expected = vec![RawTokenTree {
                rule: Rule::program,
                value: "int i = 10\nint abc = 2000",
                start: 0,
                end: 25,
                line: 1,
                children: vec![
                    RawTokenTree {
                        rule: Rule::expr,
                        value: "int i = 10",
                        start: 0,
                        end: 10,
                        line: 1,
                        children: vec![RawTokenTree {
                            rule: Rule::declare_with_assign_expr,
                            value: "int i = 10",
                            start: 0,
                            end: 10,
                            line: 1,
                            children: vec![
                                RawTokenTree {
                                    rule: Rule::r#type,
                                    value: "int",
                                    start: 0,
                                    end: 3,
                                    line: 1,
                                    children: vec![],
                                },
                                RawTokenTree {
                                    rule: Rule::identifier,
                                    value: "i",
                                    start: 4,
                                    end: 5,
                                    line: 1,
                                    children: vec![],
                                },
                                RawTokenTree {
                                    rule: Rule::expr,
                                    value: "10",
                                    start: 8,
                                    end: 10,
                                    line: 1,
                                    children: vec![RawTokenTree {
                                        rule: Rule::terms,
                                        value: "10",
                                        start: 8,
                                        end: 10,
                                        line: 1,
                                        children: vec![RawTokenTree {
                                            rule: Rule::integer,
                                            value: "10",
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
                    RawTokenTree {
                        rule: Rule::expr,
                        value: "int abc = 2000",
                        start: 0,
                        end: 14,
                        line: 2,
                        children: vec![RawTokenTree {
                            rule: Rule::declare_with_assign_expr,
                            value: "int abc = 2000",
                            start: 0,
                            end: 14,
                            line: 2,
                            children: vec![
                                RawTokenTree {
                                    rule: Rule::r#type,
                                    value: "int",
                                    start: 0,
                                    end: 3,
                                    line: 2,
                                    children: vec![],
                                },
                                RawTokenTree {
                                    rule: Rule::identifier,
                                    value: "abc",
                                    start: 4,
                                    end: 7,
                                    line: 2,
                                    children: vec![],
                                },
                                RawTokenTree {
                                    rule: Rule::expr,
                                    value: "2000",
                                    start: 10,
                                    end: 14,
                                    line: 2,
                                    children: vec![RawTokenTree {
                                        rule: Rule::terms,
                                        value: "2000",
                                        start: 10,
                                        end: 14,
                                        line: 2,
                                        children: vec![RawTokenTree {
                                            rule: Rule::integer,
                                            value: "2000",
                                            start: 10,
                                            end: 14,
                                            line: 2,
                                            children: vec![],
                                        }],
                                    }],
                                },
                            ],
                        }],
                    },
                    RawTokenTree {
                        rule: Rule::EOI,
                        value: "",
                        start: 14,
                        end: 14,
                        line: 2,
                        children: vec![],
                    },
                ],
            }];

            assert_token_tree(input, expected);
        }

        #[test]
        fn test_function_can_parse_single_line() {
            let input = "boolean isNegative(def x) { x < 0 }";
            let abc = 10;
            let expected = vec![RawTokenTree {
                rule: Rule::program,
                value: "boolean isNegative(def x) { x < 0 }",
                start: 0,
                end: 35,
                line: 1,
                children: vec![
                    RawTokenTree {
                        rule: Rule::expr,
                        value: "boolean isNegative(def x) { x < 0 }",
                        start: 0,
                        end: 35,
                        line: 1,
                        children: vec![RawTokenTree {
                            rule: Rule::function,
                            value: "boolean isNegative(def x) { x < 0 }",
                            start: 0,
                            end: 35,
                            line: 1,
                            children: vec![
                                RawTokenTree {
                                    rule: Rule::return_type,
                                    value: "boolean",
                                    start: 0,
                                    end: 7,
                                    line: 1,
                                    children: vec![RawTokenTree {
                                        rule: Rule::r#type,
                                        value: "boolean",
                                        start: 0,
                                        end: 7,
                                        line: 1,
                                        children: vec![],
                                    }],
                                },
                                RawTokenTree {
                                    rule: Rule::function_name,
                                    value: "isNegative",
                                    start: 8,
                                    end: 18,
                                    line: 1,
                                    children: vec![RawTokenTree {
                                        rule: Rule::identifier,
                                        value: "isNegative",
                                        start: 8,
                                        end: 18,
                                        line: 1,
                                        children: vec![],
                                    }],
                                },
                                RawTokenTree {
                                    rule: Rule::parameters,
                                    value: "def x",
                                    start: 19,
                                    end: 24,
                                    line: 1,
                                    children: vec![RawTokenTree {
                                        rule: Rule::parameter,
                                        value: "def x",
                                        start: 19,
                                        end: 24,
                                        line: 1,
                                        children: vec![RawTokenTree {
                                            rule: Rule::identifier,
                                            value: "x",
                                            start: 23,
                                            end: 24,
                                            line: 1,
                                            children: vec![],
                                        }],
                                    }],
                                },
                                RawTokenTree {
                                    rule: Rule::function_body,
                                    value: "{ x < 0 }",
                                    start: 26,
                                    end: 35,
                                    line: 1,
                                    children: vec![RawTokenTree {
                                        rule: Rule::expr,
                                        value: "x < 0 ",
                                        start: 28,
                                        end: 34,
                                        line: 1,
                                        children: vec![RawTokenTree {
                                            rule: Rule::dyadic_expr,
                                            value: "x < 0 ",
                                            start: 28,
                                            end: 34,
                                            line: 1,
                                            children: vec![
                                                RawTokenTree {
                                                    rule: Rule::terms,
                                                    value: "x ",
                                                    start: 28,
                                                    end: 30,
                                                    line: 1,
                                                    children: vec![RawTokenTree {
                                                        rule: Rule::identifier,
                                                        value: "x",
                                                        start: 28,
                                                        end: 29,
                                                        line: 1,
                                                        children: vec![],
                                                    }],
                                                },
                                                RawTokenTree {
                                                    rule: Rule::verb,
                                                    value: "<",
                                                    start: 30,
                                                    end: 31,
                                                    line: 1,
                                                    children: vec![],
                                                },
                                                RawTokenTree {
                                                    rule: Rule::expr,
                                                    value: "0 ",
                                                    start: 32,
                                                    end: 34,
                                                    line: 1,
                                                    children: vec![RawTokenTree {
                                                        rule: Rule::terms,
                                                        value: "0 ",
                                                        start: 32,
                                                        end: 34,
                                                        line: 1,
                                                        children: vec![RawTokenTree {
                                                            rule: Rule::integer,
                                                            value: "0",
                                                            start: 32,
                                                            end: 33,
                                                            line: 1,
                                                            children: vec![],
                                                        }],
                                                    }],
                                                },
                                            ],
                                        }],
                                    }],
                                },
                            ],
                        }],
                    },
                    RawTokenTree {
                        rule: Rule::EOI,
                        value: "",
                        start: 35,
                        end: 35,
                        line: 1,
                        children: vec![],
                    },
                ],
            }];
            assert_token_tree(input, expected);
        }

        // #[test]
        // fn test_function_can_parse_multi_line() {
        //     let input = "boolean isNegative(def x) {\n x < 0\n }";
        //     let expected = vec![];
        //     assert_token_tree(input, expected);
        // }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod testutil {
    use super::*;

    use pretty_assertions::assert_eq;
    pub struct RawTokenTree<'a> {
        pub rule: Rule,
        pub value: &'a str,
        pub start: usize,
        pub end: usize,
        pub line: usize,
        pub children: Vec<RawTokenTree<'a>>,
    }

    impl PestTokenTree {
        fn from_raw(raw: RawTokenTree) -> PestTokenTree {
            PestTokenTree {
                rule: raw.rule,
                value: raw.value.to_string(),
                start: raw.start,
                end: raw.end,
                line: raw.line,
                children: raw
                    .children
                    .into_iter()
                    .map(|raw| TokenTree::from_raw(raw))
                    .collect(),
            }
        }
    }

    pub fn assert_token_tree(input: &str, expected: Vec<RawTokenTree<'_>>) {
        let result = parse_expression(input);
        assert!(
            result.is_ok(),
            "Failed to parse input. because: {:?}",
            result.err().unwrap()
        );

        let nodes = result
            .unwrap()
            .map(|pair| TokenTree::from_pair(pair))
            .collect::<Vec<PestTokenTree>>();
        let expected_nodes = expected
            .into_iter()
            .map(|raw| TokenTree::from_raw(raw))
            .collect::<Vec<PestTokenTree>>();

        assert_eq!(nodes, expected_nodes);
    }
}
