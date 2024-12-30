use std::rc::Rc;

use better_any::TidExt;
use serde::Serialize;

use crate::antlr::generated::painlessparser::ruleNames;
use crate::antlr::generated::painlessparser::_SYMBOLIC_NAMES;

use super::generated::painlessparser::DstatementContextAll;
use super::generated::painlessparser::PainlessParserContext;
use super::generated::painlessparser::PrimaryContextAll;
use super::generated::painlessparser::RstatementContextAll;

type BaseTree<'a> = Rc<dyn PainlessParserContext<'a> + 'a>;

pub fn get_symbol_by_index(
    index: isize,
    parent_rule: Option<&RuleName>,
) -> Result<SymbolName, String> {
    let index = if index == -1 {
        return Ok(SymbolName::EOF);
    } else {
        index as usize
    };

    if let Some(Some(name)) = _SYMBOLIC_NAMES.get(index) {
        SymbolName::from_str(name)
            .map(|name| match name {
                SymbolName::ID => match parent_rule {
                    Some(RuleName::Function) => SymbolName::FunctionId,
                    Some(RuleName::Parameters) => SymbolName::ParameterId,
                    Some(RuleName::Type) => SymbolName::ClassId,
                    Some(RuleName::Declvar) => SymbolName::VariableId,
                    Some(RuleName::Trap) => SymbolName::VariableId,
                    Some(RuleName::Refcasttype) => SymbolName::ClassId,
                    Some(RuleName::Lamtype) => SymbolName::VariableId,
                    Some(RuleName::Funcref) => SymbolName::FunctionId,
                    Some(RuleName::Rstatement(RstatementRuleLabel::Each)) => SymbolName::VariableId,
                    Some(RuleName::Rstatement(RstatementRuleLabel::Ineach)) => {
                        SymbolName::VariableId
                    }
                    Some(RuleName::Primary(PrimaryRuleLabel::Variable)) => SymbolName::VariableId,
                    Some(RuleName::Primary(PrimaryRuleLabel::Calllocal)) => SymbolName::FunctionId,
                    Some(RuleName::Primary(PrimaryRuleLabel::Newobject)) => SymbolName::ClassId,
                    _ => name,
                },
                SymbolName::DOTID => match parent_rule {
                    Some(RuleName::Callinvoke) => SymbolName::FunctionId,
                    Some(RuleName::Fieldaccess) => SymbolName::VariableId,
                    _ => name,
                },
                _ => name,
            })
            .ok_or_else(|| format!("No exists symbol of name: {}", name))
    } else {
        Err(format!("invalid symbol index: {}", index))
    }
}

pub fn get_rule<'a>(tree: BaseTree<'a>) -> Result<RuleName, String> {
    return RuleName::from_tree(tree);
}

#[derive(Debug, PartialEq, Serialize)]
pub enum DstatementRuleLabel {
    Do,
    Decl,
    Continue,
    Break,
    Return,
    Throw,
    Expr,
    Error,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum RstatementRuleLabel {
    For,
    Try,
    While,
    Ineach,
    If,
    Each,
    Error,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum PrimaryRuleLabel {
    Precedence,
    Numeric,
    True,
    False,
    Null,
    String,
    Regex,
    Listinit,
    Mapinit,
    Variable,
    Calllocal,
    Newobject,
    Error,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum SymbolName {
    WS,
    COMMENT,
    LBRACK,
    RBRACK,
    LBRACE,
    RBRACE,
    LP,
    RP,
    DOLLAR,
    DOT,
    NSDOT,
    COMMA,
    SEMICOLON,
    IF,
    IN,
    ELSE,
    WHILE,
    DO,
    FOR,
    CONTINUE,
    BREAK,
    RETURN,
    NEW,
    TRY,
    CATCH,
    THROW,
    THIS,
    INSTANCEOF,
    BOOLNOT,
    BWNOT,
    MUL,
    DIV,
    REM,
    ADD,
    SUB,
    LSH,
    RSH,
    USH,
    LT,
    LTE,
    GT,
    GTE,
    EQ,
    EQR,
    NE,
    NER,
    BWAND,
    XOR,
    BWOR,
    BOOLAND,
    BOOLOR,
    COND,
    COLON,
    ELVIS,
    REF,
    ARROW,
    FIND,
    MATCH,
    INCR,
    DECR,
    ASSIGN,
    AADD,
    ASUB,
    AMUL,
    ADIV,
    AREM,
    AAND,
    AXOR,
    AOR,
    ALSH,
    ARSH,
    AUSH,
    OCTAL,
    HEX,
    INTEGER,
    DECIMAL,
    STRING,
    REGEX,
    TRUE,
    FALSE,
    NULL,
    PRIMITIVE,
    DEF,
    ID,
    DOTINTEGER,
    DOTID,
    EOF,
    // original
    VariableId,
    ClassId,
    FunctionId,
    ParameterId,
}

impl SymbolName {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "WS" => Some(SymbolName::WS),
            "COMMENT" => Some(SymbolName::COMMENT),
            "LBRACK" => Some(SymbolName::LBRACK),
            "RBRACK" => Some(SymbolName::RBRACK),
            "LBRACE" => Some(SymbolName::LBRACE),
            "RBRACE" => Some(SymbolName::RBRACE),
            "LP" => Some(SymbolName::LP),
            "RP" => Some(SymbolName::RP),
            "DOLLAR" => Some(SymbolName::DOLLAR),
            "DOT" => Some(SymbolName::DOT),
            "NSDOT" => Some(SymbolName::NSDOT),
            "COMMA" => Some(SymbolName::COMMA),
            "SEMICOLON" => Some(SymbolName::SEMICOLON),
            "IF" => Some(SymbolName::IF),
            "IN" => Some(SymbolName::IN),
            "ELSE" => Some(SymbolName::ELSE),
            "WHILE" => Some(SymbolName::WHILE),
            "DO" => Some(SymbolName::DO),
            "FOR" => Some(SymbolName::FOR),
            "CONTINUE" => Some(SymbolName::CONTINUE),
            "BREAK" => Some(SymbolName::BREAK),
            "RETURN" => Some(SymbolName::RETURN),
            "NEW" => Some(SymbolName::NEW),
            "TRY" => Some(SymbolName::TRY),
            "CATCH" => Some(SymbolName::CATCH),
            "THROW" => Some(SymbolName::THROW),
            "THIS" => Some(SymbolName::THIS),
            "INSTANCEOF" => Some(SymbolName::INSTANCEOF),
            "BOOLNOT" => Some(SymbolName::BOOLNOT),
            "BWNOT" => Some(SymbolName::BWNOT),
            "MUL" => Some(SymbolName::MUL),
            "DIV" => Some(SymbolName::DIV),
            "REM" => Some(SymbolName::REM),
            "ADD" => Some(SymbolName::ADD),
            "SUB" => Some(SymbolName::SUB),
            "LSH" => Some(SymbolName::LSH),
            "RSH" => Some(SymbolName::RSH),
            "USH" => Some(SymbolName::USH),
            "LT" => Some(SymbolName::LT),
            "LTE" => Some(SymbolName::LTE),
            "GT" => Some(SymbolName::GT),
            "GTE" => Some(SymbolName::GTE),
            "EQ" => Some(SymbolName::EQ),
            "EQR" => Some(SymbolName::EQR),
            "NE" => Some(SymbolName::NE),
            "NER" => Some(SymbolName::NER),
            "BWAND" => Some(SymbolName::BWAND),
            "XOR" => Some(SymbolName::XOR),
            "BWOR" => Some(SymbolName::BWOR),
            "BOOLAND" => Some(SymbolName::BOOLAND),
            "BOOLOR" => Some(SymbolName::BOOLOR),
            "COND" => Some(SymbolName::COND),
            "COLON" => Some(SymbolName::COLON),
            "ELVIS" => Some(SymbolName::ELVIS),
            "REF" => Some(SymbolName::REF),
            "ARROW" => Some(SymbolName::ARROW),
            "FIND" => Some(SymbolName::FIND),
            "MATCH" => Some(SymbolName::MATCH),
            "INCR" => Some(SymbolName::INCR),
            "DECR" => Some(SymbolName::DECR),
            "ASSIGN" => Some(SymbolName::ASSIGN),
            "AADD" => Some(SymbolName::AADD),
            "ASUB" => Some(SymbolName::ASUB),
            "AMUL" => Some(SymbolName::AMUL),
            "ADIV" => Some(SymbolName::ADIV),
            "AREM" => Some(SymbolName::AREM),
            "AAND" => Some(SymbolName::AAND),
            "AXOR" => Some(SymbolName::AXOR),
            "AOR" => Some(SymbolName::AOR),
            "ALSH" => Some(SymbolName::ALSH),
            "ARSH" => Some(SymbolName::ARSH),
            "AUSH" => Some(SymbolName::AUSH),
            "OCTAL" => Some(SymbolName::OCTAL),
            "HEX" => Some(SymbolName::HEX),
            "INTEGER" => Some(SymbolName::INTEGER),
            "DECIMAL" => Some(SymbolName::DECIMAL),
            "STRING" => Some(SymbolName::STRING),
            "REGEX" => Some(SymbolName::REGEX),
            "TRUE" => Some(SymbolName::TRUE),
            "FALSE" => Some(SymbolName::FALSE),
            "NULL" => Some(SymbolName::NULL),
            "PRIMITIVE" => Some(SymbolName::PRIMITIVE),
            "DEF" => Some(SymbolName::DEF),
            "ID" => Some(SymbolName::ID),
            "DOTINTEGER" => Some(SymbolName::DOTINTEGER),
            "DOTID" => Some(SymbolName::DOTID),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub enum RuleName {
    Source,
    Function,
    Parameters,
    Statement,
    Rstatement(RstatementRuleLabel),
    Dstatement(DstatementRuleLabel),
    Trailer,
    Block,
    Empty,
    Initializer,
    Afterthought,
    Declaration,
    Decltype,
    Type,
    Declvar,
    Trap,
    Noncondexpression,
    Expression,
    Unary,
    Unarynotaddsub,
    Castexpression,
    Primordefcasttype,
    Refcasttype,
    Chain,
    Primary(PrimaryRuleLabel),
    Postfix,
    Postdot,
    Callinvoke,
    Fieldaccess,
    Braceaccess,
    Arrayinitializer,
    Listinitializer,
    Mapinitializer,
    Maptoken,
    Arguments,
    Argument,
    Lambda,
    Lamtype,
    Funcref,
}

impl RuleName {
    pub fn from_tree<'a>(tree: BaseTree<'a>) -> Result<Self, String> {
        let index = tree.get_rule_index();
        let name = match ruleNames.get(index) {
            Some(name) => *name,
            None => return Err(format!("invalid rule index: {}", index)),
        };

        match name {
            "source" => Ok(RuleName::Source),
            "function" => Ok(RuleName::Function),
            "parameters" => Ok(RuleName::Parameters),
            "statement" => Ok(RuleName::Statement),
            "rstatement" => match tree.downcast_rc::<RstatementContextAll>() {
                Err(_) => {
                    return Err(format!(
                        "failed to downcast when rstatement. rule_name:{:?}:",
                        name,
                    ))
                }
                Ok(ctx) => {
                    return Ok(RuleName::Rstatement(match ctx.as_ref() {
                        RstatementContextAll::ForContext(_) => RstatementRuleLabel::For,
                        RstatementContextAll::TryContext(_) => RstatementRuleLabel::Try,
                        RstatementContextAll::WhileContext(_) => RstatementRuleLabel::While,
                        RstatementContextAll::IneachContext(_) => RstatementRuleLabel::Ineach,
                        RstatementContextAll::IfContext(_) => RstatementRuleLabel::If,
                        RstatementContextAll::EachContext(_) => RstatementRuleLabel::Each,
                        RstatementContextAll::Error(_) => RstatementRuleLabel::Error,
                    }))
                }
            },
            "dstatement" => match tree.downcast_rc::<DstatementContextAll>() {
                Err(_) => {
                    return Err(format!(
                        "failed to downcast when dstatement. rule_name:{:?}:",
                        name,
                    ))
                }
                Ok(ctx) => {
                    return Ok(RuleName::Dstatement(match ctx.as_ref() {
                        DstatementContextAll::DoContext(_) => DstatementRuleLabel::Do,
                        DstatementContextAll::DeclContext(_) => DstatementRuleLabel::Decl,
                        DstatementContextAll::ContinueContext(_) => DstatementRuleLabel::Continue,
                        DstatementContextAll::BreakContext(_) => DstatementRuleLabel::Break,
                        DstatementContextAll::ReturnContext(_) => DstatementRuleLabel::Return,
                        DstatementContextAll::ThrowContext(_) => DstatementRuleLabel::Throw,
                        DstatementContextAll::ExprContext(_) => DstatementRuleLabel::Expr,
                        DstatementContextAll::Error(_) => DstatementRuleLabel::Error,
                    }))
                }
            },
            "trailer" => Ok(RuleName::Trailer),
            "block" => Ok(RuleName::Block),
            "empty" => Ok(RuleName::Empty),
            "initializer" => Ok(RuleName::Initializer),
            "afterthought" => Ok(RuleName::Afterthought),
            "declaration" => Ok(RuleName::Declaration),
            "decltype" => Ok(RuleName::Decltype),
            "type_" => Ok(RuleName::Type),
            "declvar" => Ok(RuleName::Declvar),
            "trap" => Ok(RuleName::Trap),
            "noncondexpression" => Ok(RuleName::Noncondexpression),
            "expression" => Ok(RuleName::Expression),
            "unary" => Ok(RuleName::Unary),
            "unarynotaddsub" => Ok(RuleName::Unarynotaddsub),
            "castexpression" => Ok(RuleName::Castexpression),
            "primordefcasttype" => Ok(RuleName::Primordefcasttype),
            "refcasttype" => Ok(RuleName::Refcasttype),
            "chain" => Ok(RuleName::Chain),
            "primary" => match tree.downcast_rc::<PrimaryContextAll>() {
                Err(_) => {
                    return Err(format!(
                        "failed to downcast when dstatement. rule_name:{:?}:",
                        name,
                    ))
                }
                Ok(ctx) => {
                    return Ok(RuleName::Primary(match ctx.as_ref() {
                        PrimaryContextAll::PrecedenceContext(_) => PrimaryRuleLabel::Precedence,
                        PrimaryContextAll::NumericContext(_) => PrimaryRuleLabel::Numeric,
                        PrimaryContextAll::TrueContext(_) => PrimaryRuleLabel::True,
                        PrimaryContextAll::FalseContext(_) => PrimaryRuleLabel::False,
                        PrimaryContextAll::NullContext(_) => PrimaryRuleLabel::Null,
                        PrimaryContextAll::StringContext(_) => PrimaryRuleLabel::String,
                        PrimaryContextAll::RegexContext(_) => PrimaryRuleLabel::Regex,
                        PrimaryContextAll::ListinitContext(_) => PrimaryRuleLabel::Listinit,
                        PrimaryContextAll::MapinitContext(_) => PrimaryRuleLabel::Mapinit,
                        PrimaryContextAll::VariableContext(_) => PrimaryRuleLabel::Variable,
                        PrimaryContextAll::CalllocalContext(_) => PrimaryRuleLabel::Calllocal,
                        PrimaryContextAll::NewobjectContext(_) => PrimaryRuleLabel::Newobject,
                        PrimaryContextAll::Error(_) => PrimaryRuleLabel::Error,
                    }))
                }
            },
            "postfix" => Ok(RuleName::Postfix),
            "postdot" => Ok(RuleName::Postdot),
            "callinvoke" => Ok(RuleName::Callinvoke),
            "fieldaccess" => Ok(RuleName::Fieldaccess),
            "braceaccess" => Ok(RuleName::Braceaccess),
            "arrayinitializer" => Ok(RuleName::Arrayinitializer),
            "listinitializer" => Ok(RuleName::Listinitializer),
            "mapinitializer" => Ok(RuleName::Mapinitializer),
            "maptoken" => Ok(RuleName::Maptoken),
            "arguments" => Ok(RuleName::Arguments),
            "argument" => Ok(RuleName::Argument),
            "lambda" => Ok(RuleName::Lambda),
            "lamtype" => Ok(RuleName::Lamtype),
            "funcref" => Ok(RuleName::Funcref),
            _ => Err(format!("No exists rule of name: {}", name)),
        }
    }
}
