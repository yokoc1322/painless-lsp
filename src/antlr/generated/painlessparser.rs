// Generated from PainlessParser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use super::painlessparserlistener::*;
use super::painlessparservisitor::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const WS: isize = 1;
pub const COMMENT: isize = 2;
pub const LBRACK: isize = 3;
pub const RBRACK: isize = 4;
pub const LBRACE: isize = 5;
pub const RBRACE: isize = 6;
pub const LP: isize = 7;
pub const RP: isize = 8;
pub const DOLLAR: isize = 9;
pub const DOT: isize = 10;
pub const NSDOT: isize = 11;
pub const COMMA: isize = 12;
pub const SEMICOLON: isize = 13;
pub const IF: isize = 14;
pub const IN: isize = 15;
pub const ELSE: isize = 16;
pub const WHILE: isize = 17;
pub const DO: isize = 18;
pub const FOR: isize = 19;
pub const CONTINUE: isize = 20;
pub const BREAK: isize = 21;
pub const RETURN: isize = 22;
pub const NEW: isize = 23;
pub const TRY: isize = 24;
pub const CATCH: isize = 25;
pub const THROW: isize = 26;
pub const THIS: isize = 27;
pub const INSTANCEOF: isize = 28;
pub const BOOLNOT: isize = 29;
pub const BWNOT: isize = 30;
pub const MUL: isize = 31;
pub const DIV: isize = 32;
pub const REM: isize = 33;
pub const ADD: isize = 34;
pub const SUB: isize = 35;
pub const LSH: isize = 36;
pub const RSH: isize = 37;
pub const USH: isize = 38;
pub const LT: isize = 39;
pub const LTE: isize = 40;
pub const GT: isize = 41;
pub const GTE: isize = 42;
pub const EQ: isize = 43;
pub const EQR: isize = 44;
pub const NE: isize = 45;
pub const NER: isize = 46;
pub const BWAND: isize = 47;
pub const XOR: isize = 48;
pub const BWOR: isize = 49;
pub const BOOLAND: isize = 50;
pub const BOOLOR: isize = 51;
pub const COND: isize = 52;
pub const COLON: isize = 53;
pub const ELVIS: isize = 54;
pub const REF: isize = 55;
pub const ARROW: isize = 56;
pub const FIND: isize = 57;
pub const MATCH: isize = 58;
pub const INCR: isize = 59;
pub const DECR: isize = 60;
pub const ASSIGN: isize = 61;
pub const AADD: isize = 62;
pub const ASUB: isize = 63;
pub const AMUL: isize = 64;
pub const ADIV: isize = 65;
pub const AREM: isize = 66;
pub const AAND: isize = 67;
pub const AXOR: isize = 68;
pub const AOR: isize = 69;
pub const ALSH: isize = 70;
pub const ARSH: isize = 71;
pub const AUSH: isize = 72;
pub const OCTAL: isize = 73;
pub const HEX: isize = 74;
pub const INTEGER: isize = 75;
pub const DECIMAL: isize = 76;
pub const STRING: isize = 77;
pub const REGEX: isize = 78;
pub const TRUE: isize = 79;
pub const FALSE: isize = 80;
pub const NULL: isize = 81;
pub const PRIMITIVE: isize = 82;
pub const DEF: isize = 83;
pub const ID: isize = 84;
pub const DOTINTEGER: isize = 85;
pub const DOTID: isize = 86;
pub const RULE_source: usize = 0;
pub const RULE_function: usize = 1;
pub const RULE_parameters: usize = 2;
pub const RULE_statement: usize = 3;
pub const RULE_rstatement: usize = 4;
pub const RULE_dstatement: usize = 5;
pub const RULE_trailer: usize = 6;
pub const RULE_block: usize = 7;
pub const RULE_empty: usize = 8;
pub const RULE_initializer: usize = 9;
pub const RULE_afterthought: usize = 10;
pub const RULE_declaration: usize = 11;
pub const RULE_decltype: usize = 12;
pub const RULE_type_: usize = 13;
pub const RULE_declvar: usize = 14;
pub const RULE_trap: usize = 15;
pub const RULE_noncondexpression: usize = 16;
pub const RULE_expression: usize = 17;
pub const RULE_unary: usize = 18;
pub const RULE_unarynotaddsub: usize = 19;
pub const RULE_castexpression: usize = 20;
pub const RULE_primordefcasttype: usize = 21;
pub const RULE_refcasttype: usize = 22;
pub const RULE_chain: usize = 23;
pub const RULE_primary: usize = 24;
pub const RULE_postfix: usize = 25;
pub const RULE_postdot: usize = 26;
pub const RULE_callinvoke: usize = 27;
pub const RULE_fieldaccess: usize = 28;
pub const RULE_braceaccess: usize = 29;
pub const RULE_arrayinitializer: usize = 30;
pub const RULE_listinitializer: usize = 31;
pub const RULE_mapinitializer: usize = 32;
pub const RULE_maptoken: usize = 33;
pub const RULE_arguments: usize = 34;
pub const RULE_argument: usize = 35;
pub const RULE_lambda: usize = 36;
pub const RULE_lamtype: usize = 37;
pub const RULE_funcref: usize = 38;
pub const ruleNames: [&'static str; 39] = [
    "source",
    "function",
    "parameters",
    "statement",
    "rstatement",
    "dstatement",
    "trailer",
    "block",
    "empty",
    "initializer",
    "afterthought",
    "declaration",
    "decltype",
    "type_",
    "declvar",
    "trap",
    "noncondexpression",
    "expression",
    "unary",
    "unarynotaddsub",
    "castexpression",
    "primordefcasttype",
    "refcasttype",
    "chain",
    "primary",
    "postfix",
    "postdot",
    "callinvoke",
    "fieldaccess",
    "braceaccess",
    "arrayinitializer",
    "listinitializer",
    "mapinitializer",
    "maptoken",
    "arguments",
    "argument",
    "lambda",
    "lamtype",
    "funcref",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 84] = [
    None,
    None,
    None,
    Some("'{'"),
    Some("'}'"),
    Some("'['"),
    Some("']'"),
    Some("'('"),
    Some("')'"),
    Some("'$'"),
    Some("'.'"),
    Some("'?.'"),
    Some("','"),
    Some("';'"),
    Some("'if'"),
    Some("'in'"),
    Some("'else'"),
    Some("'while'"),
    Some("'do'"),
    Some("'for'"),
    Some("'continue'"),
    Some("'break'"),
    Some("'return'"),
    Some("'new'"),
    Some("'try'"),
    Some("'catch'"),
    Some("'throw'"),
    Some("'this'"),
    Some("'instanceof'"),
    Some("'!'"),
    Some("'~'"),
    Some("'*'"),
    Some("'/'"),
    Some("'%'"),
    Some("'+'"),
    Some("'-'"),
    Some("'<<'"),
    Some("'>>'"),
    Some("'>>>'"),
    Some("'<'"),
    Some("'<='"),
    Some("'>'"),
    Some("'>='"),
    Some("'=='"),
    Some("'==='"),
    Some("'!='"),
    Some("'!=='"),
    Some("'&'"),
    Some("'^'"),
    Some("'|'"),
    Some("'&&'"),
    Some("'||'"),
    Some("'?'"),
    Some("':'"),
    Some("'?:'"),
    Some("'::'"),
    Some("'->'"),
    Some("'=~'"),
    Some("'==~'"),
    Some("'++'"),
    Some("'--'"),
    Some("'='"),
    Some("'+='"),
    Some("'-='"),
    Some("'*='"),
    Some("'/='"),
    Some("'%='"),
    Some("'&='"),
    Some("'^='"),
    Some("'|='"),
    Some("'<<='"),
    Some("'>>='"),
    Some("'>>>='"),
    None,
    None,
    None,
    None,
    None,
    None,
    Some("'true'"),
    Some("'false'"),
    Some("'null'"),
    None,
    Some("'def'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 87] = [
    None,
    Some("WS"),
    Some("COMMENT"),
    Some("LBRACK"),
    Some("RBRACK"),
    Some("LBRACE"),
    Some("RBRACE"),
    Some("LP"),
    Some("RP"),
    Some("DOLLAR"),
    Some("DOT"),
    Some("NSDOT"),
    Some("COMMA"),
    Some("SEMICOLON"),
    Some("IF"),
    Some("IN"),
    Some("ELSE"),
    Some("WHILE"),
    Some("DO"),
    Some("FOR"),
    Some("CONTINUE"),
    Some("BREAK"),
    Some("RETURN"),
    Some("NEW"),
    Some("TRY"),
    Some("CATCH"),
    Some("THROW"),
    Some("THIS"),
    Some("INSTANCEOF"),
    Some("BOOLNOT"),
    Some("BWNOT"),
    Some("MUL"),
    Some("DIV"),
    Some("REM"),
    Some("ADD"),
    Some("SUB"),
    Some("LSH"),
    Some("RSH"),
    Some("USH"),
    Some("LT"),
    Some("LTE"),
    Some("GT"),
    Some("GTE"),
    Some("EQ"),
    Some("EQR"),
    Some("NE"),
    Some("NER"),
    Some("BWAND"),
    Some("XOR"),
    Some("BWOR"),
    Some("BOOLAND"),
    Some("BOOLOR"),
    Some("COND"),
    Some("COLON"),
    Some("ELVIS"),
    Some("REF"),
    Some("ARROW"),
    Some("FIND"),
    Some("MATCH"),
    Some("INCR"),
    Some("DECR"),
    Some("ASSIGN"),
    Some("AADD"),
    Some("ASUB"),
    Some("AMUL"),
    Some("ADIV"),
    Some("AREM"),
    Some("AAND"),
    Some("AXOR"),
    Some("AOR"),
    Some("ALSH"),
    Some("ARSH"),
    Some("AUSH"),
    Some("OCTAL"),
    Some("HEX"),
    Some("INTEGER"),
    Some("DECIMAL"),
    Some("STRING"),
    Some("REGEX"),
    Some("TRUE"),
    Some("FALSE"),
    Some("NULL"),
    Some("PRIMITIVE"),
    Some("DEF"),
    Some("ID"),
    Some("DOTINTEGER"),
    Some("DOTID"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    PainlessParserExt<'input>,
    I,
    PainlessParserContextType,
    dyn PainlessParserListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type PainlessParserTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, PainlessParserContextType, dyn PainlessParserListener<'input> + 'a>;

/// Parser for PainlessParser grammar
pub struct PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                PainlessParserExt {
                    _pd: Default::default(),
                },
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> PainlessParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> PainlessParser<'input, I, DefaultErrorStrategy<'input, PainlessParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for PainlessParser
pub trait PainlessParserContext<'input>:
    for<'x> Listenable<dyn PainlessParserListener<'input> + 'x>
    + for<'x> Visitable<dyn PainlessParserVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = PainlessParserContextType>
{
}

antlr_rust::coerce_from! { 'input : PainlessParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn PainlessParserContext<'input> + 'input
where
    T: PainlessParserVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn PainlessParserVisitor<'input> + 'x))
    }
}

impl<'input> PainlessParserContext<'input> for TerminalNode<'input, PainlessParserContextType> {}
impl<'input> PainlessParserContext<'input> for ErrorNode<'input, PainlessParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn PainlessParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn PainlessParserListener<'input> + 'input }

pub struct PainlessParserContextType;
antlr_rust::tid! {PainlessParserContextType}

impl<'input> ParserNodeType<'input> for PainlessParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn PainlessParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct PainlessParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> PainlessParserExt<'input> {}
antlr_rust::tid! { PainlessParserExt<'a> }

impl<'input> TokenAware<'input> for PainlessParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for PainlessParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for PainlessParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "PainlessParser.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
    fn sempred(
        _localctx: Option<&(dyn PainlessParserContext<'input> + 'input)>,
        rule_index: isize,
        pred_index: isize,
        recog: &mut BaseParserType<'input, I>,
    ) -> bool {
        match rule_index {
            16 => PainlessParser::<'input, I, _>::noncondexpression_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            _ => true,
        }
    }
}

impl<'input, I> PainlessParser<'input, I, DefaultErrorStrategy<'input, PainlessParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    fn noncondexpression_sempred(
        _localctx: Option<&NoncondexpressionContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            0 => recog.precpred(None, 13),
            1 => recog.precpred(None, 12),
            2 => recog.precpred(None, 11),
            3 => recog.precpred(None, 10),
            4 => recog.precpred(None, 9),
            5 => recog.precpred(None, 7),
            6 => recog.precpred(None, 6),
            7 => recog.precpred(None, 5),
            8 => recog.precpred(None, 4),
            9 => recog.precpred(None, 3),
            10 => recog.precpred(None, 2),
            11 => recog.precpred(None, 1),
            12 => recog.precpred(None, 8),
            _ => true,
        }
    }
}
//------------------- source ----------------
pub type SourceContextAll<'input> = SourceContext<'input>;

pub type SourceContext<'input> = BaseParserRuleContext<'input, SourceContextExt<'input>>;

#[derive(Clone)]
pub struct SourceContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for SourceContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for SourceContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_source(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_source(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for SourceContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_source(self);
    }
}

impl<'input> CustomRuleContext<'input> for SourceContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_source
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_source }
}
antlr_rust::tid! {SourceContextExt<'a>}

impl<'input> SourceContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SourceContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SourceContextExt { ph: PhantomData },
        ))
    }
}

pub trait SourceContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<SourceContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
    fn function_all(&self) -> Vec<Rc<FunctionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn function(&self, i: usize) -> Option<Rc<FunctionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn statement_all(&self) -> Vec<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> SourceContextAttrs<'input> for SourceContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn source(&mut self) -> Result<Rc<SourceContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_source);
        let mut _localctx: Rc<SourceContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(81);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(0, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule function*/
                                recog.base.set_state(78);
                                recog.function()?;
                            }
                        }
                    }
                    recog.base.set_state(83);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(0, &mut recog.base)?;
                }
                recog.base.set_state(87);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la - 5) & !0x3f) == 0
                    && ((1usize << (_la - 5))
                        & ((1usize << (LBRACE - 5))
                            | (1usize << (LP - 5))
                            | (1usize << (DOLLAR - 5))
                            | (1usize << (IF - 5))
                            | (1usize << (WHILE - 5))
                            | (1usize << (DO - 5))
                            | (1usize << (FOR - 5))
                            | (1usize << (CONTINUE - 5))
                            | (1usize << (BREAK - 5))
                            | (1usize << (RETURN - 5))
                            | (1usize << (NEW - 5))
                            | (1usize << (TRY - 5))
                            | (1usize << (THROW - 5))
                            | (1usize << (BOOLNOT - 5))
                            | (1usize << (BWNOT - 5))
                            | (1usize << (ADD - 5))
                            | (1usize << (SUB - 5))))
                        != 0)
                    || (((_la - 59) & !0x3f) == 0
                        && ((1usize << (_la - 59))
                            & ((1usize << (INCR - 59))
                                | (1usize << (DECR - 59))
                                | (1usize << (OCTAL - 59))
                                | (1usize << (HEX - 59))
                                | (1usize << (INTEGER - 59))
                                | (1usize << (DECIMAL - 59))
                                | (1usize << (STRING - 59))
                                | (1usize << (REGEX - 59))
                                | (1usize << (TRUE - 59))
                                | (1usize << (FALSE - 59))
                                | (1usize << (NULL - 59))
                                | (1usize << (PRIMITIVE - 59))
                                | (1usize << (DEF - 59))
                                | (1usize << (ID - 59))))
                            != 0)
                {
                    {
                        {
                            /*InvokeRule statement*/
                            recog.base.set_state(84);
                            recog.statement()?;
                        }
                    }
                    recog.base.set_state(89);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(90);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- function ----------------
pub type FunctionContextAll<'input> = FunctionContext<'input>;

pub type FunctionContext<'input> = BaseParserRuleContext<'input, FunctionContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for FunctionContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for FunctionContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_function(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_function(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for FunctionContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_function(self);
    }
}

impl<'input> CustomRuleContext<'input> for FunctionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_function
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_function }
}
antlr_rust::tid! {FunctionContextExt<'a>}

impl<'input> FunctionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FunctionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FunctionContextExt { ph: PhantomData },
        ))
    }
}

pub trait FunctionContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<FunctionContextExt<'input>>
{
    fn decltype(&self) -> Option<Rc<DecltypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    fn parameters(&self) -> Option<Rc<ParametersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> FunctionContextAttrs<'input> for FunctionContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn function(&mut self) -> Result<Rc<FunctionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_function);
        let mut _localctx: Rc<FunctionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule decltype*/
                recog.base.set_state(92);
                recog.decltype()?;

                recog.base.set_state(93);
                recog.base.match_token(ID, &mut recog.err_handler)?;

                /*InvokeRule parameters*/
                recog.base.set_state(94);
                recog.parameters()?;

                /*InvokeRule block*/
                recog.base.set_state(95);
                recog.block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- parameters ----------------
pub type ParametersContextAll<'input> = ParametersContext<'input>;

pub type ParametersContext<'input> = BaseParserRuleContext<'input, ParametersContextExt<'input>>;

#[derive(Clone)]
pub struct ParametersContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for ParametersContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ParametersContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_parameters(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_parameters(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ParametersContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_parameters(self);
    }
}

impl<'input> CustomRuleContext<'input> for ParametersContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_parameters
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_parameters }
}
antlr_rust::tid! {ParametersContextExt<'a>}

impl<'input> ParametersContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ParametersContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ParametersContextExt { ph: PhantomData },
        ))
    }
}

pub trait ParametersContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<ParametersContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
    fn decltype_all(&self) -> Vec<Rc<DecltypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn decltype(&self, i: usize) -> Option<Rc<DecltypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less or equal than `i`.
    fn ID(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> ParametersContextAttrs<'input> for ParametersContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn parameters(&mut self) -> Result<Rc<ParametersContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_parameters);
        let mut _localctx: Rc<ParametersContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(97);
                recog.base.match_token(LP, &mut recog.err_handler)?;

                recog.base.set_state(109);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la - 82) & !0x3f) == 0
                    && ((1usize << (_la - 82))
                        & ((1usize << (PRIMITIVE - 82))
                            | (1usize << (DEF - 82))
                            | (1usize << (ID - 82))))
                        != 0)
                {
                    {
                        /*InvokeRule decltype*/
                        recog.base.set_state(98);
                        recog.decltype()?;

                        recog.base.set_state(99);
                        recog.base.match_token(ID, &mut recog.err_handler)?;

                        recog.base.set_state(106);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(100);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule decltype*/
                                    recog.base.set_state(101);
                                    recog.decltype()?;

                                    recog.base.set_state(102);
                                    recog.base.match_token(ID, &mut recog.err_handler)?;
                                }
                            }
                            recog.base.set_state(108);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                    }
                }

                recog.base.set_state(111);
                recog.base.match_token(RP, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;

pub type StatementContext<'input> = BaseParserRuleContext<'input, StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for StatementContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for StatementContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_statement(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for StatementContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid! {StatementContextExt<'a>}

impl<'input> StatementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StatementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StatementContextExt { ph: PhantomData },
        ))
    }
}

pub trait StatementContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<StatementContextExt<'input>>
{
    fn rstatement(&self) -> Option<Rc<RstatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn dstatement(&self) -> Option<Rc<DstatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn statement(&mut self) -> Result<Rc<StatementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(117);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                IF | WHILE | FOR | TRY => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule rstatement*/
                        recog.base.set_state(113);
                        recog.rstatement()?;
                    }
                }

                LBRACE | LP | DOLLAR | DO | CONTINUE | BREAK | RETURN | NEW | THROW | BOOLNOT
                | BWNOT | ADD | SUB | INCR | DECR | OCTAL | HEX | INTEGER | DECIMAL | STRING
                | REGEX | TRUE | FALSE | NULL | PRIMITIVE | DEF | ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule dstatement*/
                        recog.base.set_state(114);
                        recog.dstatement()?;

                        recog.base.set_state(115);
                        _la = recog.base.input.la(1);
                        if { !(_la == EOF || _la == SEMICOLON) } {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- rstatement ----------------
#[derive(Debug)]
pub enum RstatementContextAll<'input> {
    ForContext(ForContext<'input>),
    TryContext(TryContext<'input>),
    WhileContext(WhileContext<'input>),
    IneachContext(IneachContext<'input>),
    IfContext(IfContext<'input>),
    EachContext(EachContext<'input>),
    Error(RstatementContext<'input>),
}
antlr_rust::tid! {RstatementContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for RstatementContextAll<'input> {}

impl<'input> PainlessParserContext<'input> for RstatementContextAll<'input> {}

impl<'input> Deref for RstatementContextAll<'input> {
    type Target = dyn RstatementContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use RstatementContextAll::*;
        match self {
            ForContext(inner) => inner,
            TryContext(inner) => inner,
            WhileContext(inner) => inner,
            IneachContext(inner) => inner,
            IfContext(inner) => inner,
            EachContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for RstatementContextAll<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for RstatementContextAll<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type RstatementContext<'input> = BaseParserRuleContext<'input, RstatementContextExt<'input>>;

#[derive(Clone)]
pub struct RstatementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for RstatementContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for RstatementContext<'input> {}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for RstatementContext<'input> {}

impl<'input> CustomRuleContext<'input> for RstatementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_rstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_rstatement }
}
antlr_rust::tid! {RstatementContextExt<'a>}

impl<'input> RstatementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<RstatementContextAll<'input>> {
        Rc::new(RstatementContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                RstatementContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait RstatementContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<RstatementContextExt<'input>>
{
}

impl<'input> RstatementContextAttrs<'input> for RstatementContext<'input> {}

pub type ForContext<'input> = BaseParserRuleContext<'input, ForContextExt<'input>>;

pub trait ForContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token FOR
    /// Returns `None` if there is no child corresponding to token FOR
    fn FOR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
    fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
    /// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
    fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, i)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
    fn trailer(&self) -> Option<Rc<TrailerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn empty(&self) -> Option<Rc<EmptyContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn initializer(&self) -> Option<Rc<InitializerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn afterthought(&self) -> Option<Rc<AfterthoughtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ForContextAttrs<'input> for ForContext<'input> {}

pub struct ForContextExt<'input> {
    base: RstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ForContextExt<'a>}

impl<'input> PainlessParserContext<'input> for ForContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ForContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_for(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_for(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ForContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_for(self);
    }
}

impl<'input> CustomRuleContext<'input> for ForContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_rstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_rstatement }
}

impl<'input> Borrow<RstatementContextExt<'input>> for ForContext<'input> {
    fn borrow(&self) -> &RstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<RstatementContextExt<'input>> for ForContext<'input> {
    fn borrow_mut(&mut self) -> &mut RstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> RstatementContextAttrs<'input> for ForContext<'input> {}

impl<'input> ForContextExt<'input> {
    fn new(ctx: &dyn RstatementContextAttrs<'input>) -> Rc<RstatementContextAll<'input>> {
        Rc::new(RstatementContextAll::ForContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ForContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type TryContext<'input> = BaseParserRuleContext<'input, TryContextExt<'input>>;

pub trait TryContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token TRY
    /// Returns `None` if there is no child corresponding to token TRY
    fn TRY(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TRY, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn trap_all(&self) -> Vec<Rc<TrapContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn trap(&self, i: usize) -> Option<Rc<TrapContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> TryContextAttrs<'input> for TryContext<'input> {}

pub struct TryContextExt<'input> {
    base: RstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {TryContextExt<'a>}

impl<'input> PainlessParserContext<'input> for TryContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for TryContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_try(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_try(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for TryContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_try(self);
    }
}

impl<'input> CustomRuleContext<'input> for TryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_rstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_rstatement }
}

impl<'input> Borrow<RstatementContextExt<'input>> for TryContext<'input> {
    fn borrow(&self) -> &RstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<RstatementContextExt<'input>> for TryContext<'input> {
    fn borrow_mut(&mut self) -> &mut RstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> RstatementContextAttrs<'input> for TryContext<'input> {}

impl<'input> TryContextExt<'input> {
    fn new(ctx: &dyn RstatementContextAttrs<'input>) -> Rc<RstatementContextAll<'input>> {
        Rc::new(RstatementContextAll::TryContext(
            BaseParserRuleContext::copy_from(
                ctx,
                TryContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type WhileContext<'input> = BaseParserRuleContext<'input, WhileContextExt<'input>>;

pub trait WhileContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token WHILE
    /// Returns `None` if there is no child corresponding to token WHILE
    fn WHILE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WHILE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
    fn trailer(&self) -> Option<Rc<TrailerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn empty(&self) -> Option<Rc<EmptyContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> WhileContextAttrs<'input> for WhileContext<'input> {}

pub struct WhileContextExt<'input> {
    base: RstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {WhileContextExt<'a>}

impl<'input> PainlessParserContext<'input> for WhileContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for WhileContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_while(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_while(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for WhileContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_while(self);
    }
}

impl<'input> CustomRuleContext<'input> for WhileContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_rstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_rstatement }
}

impl<'input> Borrow<RstatementContextExt<'input>> for WhileContext<'input> {
    fn borrow(&self) -> &RstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<RstatementContextExt<'input>> for WhileContext<'input> {
    fn borrow_mut(&mut self) -> &mut RstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> RstatementContextAttrs<'input> for WhileContext<'input> {}

impl<'input> WhileContextExt<'input> {
    fn new(ctx: &dyn RstatementContextAttrs<'input>) -> Rc<RstatementContextAll<'input>> {
        Rc::new(RstatementContextAll::WhileContext(
            BaseParserRuleContext::copy_from(
                ctx,
                WhileContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type IneachContext<'input> = BaseParserRuleContext<'input, IneachContextExt<'input>>;

pub trait IneachContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token FOR
    /// Returns `None` if there is no child corresponding to token FOR
    fn FOR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
    fn trailer(&self) -> Option<Rc<TrailerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> IneachContextAttrs<'input> for IneachContext<'input> {}

pub struct IneachContextExt<'input> {
    base: RstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {IneachContextExt<'a>}

impl<'input> PainlessParserContext<'input> for IneachContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for IneachContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_ineach(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_ineach(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for IneachContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_ineach(self);
    }
}

impl<'input> CustomRuleContext<'input> for IneachContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_rstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_rstatement }
}

impl<'input> Borrow<RstatementContextExt<'input>> for IneachContext<'input> {
    fn borrow(&self) -> &RstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<RstatementContextExt<'input>> for IneachContext<'input> {
    fn borrow_mut(&mut self) -> &mut RstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> RstatementContextAttrs<'input> for IneachContext<'input> {}

impl<'input> IneachContextExt<'input> {
    fn new(ctx: &dyn RstatementContextAttrs<'input>) -> Rc<RstatementContextAll<'input>> {
        Rc::new(RstatementContextAll::IneachContext(
            BaseParserRuleContext::copy_from(
                ctx,
                IneachContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type IfContext<'input> = BaseParserRuleContext<'input, IfContextExt<'input>>;

pub trait IfContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token IF
    /// Returns `None` if there is no child corresponding to token IF
    fn IF(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
    fn trailer_all(&self) -> Vec<Rc<TrailerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn trailer(&self, i: usize) -> Option<Rc<TrailerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELSE, 0)
    }
}

impl<'input> IfContextAttrs<'input> for IfContext<'input> {}

pub struct IfContextExt<'input> {
    base: RstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {IfContextExt<'a>}

impl<'input> PainlessParserContext<'input> for IfContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for IfContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_if(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_if(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for IfContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_if(self);
    }
}

impl<'input> CustomRuleContext<'input> for IfContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_rstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_rstatement }
}

impl<'input> Borrow<RstatementContextExt<'input>> for IfContext<'input> {
    fn borrow(&self) -> &RstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<RstatementContextExt<'input>> for IfContext<'input> {
    fn borrow_mut(&mut self) -> &mut RstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> RstatementContextAttrs<'input> for IfContext<'input> {}

impl<'input> IfContextExt<'input> {
    fn new(ctx: &dyn RstatementContextAttrs<'input>) -> Rc<RstatementContextAll<'input>> {
        Rc::new(RstatementContextAll::IfContext(
            BaseParserRuleContext::copy_from(
                ctx,
                IfContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type EachContext<'input> = BaseParserRuleContext<'input, EachContextExt<'input>>;

pub trait EachContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token FOR
    /// Returns `None` if there is no child corresponding to token FOR
    fn FOR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    fn decltype(&self) -> Option<Rc<DecltypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
    fn trailer(&self) -> Option<Rc<TrailerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> EachContextAttrs<'input> for EachContext<'input> {}

pub struct EachContextExt<'input> {
    base: RstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {EachContextExt<'a>}

impl<'input> PainlessParserContext<'input> for EachContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for EachContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_each(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_each(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for EachContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_each(self);
    }
}

impl<'input> CustomRuleContext<'input> for EachContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_rstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_rstatement }
}

impl<'input> Borrow<RstatementContextExt<'input>> for EachContext<'input> {
    fn borrow(&self) -> &RstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<RstatementContextExt<'input>> for EachContext<'input> {
    fn borrow_mut(&mut self) -> &mut RstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> RstatementContextAttrs<'input> for EachContext<'input> {}

impl<'input> EachContextExt<'input> {
    fn new(ctx: &dyn RstatementContextAttrs<'input>) -> Rc<RstatementContextAll<'input>> {
        Rc::new(RstatementContextAll::EachContext(
            BaseParserRuleContext::copy_from(
                ctx,
                EachContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn rstatement(&mut self) -> Result<Rc<RstatementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = RstatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_rstatement);
        let mut _localctx: Rc<RstatementContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(178);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(12, &mut recog.base)? {
                1 => {
                    let tmp = IfContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(119);
                        recog.base.match_token(IF, &mut recog.err_handler)?;

                        recog.base.set_state(120);
                        recog.base.match_token(LP, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(121);
                        recog.expression()?;

                        recog.base.set_state(122);
                        recog.base.match_token(RP, &mut recog.err_handler)?;

                        /*InvokeRule trailer*/
                        recog.base.set_state(123);
                        recog.trailer()?;

                        recog.base.set_state(126);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(5, &mut recog.base)? {
                            x if x == 1 => {
                                {
                                    recog.base.set_state(124);
                                    recog.base.match_token(ELSE, &mut recog.err_handler)?;

                                    /*InvokeRule trailer*/
                                    recog.base.set_state(125);
                                    recog.trailer()?;
                                }
                            }

                            _ => {}
                        }
                    }
                }
                2 => {
                    let tmp = WhileContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(128);
                        recog.base.match_token(WHILE, &mut recog.err_handler)?;

                        recog.base.set_state(129);
                        recog.base.match_token(LP, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(130);
                        recog.expression()?;

                        recog.base.set_state(131);
                        recog.base.match_token(RP, &mut recog.err_handler)?;

                        recog.base.set_state(134);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            LBRACK | LBRACE | LP | DOLLAR | IF | WHILE | DO | FOR | CONTINUE
                            | BREAK | RETURN | NEW | TRY | THROW | BOOLNOT | BWNOT | ADD | SUB
                            | INCR | DECR | OCTAL | HEX | INTEGER | DECIMAL | STRING | REGEX
                            | TRUE | FALSE | NULL | PRIMITIVE | DEF | ID => {
                                {
                                    /*InvokeRule trailer*/
                                    recog.base.set_state(132);
                                    recog.trailer()?;
                                }
                            }

                            SEMICOLON => {
                                {
                                    /*InvokeRule empty*/
                                    recog.base.set_state(133);
                                    recog.empty()?;
                                }
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                &mut recog.base,
                            )))?,
                        }
                    }
                }
                3 => {
                    let tmp = ForContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(136);
                        recog.base.match_token(FOR, &mut recog.err_handler)?;

                        recog.base.set_state(137);
                        recog.base.match_token(LP, &mut recog.err_handler)?;

                        recog.base.set_state(139);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la - 5) & !0x3f) == 0
                            && ((1usize << (_la - 5))
                                & ((1usize << (LBRACE - 5))
                                    | (1usize << (LP - 5))
                                    | (1usize << (DOLLAR - 5))
                                    | (1usize << (NEW - 5))
                                    | (1usize << (BOOLNOT - 5))
                                    | (1usize << (BWNOT - 5))
                                    | (1usize << (ADD - 5))
                                    | (1usize << (SUB - 5))))
                                != 0)
                            || (((_la - 59) & !0x3f) == 0
                                && ((1usize << (_la - 59))
                                    & ((1usize << (INCR - 59))
                                        | (1usize << (DECR - 59))
                                        | (1usize << (OCTAL - 59))
                                        | (1usize << (HEX - 59))
                                        | (1usize << (INTEGER - 59))
                                        | (1usize << (DECIMAL - 59))
                                        | (1usize << (STRING - 59))
                                        | (1usize << (REGEX - 59))
                                        | (1usize << (TRUE - 59))
                                        | (1usize << (FALSE - 59))
                                        | (1usize << (NULL - 59))
                                        | (1usize << (PRIMITIVE - 59))
                                        | (1usize << (DEF - 59))
                                        | (1usize << (ID - 59))))
                                    != 0)
                        {
                            {
                                /*InvokeRule initializer*/
                                recog.base.set_state(138);
                                recog.initializer()?;
                            }
                        }

                        recog.base.set_state(141);
                        recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;

                        recog.base.set_state(143);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la - 5) & !0x3f) == 0
                            && ((1usize << (_la - 5))
                                & ((1usize << (LBRACE - 5))
                                    | (1usize << (LP - 5))
                                    | (1usize << (DOLLAR - 5))
                                    | (1usize << (NEW - 5))
                                    | (1usize << (BOOLNOT - 5))
                                    | (1usize << (BWNOT - 5))
                                    | (1usize << (ADD - 5))
                                    | (1usize << (SUB - 5))))
                                != 0)
                            || (((_la - 59) & !0x3f) == 0
                                && ((1usize << (_la - 59))
                                    & ((1usize << (INCR - 59))
                                        | (1usize << (DECR - 59))
                                        | (1usize << (OCTAL - 59))
                                        | (1usize << (HEX - 59))
                                        | (1usize << (INTEGER - 59))
                                        | (1usize << (DECIMAL - 59))
                                        | (1usize << (STRING - 59))
                                        | (1usize << (REGEX - 59))
                                        | (1usize << (TRUE - 59))
                                        | (1usize << (FALSE - 59))
                                        | (1usize << (NULL - 59))
                                        | (1usize << (ID - 59))))
                                    != 0)
                        {
                            {
                                /*InvokeRule expression*/
                                recog.base.set_state(142);
                                recog.expression()?;
                            }
                        }

                        recog.base.set_state(145);
                        recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;

                        recog.base.set_state(147);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la - 5) & !0x3f) == 0
                            && ((1usize << (_la - 5))
                                & ((1usize << (LBRACE - 5))
                                    | (1usize << (LP - 5))
                                    | (1usize << (DOLLAR - 5))
                                    | (1usize << (NEW - 5))
                                    | (1usize << (BOOLNOT - 5))
                                    | (1usize << (BWNOT - 5))
                                    | (1usize << (ADD - 5))
                                    | (1usize << (SUB - 5))))
                                != 0)
                            || (((_la - 59) & !0x3f) == 0
                                && ((1usize << (_la - 59))
                                    & ((1usize << (INCR - 59))
                                        | (1usize << (DECR - 59))
                                        | (1usize << (OCTAL - 59))
                                        | (1usize << (HEX - 59))
                                        | (1usize << (INTEGER - 59))
                                        | (1usize << (DECIMAL - 59))
                                        | (1usize << (STRING - 59))
                                        | (1usize << (REGEX - 59))
                                        | (1usize << (TRUE - 59))
                                        | (1usize << (FALSE - 59))
                                        | (1usize << (NULL - 59))
                                        | (1usize << (ID - 59))))
                                    != 0)
                        {
                            {
                                /*InvokeRule afterthought*/
                                recog.base.set_state(146);
                                recog.afterthought()?;
                            }
                        }

                        recog.base.set_state(149);
                        recog.base.match_token(RP, &mut recog.err_handler)?;

                        recog.base.set_state(152);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            LBRACK | LBRACE | LP | DOLLAR | IF | WHILE | DO | FOR | CONTINUE
                            | BREAK | RETURN | NEW | TRY | THROW | BOOLNOT | BWNOT | ADD | SUB
                            | INCR | DECR | OCTAL | HEX | INTEGER | DECIMAL | STRING | REGEX
                            | TRUE | FALSE | NULL | PRIMITIVE | DEF | ID => {
                                {
                                    /*InvokeRule trailer*/
                                    recog.base.set_state(150);
                                    recog.trailer()?;
                                }
                            }

                            SEMICOLON => {
                                {
                                    /*InvokeRule empty*/
                                    recog.base.set_state(151);
                                    recog.empty()?;
                                }
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                &mut recog.base,
                            )))?,
                        }
                    }
                }
                4 => {
                    let tmp = EachContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        recog.base.set_state(154);
                        recog.base.match_token(FOR, &mut recog.err_handler)?;

                        recog.base.set_state(155);
                        recog.base.match_token(LP, &mut recog.err_handler)?;

                        /*InvokeRule decltype*/
                        recog.base.set_state(156);
                        recog.decltype()?;

                        recog.base.set_state(157);
                        recog.base.match_token(ID, &mut recog.err_handler)?;

                        recog.base.set_state(158);
                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(159);
                        recog.expression()?;

                        recog.base.set_state(160);
                        recog.base.match_token(RP, &mut recog.err_handler)?;

                        /*InvokeRule trailer*/
                        recog.base.set_state(161);
                        recog.trailer()?;
                    }
                }
                5 => {
                    let tmp = IneachContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        recog.base.set_state(163);
                        recog.base.match_token(FOR, &mut recog.err_handler)?;

                        recog.base.set_state(164);
                        recog.base.match_token(LP, &mut recog.err_handler)?;

                        recog.base.set_state(165);
                        recog.base.match_token(ID, &mut recog.err_handler)?;

                        recog.base.set_state(166);
                        recog.base.match_token(IN, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(167);
                        recog.expression()?;

                        recog.base.set_state(168);
                        recog.base.match_token(RP, &mut recog.err_handler)?;

                        /*InvokeRule trailer*/
                        recog.base.set_state(169);
                        recog.trailer()?;
                    }
                }
                6 => {
                    let tmp = TryContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 6);
                    _localctx = tmp;
                    {
                        recog.base.set_state(171);
                        recog.base.match_token(TRY, &mut recog.err_handler)?;

                        /*InvokeRule block*/
                        recog.base.set_state(172);
                        recog.block()?;

                        recog.base.set_state(174);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule trap*/
                                    recog.base.set_state(173);
                                    recog.trap()?;
                                }
                            }
                            recog.base.set_state(176);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == CATCH) {
                                break;
                            }
                        }
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- dstatement ----------------
#[derive(Debug)]
pub enum DstatementContextAll<'input> {
    DeclContext(DeclContext<'input>),
    BreakContext(BreakContext<'input>),
    ThrowContext(ThrowContext<'input>),
    ContinueContext(ContinueContext<'input>),
    ExprContext(ExprContext<'input>),
    DoContext(DoContext<'input>),
    ReturnContext(ReturnContext<'input>),
    Error(DstatementContext<'input>),
}
antlr_rust::tid! {DstatementContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for DstatementContextAll<'input> {}

impl<'input> PainlessParserContext<'input> for DstatementContextAll<'input> {}

impl<'input> Deref for DstatementContextAll<'input> {
    type Target = dyn DstatementContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use DstatementContextAll::*;
        match self {
            DeclContext(inner) => inner,
            BreakContext(inner) => inner,
            ThrowContext(inner) => inner,
            ContinueContext(inner) => inner,
            ExprContext(inner) => inner,
            DoContext(inner) => inner,
            ReturnContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for DstatementContextAll<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for DstatementContextAll<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type DstatementContext<'input> = BaseParserRuleContext<'input, DstatementContextExt<'input>>;

#[derive(Clone)]
pub struct DstatementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for DstatementContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for DstatementContext<'input> {}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for DstatementContext<'input> {}

impl<'input> CustomRuleContext<'input> for DstatementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dstatement }
}
antlr_rust::tid! {DstatementContextExt<'a>}

impl<'input> DstatementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DstatementContextAll<'input>> {
        Rc::new(DstatementContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                DstatementContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait DstatementContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<DstatementContextExt<'input>>
{
}

impl<'input> DstatementContextAttrs<'input> for DstatementContext<'input> {}

pub type DeclContext<'input> = BaseParserRuleContext<'input, DeclContextExt<'input>>;

pub trait DeclContextAttrs<'input>: PainlessParserContext<'input> {
    fn declaration(&self) -> Option<Rc<DeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> DeclContextAttrs<'input> for DeclContext<'input> {}

pub struct DeclContextExt<'input> {
    base: DstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {DeclContextExt<'a>}

impl<'input> PainlessParserContext<'input> for DeclContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for DeclContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_decl(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_decl(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for DeclContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_decl(self);
    }
}

impl<'input> CustomRuleContext<'input> for DeclContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dstatement }
}

impl<'input> Borrow<DstatementContextExt<'input>> for DeclContext<'input> {
    fn borrow(&self) -> &DstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<DstatementContextExt<'input>> for DeclContext<'input> {
    fn borrow_mut(&mut self) -> &mut DstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> DstatementContextAttrs<'input> for DeclContext<'input> {}

impl<'input> DeclContextExt<'input> {
    fn new(ctx: &dyn DstatementContextAttrs<'input>) -> Rc<DstatementContextAll<'input>> {
        Rc::new(DstatementContextAll::DeclContext(
            BaseParserRuleContext::copy_from(
                ctx,
                DeclContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type BreakContext<'input> = BaseParserRuleContext<'input, BreakContextExt<'input>>;

pub trait BreakContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token BREAK
    /// Returns `None` if there is no child corresponding to token BREAK
    fn BREAK(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BREAK, 0)
    }
}

impl<'input> BreakContextAttrs<'input> for BreakContext<'input> {}

pub struct BreakContextExt<'input> {
    base: DstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {BreakContextExt<'a>}

impl<'input> PainlessParserContext<'input> for BreakContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for BreakContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_break(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_break(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for BreakContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_break(self);
    }
}

impl<'input> CustomRuleContext<'input> for BreakContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dstatement }
}

impl<'input> Borrow<DstatementContextExt<'input>> for BreakContext<'input> {
    fn borrow(&self) -> &DstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<DstatementContextExt<'input>> for BreakContext<'input> {
    fn borrow_mut(&mut self) -> &mut DstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> DstatementContextAttrs<'input> for BreakContext<'input> {}

impl<'input> BreakContextExt<'input> {
    fn new(ctx: &dyn DstatementContextAttrs<'input>) -> Rc<DstatementContextAll<'input>> {
        Rc::new(DstatementContextAll::BreakContext(
            BaseParserRuleContext::copy_from(
                ctx,
                BreakContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ThrowContext<'input> = BaseParserRuleContext<'input, ThrowContextExt<'input>>;

pub trait ThrowContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token THROW
    /// Returns `None` if there is no child corresponding to token THROW
    fn THROW(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(THROW, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ThrowContextAttrs<'input> for ThrowContext<'input> {}

pub struct ThrowContextExt<'input> {
    base: DstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ThrowContextExt<'a>}

impl<'input> PainlessParserContext<'input> for ThrowContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ThrowContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_throw(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_throw(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ThrowContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_throw(self);
    }
}

impl<'input> CustomRuleContext<'input> for ThrowContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dstatement }
}

impl<'input> Borrow<DstatementContextExt<'input>> for ThrowContext<'input> {
    fn borrow(&self) -> &DstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<DstatementContextExt<'input>> for ThrowContext<'input> {
    fn borrow_mut(&mut self) -> &mut DstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> DstatementContextAttrs<'input> for ThrowContext<'input> {}

impl<'input> ThrowContextExt<'input> {
    fn new(ctx: &dyn DstatementContextAttrs<'input>) -> Rc<DstatementContextAll<'input>> {
        Rc::new(DstatementContextAll::ThrowContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ThrowContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ContinueContext<'input> = BaseParserRuleContext<'input, ContinueContextExt<'input>>;

pub trait ContinueContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token CONTINUE
    /// Returns `None` if there is no child corresponding to token CONTINUE
    fn CONTINUE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CONTINUE, 0)
    }
}

impl<'input> ContinueContextAttrs<'input> for ContinueContext<'input> {}

pub struct ContinueContextExt<'input> {
    base: DstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ContinueContextExt<'a>}

impl<'input> PainlessParserContext<'input> for ContinueContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ContinueContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_continue(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_continue(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ContinueContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_continue(self);
    }
}

impl<'input> CustomRuleContext<'input> for ContinueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dstatement }
}

impl<'input> Borrow<DstatementContextExt<'input>> for ContinueContext<'input> {
    fn borrow(&self) -> &DstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<DstatementContextExt<'input>> for ContinueContext<'input> {
    fn borrow_mut(&mut self) -> &mut DstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> DstatementContextAttrs<'input> for ContinueContext<'input> {}

impl<'input> ContinueContextExt<'input> {
    fn new(ctx: &dyn DstatementContextAttrs<'input>) -> Rc<DstatementContextAll<'input>> {
        Rc::new(DstatementContextAll::ContinueContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ContinueContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ExprContext<'input> = BaseParserRuleContext<'input, ExprContextExt<'input>>;

pub trait ExprContextAttrs<'input>: PainlessParserContext<'input> {
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input> {}

pub struct ExprContextExt<'input> {
    base: DstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ExprContextExt<'a>}

impl<'input> PainlessParserContext<'input> for ExprContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ExprContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expr(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_expr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ExprContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dstatement }
}

impl<'input> Borrow<DstatementContextExt<'input>> for ExprContext<'input> {
    fn borrow(&self) -> &DstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<DstatementContextExt<'input>> for ExprContext<'input> {
    fn borrow_mut(&mut self) -> &mut DstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> DstatementContextAttrs<'input> for ExprContext<'input> {}

impl<'input> ExprContextExt<'input> {
    fn new(ctx: &dyn DstatementContextAttrs<'input>) -> Rc<DstatementContextAll<'input>> {
        Rc::new(DstatementContextAll::ExprContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type DoContext<'input> = BaseParserRuleContext<'input, DoContextExt<'input>>;

pub trait DoContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token DO
    /// Returns `None` if there is no child corresponding to token DO
    fn DO(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DO, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token WHILE
    /// Returns `None` if there is no child corresponding to token WHILE
    fn WHILE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WHILE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
}

impl<'input> DoContextAttrs<'input> for DoContext<'input> {}

pub struct DoContextExt<'input> {
    base: DstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {DoContextExt<'a>}

impl<'input> PainlessParserContext<'input> for DoContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for DoContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_do(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_do(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for DoContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_do(self);
    }
}

impl<'input> CustomRuleContext<'input> for DoContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dstatement }
}

impl<'input> Borrow<DstatementContextExt<'input>> for DoContext<'input> {
    fn borrow(&self) -> &DstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<DstatementContextExt<'input>> for DoContext<'input> {
    fn borrow_mut(&mut self) -> &mut DstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> DstatementContextAttrs<'input> for DoContext<'input> {}

impl<'input> DoContextExt<'input> {
    fn new(ctx: &dyn DstatementContextAttrs<'input>) -> Rc<DstatementContextAll<'input>> {
        Rc::new(DstatementContextAll::DoContext(
            BaseParserRuleContext::copy_from(
                ctx,
                DoContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ReturnContext<'input> = BaseParserRuleContext<'input, ReturnContextExt<'input>>;

pub trait ReturnContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token RETURN
    /// Returns `None` if there is no child corresponding to token RETURN
    fn RETURN(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RETURN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ReturnContextAttrs<'input> for ReturnContext<'input> {}

pub struct ReturnContextExt<'input> {
    base: DstatementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ReturnContextExt<'a>}

impl<'input> PainlessParserContext<'input> for ReturnContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ReturnContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_return(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_return(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ReturnContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_return(self);
    }
}

impl<'input> CustomRuleContext<'input> for ReturnContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dstatement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dstatement }
}

impl<'input> Borrow<DstatementContextExt<'input>> for ReturnContext<'input> {
    fn borrow(&self) -> &DstatementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<DstatementContextExt<'input>> for ReturnContext<'input> {
    fn borrow_mut(&mut self) -> &mut DstatementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> DstatementContextAttrs<'input> for ReturnContext<'input> {}

impl<'input> ReturnContextExt<'input> {
    fn new(ctx: &dyn DstatementContextAttrs<'input>) -> Rc<DstatementContextAll<'input>> {
        Rc::new(DstatementContextAll::ReturnContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ReturnContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn dstatement(&mut self) -> Result<Rc<DstatementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DstatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 10, RULE_dstatement);
        let mut _localctx: Rc<DstatementContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(197);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(14, &mut recog.base)? {
                1 => {
                    let tmp = DoContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(180);
                        recog.base.match_token(DO, &mut recog.err_handler)?;

                        /*InvokeRule block*/
                        recog.base.set_state(181);
                        recog.block()?;

                        recog.base.set_state(182);
                        recog.base.match_token(WHILE, &mut recog.err_handler)?;

                        recog.base.set_state(183);
                        recog.base.match_token(LP, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(184);
                        recog.expression()?;

                        recog.base.set_state(185);
                        recog.base.match_token(RP, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    let tmp = DeclContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        /*InvokeRule declaration*/
                        recog.base.set_state(187);
                        recog.declaration()?;
                    }
                }
                3 => {
                    let tmp = ContinueContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(188);
                        recog.base.match_token(CONTINUE, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    let tmp = BreakContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        recog.base.set_state(189);
                        recog.base.match_token(BREAK, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    let tmp = ReturnContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        recog.base.set_state(190);
                        recog.base.match_token(RETURN, &mut recog.err_handler)?;

                        recog.base.set_state(192);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la - 5) & !0x3f) == 0
                            && ((1usize << (_la - 5))
                                & ((1usize << (LBRACE - 5))
                                    | (1usize << (LP - 5))
                                    | (1usize << (DOLLAR - 5))
                                    | (1usize << (NEW - 5))
                                    | (1usize << (BOOLNOT - 5))
                                    | (1usize << (BWNOT - 5))
                                    | (1usize << (ADD - 5))
                                    | (1usize << (SUB - 5))))
                                != 0)
                            || (((_la - 59) & !0x3f) == 0
                                && ((1usize << (_la - 59))
                                    & ((1usize << (INCR - 59))
                                        | (1usize << (DECR - 59))
                                        | (1usize << (OCTAL - 59))
                                        | (1usize << (HEX - 59))
                                        | (1usize << (INTEGER - 59))
                                        | (1usize << (DECIMAL - 59))
                                        | (1usize << (STRING - 59))
                                        | (1usize << (REGEX - 59))
                                        | (1usize << (TRUE - 59))
                                        | (1usize << (FALSE - 59))
                                        | (1usize << (NULL - 59))
                                        | (1usize << (ID - 59))))
                                    != 0)
                        {
                            {
                                /*InvokeRule expression*/
                                recog.base.set_state(191);
                                recog.expression()?;
                            }
                        }
                    }
                }
                6 => {
                    let tmp = ThrowContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 6);
                    _localctx = tmp;
                    {
                        recog.base.set_state(194);
                        recog.base.match_token(THROW, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(195);
                        recog.expression()?;
                    }
                }
                7 => {
                    let tmp = ExprContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 7);
                    _localctx = tmp;
                    {
                        /*InvokeRule expression*/
                        recog.base.set_state(196);
                        recog.expression()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- trailer ----------------
pub type TrailerContextAll<'input> = TrailerContext<'input>;

pub type TrailerContext<'input> = BaseParserRuleContext<'input, TrailerContextExt<'input>>;

#[derive(Clone)]
pub struct TrailerContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for TrailerContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for TrailerContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_trailer(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_trailer(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for TrailerContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_trailer(self);
    }
}

impl<'input> CustomRuleContext<'input> for TrailerContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_trailer
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_trailer }
}
antlr_rust::tid! {TrailerContextExt<'a>}

impl<'input> TrailerContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TrailerContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TrailerContextExt { ph: PhantomData },
        ))
    }
}

pub trait TrailerContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<TrailerContextExt<'input>>
{
    fn block(&self) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn statement(&self) -> Option<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> TrailerContextAttrs<'input> for TrailerContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn trailer(&mut self) -> Result<Rc<TrailerContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = TrailerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_trailer);
        let mut _localctx: Rc<TrailerContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(201);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                LBRACK => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule block*/
                        recog.base.set_state(199);
                        recog.block()?;
                    }
                }

                LBRACE | LP | DOLLAR | IF | WHILE | DO | FOR | CONTINUE | BREAK | RETURN | NEW
                | TRY | THROW | BOOLNOT | BWNOT | ADD | SUB | INCR | DECR | OCTAL | HEX
                | INTEGER | DECIMAL | STRING | REGEX | TRUE | FALSE | NULL | PRIMITIVE | DEF
                | ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule statement*/
                        recog.base.set_state(200);
                        recog.statement()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- block ----------------
pub type BlockContextAll<'input> = BlockContext<'input>;

pub type BlockContext<'input> = BaseParserRuleContext<'input, BlockContextExt<'input>>;

#[derive(Clone)]
pub struct BlockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for BlockContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for BlockContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_block(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_block(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for BlockContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_block(self);
    }
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_block
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::tid! {BlockContextExt<'a>}

impl<'input> BlockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<BlockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            BlockContextExt { ph: PhantomData },
        ))
    }
}

pub trait BlockContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<BlockContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LBRACK
    /// Returns `None` if there is no child corresponding to token LBRACK
    fn LBRACK(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACK
    /// Returns `None` if there is no child corresponding to token RBRACK
    fn RBRACK(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACK, 0)
    }
    fn statement_all(&self) -> Vec<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn dstatement(&self) -> Option<Rc<DstatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn block(&mut self) -> Result<Rc<BlockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(203);
                recog.base.match_token(LBRACK, &mut recog.err_handler)?;

                recog.base.set_state(207);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(16, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule statement*/
                                recog.base.set_state(204);
                                recog.statement()?;
                            }
                        }
                    }
                    recog.base.set_state(209);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(16, &mut recog.base)?;
                }
                recog.base.set_state(211);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la - 5) & !0x3f) == 0
                    && ((1usize << (_la - 5))
                        & ((1usize << (LBRACE - 5))
                            | (1usize << (LP - 5))
                            | (1usize << (DOLLAR - 5))
                            | (1usize << (DO - 5))
                            | (1usize << (CONTINUE - 5))
                            | (1usize << (BREAK - 5))
                            | (1usize << (RETURN - 5))
                            | (1usize << (NEW - 5))
                            | (1usize << (THROW - 5))
                            | (1usize << (BOOLNOT - 5))
                            | (1usize << (BWNOT - 5))
                            | (1usize << (ADD - 5))
                            | (1usize << (SUB - 5))))
                        != 0)
                    || (((_la - 59) & !0x3f) == 0
                        && ((1usize << (_la - 59))
                            & ((1usize << (INCR - 59))
                                | (1usize << (DECR - 59))
                                | (1usize << (OCTAL - 59))
                                | (1usize << (HEX - 59))
                                | (1usize << (INTEGER - 59))
                                | (1usize << (DECIMAL - 59))
                                | (1usize << (STRING - 59))
                                | (1usize << (REGEX - 59))
                                | (1usize << (TRUE - 59))
                                | (1usize << (FALSE - 59))
                                | (1usize << (NULL - 59))
                                | (1usize << (PRIMITIVE - 59))
                                | (1usize << (DEF - 59))
                                | (1usize << (ID - 59))))
                            != 0)
                {
                    {
                        /*InvokeRule dstatement*/
                        recog.base.set_state(210);
                        recog.dstatement()?;
                    }
                }

                recog.base.set_state(213);
                recog.base.match_token(RBRACK, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- empty ----------------
pub type EmptyContextAll<'input> = EmptyContext<'input>;

pub type EmptyContext<'input> = BaseParserRuleContext<'input, EmptyContextExt<'input>>;

#[derive(Clone)]
pub struct EmptyContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for EmptyContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for EmptyContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_empty(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_empty(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for EmptyContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_empty(self);
    }
}

impl<'input> CustomRuleContext<'input> for EmptyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_empty
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_empty }
}
antlr_rust::tid! {EmptyContextExt<'a>}

impl<'input> EmptyContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<EmptyContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            EmptyContextExt { ph: PhantomData },
        ))
    }
}

pub trait EmptyContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<EmptyContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token SEMICOLON
    /// Returns `None` if there is no child corresponding to token SEMICOLON
    fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, 0)
    }
}

impl<'input> EmptyContextAttrs<'input> for EmptyContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn empty(&mut self) -> Result<Rc<EmptyContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = EmptyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_empty);
        let mut _localctx: Rc<EmptyContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(215);
                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- initializer ----------------
pub type InitializerContextAll<'input> = InitializerContext<'input>;

pub type InitializerContext<'input> = BaseParserRuleContext<'input, InitializerContextExt<'input>>;

#[derive(Clone)]
pub struct InitializerContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for InitializerContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for InitializerContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_initializer(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_initializer(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for InitializerContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_initializer(self);
    }
}

impl<'input> CustomRuleContext<'input> for InitializerContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_initializer
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_initializer }
}
antlr_rust::tid! {InitializerContextExt<'a>}

impl<'input> InitializerContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<InitializerContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            InitializerContextExt { ph: PhantomData },
        ))
    }
}

pub trait InitializerContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<InitializerContextExt<'input>>
{
    fn declaration(&self) -> Option<Rc<DeclarationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> InitializerContextAttrs<'input> for InitializerContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn initializer(&mut self) -> Result<Rc<InitializerContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = InitializerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 18, RULE_initializer);
        let mut _localctx: Rc<InitializerContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(219);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(18, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule declaration*/
                        recog.base.set_state(217);
                        recog.declaration()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule expression*/
                        recog.base.set_state(218);
                        recog.expression()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- afterthought ----------------
pub type AfterthoughtContextAll<'input> = AfterthoughtContext<'input>;

pub type AfterthoughtContext<'input> =
    BaseParserRuleContext<'input, AfterthoughtContextExt<'input>>;

#[derive(Clone)]
pub struct AfterthoughtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for AfterthoughtContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for AfterthoughtContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_afterthought(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_afterthought(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for AfterthoughtContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_afterthought(self);
    }
}

impl<'input> CustomRuleContext<'input> for AfterthoughtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_afterthought
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_afterthought }
}
antlr_rust::tid! {AfterthoughtContextExt<'a>}

impl<'input> AfterthoughtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AfterthoughtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AfterthoughtContextExt { ph: PhantomData },
        ))
    }
}

pub trait AfterthoughtContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<AfterthoughtContextExt<'input>>
{
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> AfterthoughtContextAttrs<'input> for AfterthoughtContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn afterthought(&mut self) -> Result<Rc<AfterthoughtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AfterthoughtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 20, RULE_afterthought);
        let mut _localctx: Rc<AfterthoughtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule expression*/
                recog.base.set_state(221);
                recog.expression()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- declaration ----------------
pub type DeclarationContextAll<'input> = DeclarationContext<'input>;

pub type DeclarationContext<'input> = BaseParserRuleContext<'input, DeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for DeclarationContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for DeclarationContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_declaration(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_declaration(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for DeclarationContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_declaration(self);
    }
}

impl<'input> CustomRuleContext<'input> for DeclarationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_declaration
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_declaration }
}
antlr_rust::tid! {DeclarationContextExt<'a>}

impl<'input> DeclarationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DeclarationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DeclarationContextExt { ph: PhantomData },
        ))
    }
}

pub trait DeclarationContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<DeclarationContextExt<'input>>
{
    fn decltype(&self) -> Option<Rc<DecltypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn declvar_all(&self) -> Vec<Rc<DeclvarContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn declvar(&self, i: usize) -> Option<Rc<DeclvarContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> DeclarationContextAttrs<'input> for DeclarationContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn declaration(&mut self) -> Result<Rc<DeclarationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 22, RULE_declaration);
        let mut _localctx: Rc<DeclarationContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule decltype*/
                recog.base.set_state(223);
                recog.decltype()?;

                /*InvokeRule declvar*/
                recog.base.set_state(224);
                recog.declvar()?;

                recog.base.set_state(229);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(225);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule declvar*/
                            recog.base.set_state(226);
                            recog.declvar()?;
                        }
                    }
                    recog.base.set_state(231);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- decltype ----------------
pub type DecltypeContextAll<'input> = DecltypeContext<'input>;

pub type DecltypeContext<'input> = BaseParserRuleContext<'input, DecltypeContextExt<'input>>;

#[derive(Clone)]
pub struct DecltypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for DecltypeContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for DecltypeContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_decltype(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_decltype(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for DecltypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_decltype(self);
    }
}

impl<'input> CustomRuleContext<'input> for DecltypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_decltype
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_decltype }
}
antlr_rust::tid! {DecltypeContextExt<'a>}

impl<'input> DecltypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DecltypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DecltypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait DecltypeContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<DecltypeContextExt<'input>>
{
    fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token LBRACE in current rule
    fn LBRACE_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token LBRACE, starting from 0.
    /// Returns `None` if number of children corresponding to token LBRACE is less or equal than `i`.
    fn LBRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token RBRACE in current rule
    fn RBRACE_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token RBRACE, starting from 0.
    /// Returns `None` if number of children corresponding to token RBRACE is less or equal than `i`.
    fn RBRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, i)
    }
}

impl<'input> DecltypeContextAttrs<'input> for DecltypeContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn decltype(&mut self) -> Result<Rc<DecltypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DecltypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_decltype);
        let mut _localctx: Rc<DecltypeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule type_*/
                recog.base.set_state(232);
                recog.type_()?;

                recog.base.set_state(237);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(20, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(233);
                                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                                recog.base.set_state(234);
                                recog.base.match_token(RBRACE, &mut recog.err_handler)?;
                            }
                        }
                    }
                    recog.base.set_state(239);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(20, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- type_ ----------------
pub type Type_ContextAll<'input> = Type_Context<'input>;

pub type Type_Context<'input> = BaseParserRuleContext<'input, Type_ContextExt<'input>>;

#[derive(Clone)]
pub struct Type_ContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for Type_Context<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for Type_Context<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_type_(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_type_(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for Type_Context<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_type_(self);
    }
}

impl<'input> CustomRuleContext<'input> for Type_ContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_type_
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_type_ }
}
antlr_rust::tid! {Type_ContextExt<'a>}

impl<'input> Type_ContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Type_ContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Type_ContextExt { ph: PhantomData },
        ))
    }
}

pub trait Type_ContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<Type_ContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DEF
    /// Returns `None` if there is no child corresponding to token DEF
    fn DEF(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DEF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PRIMITIVE
    /// Returns `None` if there is no child corresponding to token PRIMITIVE
    fn PRIMITIVE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PRIMITIVE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOTID in current rule
    fn DOTID_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOTID, starting from 0.
    /// Returns `None` if number of children corresponding to token DOTID is less or equal than `i`.
    fn DOTID(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOTID, i)
    }
}

impl<'input> Type_ContextAttrs<'input> for Type_Context<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn type_(&mut self) -> Result<Rc<Type_ContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Type_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_type_);
        let mut _localctx: Rc<Type_ContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            recog.base.set_state(250);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                DEF => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(240);
                        recog.base.match_token(DEF, &mut recog.err_handler)?;
                    }
                }

                PRIMITIVE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(241);
                        recog.base.match_token(PRIMITIVE, &mut recog.err_handler)?;
                    }
                }

                ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(242);
                        recog.base.match_token(ID, &mut recog.err_handler)?;

                        recog.base.set_state(247);
                        recog.err_handler.sync(&mut recog.base)?;
                        _alt = recog.interpreter.adaptive_predict(21, &mut recog.base)?;
                        while { _alt != 2 && _alt != INVALID_ALT } {
                            if _alt == 1 {
                                {
                                    {
                                        recog.base.set_state(243);
                                        recog.base.match_token(DOT, &mut recog.err_handler)?;

                                        recog.base.set_state(244);
                                        recog.base.match_token(DOTID, &mut recog.err_handler)?;
                                    }
                                }
                            }
                            recog.base.set_state(249);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(21, &mut recog.base)?;
                        }
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- declvar ----------------
pub type DeclvarContextAll<'input> = DeclvarContext<'input>;

pub type DeclvarContext<'input> = BaseParserRuleContext<'input, DeclvarContextExt<'input>>;

#[derive(Clone)]
pub struct DeclvarContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for DeclvarContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for DeclvarContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_declvar(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_declvar(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for DeclvarContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_declvar(self);
    }
}

impl<'input> CustomRuleContext<'input> for DeclvarContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_declvar
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_declvar }
}
antlr_rust::tid! {DeclvarContextExt<'a>}

impl<'input> DeclvarContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DeclvarContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DeclvarContextExt { ph: PhantomData },
        ))
    }
}

pub trait DeclvarContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<DeclvarContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> DeclvarContextAttrs<'input> for DeclvarContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn declvar(&mut self) -> Result<Rc<DeclvarContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DeclvarContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_declvar);
        let mut _localctx: Rc<DeclvarContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(252);
                recog.base.match_token(ID, &mut recog.err_handler)?;

                recog.base.set_state(255);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ASSIGN {
                    {
                        recog.base.set_state(253);
                        recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(254);
                        recog.expression()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- trap ----------------
pub type TrapContextAll<'input> = TrapContext<'input>;

pub type TrapContext<'input> = BaseParserRuleContext<'input, TrapContextExt<'input>>;

#[derive(Clone)]
pub struct TrapContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for TrapContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for TrapContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_trap(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_trap(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for TrapContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_trap(self);
    }
}

impl<'input> CustomRuleContext<'input> for TrapContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_trap
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_trap }
}
antlr_rust::tid! {TrapContextExt<'a>}

impl<'input> TrapContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TrapContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TrapContextExt { ph: PhantomData },
        ))
    }
}

pub trait TrapContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<TrapContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CATCH
    /// Returns `None` if there is no child corresponding to token CATCH
    fn CATCH(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CATCH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> TrapContextAttrs<'input> for TrapContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn trap(&mut self) -> Result<Rc<TrapContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = TrapContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_trap);
        let mut _localctx: Rc<TrapContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(257);
                recog.base.match_token(CATCH, &mut recog.err_handler)?;

                recog.base.set_state(258);
                recog.base.match_token(LP, &mut recog.err_handler)?;

                /*InvokeRule type_*/
                recog.base.set_state(259);
                recog.type_()?;

                recog.base.set_state(260);
                recog.base.match_token(ID, &mut recog.err_handler)?;

                recog.base.set_state(261);
                recog.base.match_token(RP, &mut recog.err_handler)?;

                /*InvokeRule block*/
                recog.base.set_state(262);
                recog.block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- noncondexpression ----------------
#[derive(Debug)]
pub enum NoncondexpressionContextAll<'input> {
    SingleContext(SingleContext<'input>),
    CompContext(CompContext<'input>),
    BoolContext(BoolContext<'input>),
    BinaryContext(BinaryContext<'input>),
    ElvisContext(ElvisContext<'input>),
    InstanceofContext(InstanceofContext<'input>),
    Error(NoncondexpressionContext<'input>),
}
antlr_rust::tid! {NoncondexpressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for NoncondexpressionContextAll<'input> {}

impl<'input> PainlessParserContext<'input> for NoncondexpressionContextAll<'input> {}

impl<'input> Deref for NoncondexpressionContextAll<'input> {
    type Target = dyn NoncondexpressionContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use NoncondexpressionContextAll::*;
        match self {
            SingleContext(inner) => inner,
            CompContext(inner) => inner,
            BoolContext(inner) => inner,
            BinaryContext(inner) => inner,
            ElvisContext(inner) => inner,
            InstanceofContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for NoncondexpressionContextAll<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for NoncondexpressionContextAll<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type NoncondexpressionContext<'input> =
    BaseParserRuleContext<'input, NoncondexpressionContextExt<'input>>;

#[derive(Clone)]
pub struct NoncondexpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for NoncondexpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for NoncondexpressionContext<'input>
{
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for NoncondexpressionContext<'input>
{
}

impl<'input> CustomRuleContext<'input> for NoncondexpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_noncondexpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_noncondexpression }
}
antlr_rust::tid! {NoncondexpressionContextExt<'a>}

impl<'input> NoncondexpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<NoncondexpressionContextAll<'input>> {
        Rc::new(NoncondexpressionContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                NoncondexpressionContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait NoncondexpressionContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<NoncondexpressionContextExt<'input>>
{
}

impl<'input> NoncondexpressionContextAttrs<'input> for NoncondexpressionContext<'input> {}

pub type SingleContext<'input> = BaseParserRuleContext<'input, SingleContextExt<'input>>;

pub trait SingleContextAttrs<'input>: PainlessParserContext<'input> {
    fn unary(&self) -> Option<Rc<UnaryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> SingleContextAttrs<'input> for SingleContext<'input> {}

pub struct SingleContextExt<'input> {
    base: NoncondexpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {SingleContextExt<'a>}

impl<'input> PainlessParserContext<'input> for SingleContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for SingleContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_single(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_single(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for SingleContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_single(self);
    }
}

impl<'input> CustomRuleContext<'input> for SingleContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_noncondexpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_noncondexpression }
}

impl<'input> Borrow<NoncondexpressionContextExt<'input>> for SingleContext<'input> {
    fn borrow(&self) -> &NoncondexpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<NoncondexpressionContextExt<'input>> for SingleContext<'input> {
    fn borrow_mut(&mut self) -> &mut NoncondexpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> NoncondexpressionContextAttrs<'input> for SingleContext<'input> {}

impl<'input> SingleContextExt<'input> {
    fn new(
        ctx: &dyn NoncondexpressionContextAttrs<'input>,
    ) -> Rc<NoncondexpressionContextAll<'input>> {
        Rc::new(NoncondexpressionContextAll::SingleContext(
            BaseParserRuleContext::copy_from(
                ctx,
                SingleContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type CompContext<'input> = BaseParserRuleContext<'input, CompContextExt<'input>>;

pub trait CompContextAttrs<'input>: PainlessParserContext<'input> {
    fn noncondexpression_all(&self) -> Vec<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn noncondexpression(&self, i: usize) -> Option<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LTE
    /// Returns `None` if there is no child corresponding to token LTE
    fn LTE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LTE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GTE
    /// Returns `None` if there is no child corresponding to token GTE
    fn GTE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GTE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EQ
    /// Returns `None` if there is no child corresponding to token EQ
    fn EQ(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EQR
    /// Returns `None` if there is no child corresponding to token EQR
    fn EQR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EQR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NE
    /// Returns `None` if there is no child corresponding to token NE
    fn NE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NER
    /// Returns `None` if there is no child corresponding to token NER
    fn NER(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NER, 0)
    }
}

impl<'input> CompContextAttrs<'input> for CompContext<'input> {}

pub struct CompContextExt<'input> {
    base: NoncondexpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {CompContextExt<'a>}

impl<'input> PainlessParserContext<'input> for CompContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for CompContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_comp(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_comp(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for CompContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_comp(self);
    }
}

impl<'input> CustomRuleContext<'input> for CompContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_noncondexpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_noncondexpression }
}

impl<'input> Borrow<NoncondexpressionContextExt<'input>> for CompContext<'input> {
    fn borrow(&self) -> &NoncondexpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<NoncondexpressionContextExt<'input>> for CompContext<'input> {
    fn borrow_mut(&mut self) -> &mut NoncondexpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> NoncondexpressionContextAttrs<'input> for CompContext<'input> {}

impl<'input> CompContextExt<'input> {
    fn new(
        ctx: &dyn NoncondexpressionContextAttrs<'input>,
    ) -> Rc<NoncondexpressionContextAll<'input>> {
        Rc::new(NoncondexpressionContextAll::CompContext(
            BaseParserRuleContext::copy_from(
                ctx,
                CompContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type BoolContext<'input> = BaseParserRuleContext<'input, BoolContextExt<'input>>;

pub trait BoolContextAttrs<'input>: PainlessParserContext<'input> {
    fn noncondexpression_all(&self) -> Vec<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn noncondexpression(&self, i: usize) -> Option<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token BOOLAND
    /// Returns `None` if there is no child corresponding to token BOOLAND
    fn BOOLAND(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BOOLAND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BOOLOR
    /// Returns `None` if there is no child corresponding to token BOOLOR
    fn BOOLOR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BOOLOR, 0)
    }
}

impl<'input> BoolContextAttrs<'input> for BoolContext<'input> {}

pub struct BoolContextExt<'input> {
    base: NoncondexpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {BoolContextExt<'a>}

impl<'input> PainlessParserContext<'input> for BoolContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for BoolContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_bool(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_bool(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for BoolContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_bool(self);
    }
}

impl<'input> CustomRuleContext<'input> for BoolContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_noncondexpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_noncondexpression }
}

impl<'input> Borrow<NoncondexpressionContextExt<'input>> for BoolContext<'input> {
    fn borrow(&self) -> &NoncondexpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<NoncondexpressionContextExt<'input>> for BoolContext<'input> {
    fn borrow_mut(&mut self) -> &mut NoncondexpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> NoncondexpressionContextAttrs<'input> for BoolContext<'input> {}

impl<'input> BoolContextExt<'input> {
    fn new(
        ctx: &dyn NoncondexpressionContextAttrs<'input>,
    ) -> Rc<NoncondexpressionContextAll<'input>> {
        Rc::new(NoncondexpressionContextAll::BoolContext(
            BaseParserRuleContext::copy_from(
                ctx,
                BoolContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type BinaryContext<'input> = BaseParserRuleContext<'input, BinaryContextExt<'input>>;

pub trait BinaryContextAttrs<'input>: PainlessParserContext<'input> {
    fn noncondexpression_all(&self) -> Vec<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn noncondexpression(&self, i: usize) -> Option<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token MUL
    /// Returns `None` if there is no child corresponding to token MUL
    fn MUL(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MUL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DIV
    /// Returns `None` if there is no child corresponding to token DIV
    fn DIV(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DIV, 0)
    }
    /// Retrieves first TerminalNode corresponding to token REM
    /// Returns `None` if there is no child corresponding to token REM
    fn REM(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REM, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ADD
    /// Returns `None` if there is no child corresponding to token ADD
    fn ADD(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ADD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SUB
    /// Returns `None` if there is no child corresponding to token SUB
    fn SUB(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SUB, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FIND
    /// Returns `None` if there is no child corresponding to token FIND
    fn FIND(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FIND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MATCH
    /// Returns `None` if there is no child corresponding to token MATCH
    fn MATCH(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LSH
    /// Returns `None` if there is no child corresponding to token LSH
    fn LSH(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LSH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RSH
    /// Returns `None` if there is no child corresponding to token RSH
    fn RSH(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RSH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token USH
    /// Returns `None` if there is no child corresponding to token USH
    fn USH(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(USH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BWAND
    /// Returns `None` if there is no child corresponding to token BWAND
    fn BWAND(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BWAND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token XOR
    /// Returns `None` if there is no child corresponding to token XOR
    fn XOR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(XOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BWOR
    /// Returns `None` if there is no child corresponding to token BWOR
    fn BWOR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BWOR, 0)
    }
}

impl<'input> BinaryContextAttrs<'input> for BinaryContext<'input> {}

pub struct BinaryContextExt<'input> {
    base: NoncondexpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {BinaryContextExt<'a>}

impl<'input> PainlessParserContext<'input> for BinaryContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for BinaryContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_binary(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_binary(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for BinaryContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_binary(self);
    }
}

impl<'input> CustomRuleContext<'input> for BinaryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_noncondexpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_noncondexpression }
}

impl<'input> Borrow<NoncondexpressionContextExt<'input>> for BinaryContext<'input> {
    fn borrow(&self) -> &NoncondexpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<NoncondexpressionContextExt<'input>> for BinaryContext<'input> {
    fn borrow_mut(&mut self) -> &mut NoncondexpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> NoncondexpressionContextAttrs<'input> for BinaryContext<'input> {}

impl<'input> BinaryContextExt<'input> {
    fn new(
        ctx: &dyn NoncondexpressionContextAttrs<'input>,
    ) -> Rc<NoncondexpressionContextAll<'input>> {
        Rc::new(NoncondexpressionContextAll::BinaryContext(
            BaseParserRuleContext::copy_from(
                ctx,
                BinaryContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ElvisContext<'input> = BaseParserRuleContext<'input, ElvisContextExt<'input>>;

pub trait ElvisContextAttrs<'input>: PainlessParserContext<'input> {
    fn noncondexpression_all(&self) -> Vec<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn noncondexpression(&self, i: usize) -> Option<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ELVIS
    /// Returns `None` if there is no child corresponding to token ELVIS
    fn ELVIS(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELVIS, 0)
    }
}

impl<'input> ElvisContextAttrs<'input> for ElvisContext<'input> {}

pub struct ElvisContextExt<'input> {
    base: NoncondexpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ElvisContextExt<'a>}

impl<'input> PainlessParserContext<'input> for ElvisContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ElvisContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_elvis(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_elvis(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ElvisContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_elvis(self);
    }
}

impl<'input> CustomRuleContext<'input> for ElvisContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_noncondexpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_noncondexpression }
}

impl<'input> Borrow<NoncondexpressionContextExt<'input>> for ElvisContext<'input> {
    fn borrow(&self) -> &NoncondexpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<NoncondexpressionContextExt<'input>> for ElvisContext<'input> {
    fn borrow_mut(&mut self) -> &mut NoncondexpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> NoncondexpressionContextAttrs<'input> for ElvisContext<'input> {}

impl<'input> ElvisContextExt<'input> {
    fn new(
        ctx: &dyn NoncondexpressionContextAttrs<'input>,
    ) -> Rc<NoncondexpressionContextAll<'input>> {
        Rc::new(NoncondexpressionContextAll::ElvisContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ElvisContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type InstanceofContext<'input> = BaseParserRuleContext<'input, InstanceofContextExt<'input>>;

pub trait InstanceofContextAttrs<'input>: PainlessParserContext<'input> {
    fn noncondexpression(&self) -> Option<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INSTANCEOF
    /// Returns `None` if there is no child corresponding to token INSTANCEOF
    fn INSTANCEOF(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INSTANCEOF, 0)
    }
    fn decltype(&self) -> Option<Rc<DecltypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> InstanceofContextAttrs<'input> for InstanceofContext<'input> {}

pub struct InstanceofContextExt<'input> {
    base: NoncondexpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {InstanceofContextExt<'a>}

impl<'input> PainlessParserContext<'input> for InstanceofContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for InstanceofContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_instanceof(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_instanceof(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for InstanceofContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_instanceof(self);
    }
}

impl<'input> CustomRuleContext<'input> for InstanceofContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_noncondexpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_noncondexpression }
}

impl<'input> Borrow<NoncondexpressionContextExt<'input>> for InstanceofContext<'input> {
    fn borrow(&self) -> &NoncondexpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<NoncondexpressionContextExt<'input>> for InstanceofContext<'input> {
    fn borrow_mut(&mut self) -> &mut NoncondexpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> NoncondexpressionContextAttrs<'input> for InstanceofContext<'input> {}

impl<'input> InstanceofContextExt<'input> {
    fn new(
        ctx: &dyn NoncondexpressionContextAttrs<'input>,
    ) -> Rc<NoncondexpressionContextAll<'input>> {
        Rc::new(NoncondexpressionContextAll::InstanceofContext(
            BaseParserRuleContext::copy_from(
                ctx,
                InstanceofContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn noncondexpression(
        &mut self,
    ) -> Result<Rc<NoncondexpressionContextAll<'input>>, ANTLRError> {
        self.noncondexpression_rec(0)
    }

    fn noncondexpression_rec(
        &mut self,
        _p: isize,
    ) -> Result<Rc<NoncondexpressionContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx =
            NoncondexpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 32, RULE_noncondexpression, _p);
        let mut _localctx: Rc<NoncondexpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 32;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    let mut tmp = SingleContextExt::new(&**_localctx);
                    recog.ctx = Some(tmp.clone());
                    _localctx = tmp;
                    _prevctx = _localctx.clone();

                    /*InvokeRule unary*/
                    recog.base.set_state(265);
                    recog.unary()?;
                }

                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(308);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(25, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(306);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(24, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BinaryContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(267);
                                        if !({ recog.precpred(None, 13) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 13)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(268);
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la - 31) & !0x3f) == 0
                                                && ((1usize << (_la - 31))
                                                    & ((1usize << (MUL - 31))
                                                        | (1usize << (DIV - 31))
                                                        | (1usize << (REM - 31))))
                                                    != 0)
                                        } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(269);
                                        recog.noncondexpression_rec(14)?;
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BinaryContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(270);
                                        if !({ recog.precpred(None, 12) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 12)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(271);
                                        _la = recog.base.input.la(1);
                                        if { !(_la == ADD || _la == SUB) } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(272);
                                        recog.noncondexpression_rec(13)?;
                                    }
                                }
                                3 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BinaryContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(273);
                                        if !({ recog.precpred(None, 11) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 11)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(274);
                                        _la = recog.base.input.la(1);
                                        if { !(_la == FIND || _la == MATCH) } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(275);
                                        recog.noncondexpression_rec(12)?;
                                    }
                                }
                                4 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BinaryContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(276);
                                        if !({ recog.precpred(None, 10) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 10)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(277);
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la - 36) & !0x3f) == 0
                                                && ((1usize << (_la - 36))
                                                    & ((1usize << (LSH - 36))
                                                        | (1usize << (RSH - 36))
                                                        | (1usize << (USH - 36))))
                                                    != 0)
                                        } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(278);
                                        recog.noncondexpression_rec(11)?;
                                    }
                                }
                                5 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = CompContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(279);
                                        if !({ recog.precpred(None, 9) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 9)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(280);
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la - 39) & !0x3f) == 0
                                                && ((1usize << (_la - 39))
                                                    & ((1usize << (LT - 39))
                                                        | (1usize << (LTE - 39))
                                                        | (1usize << (GT - 39))
                                                        | (1usize << (GTE - 39))))
                                                    != 0)
                                        } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(281);
                                        recog.noncondexpression_rec(10)?;
                                    }
                                }
                                6 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = CompContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(282);
                                        if !({ recog.precpred(None, 7) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 7)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(283);
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la - 43) & !0x3f) == 0
                                                && ((1usize << (_la - 43))
                                                    & ((1usize << (EQ - 43))
                                                        | (1usize << (EQR - 43))
                                                        | (1usize << (NE - 43))
                                                        | (1usize << (NER - 43))))
                                                    != 0)
                                        } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(284);
                                        recog.noncondexpression_rec(8)?;
                                    }
                                }
                                7 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BinaryContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(285);
                                        if !({ recog.precpred(None, 6) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 6)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(286);
                                        recog.base.match_token(BWAND, &mut recog.err_handler)?;

                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(287);
                                        recog.noncondexpression_rec(7)?;
                                    }
                                }
                                8 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BinaryContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(288);
                                        if !({ recog.precpred(None, 5) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 5)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(289);
                                        recog.base.match_token(XOR, &mut recog.err_handler)?;

                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(290);
                                        recog.noncondexpression_rec(6)?;
                                    }
                                }
                                9 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BinaryContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(291);
                                        if !({ recog.precpred(None, 4) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 4)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(292);
                                        recog.base.match_token(BWOR, &mut recog.err_handler)?;

                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(293);
                                        recog.noncondexpression_rec(5)?;
                                    }
                                }
                                10 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BoolContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(294);
                                        if !({ recog.precpred(None, 3) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 3)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(295);
                                        recog.base.match_token(BOOLAND, &mut recog.err_handler)?;

                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(296);
                                        recog.noncondexpression_rec(4)?;
                                    }
                                }
                                11 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = BoolContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(297);
                                        if !({ recog.precpred(None, 2) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 2)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(298);
                                        recog.base.match_token(BOOLOR, &mut recog.err_handler)?;

                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(299);
                                        recog.noncondexpression_rec(3)?;
                                    }
                                }
                                12 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = ElvisContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(300);
                                        if !({ recog.precpred(None, 1) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 1)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(301);
                                        recog.base.match_token(ELVIS, &mut recog.err_handler)?;

                                        /*InvokeRule noncondexpression*/
                                        recog.base.set_state(302);
                                        recog.noncondexpression_rec(1)?;
                                    }
                                }
                                13 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = InstanceofContextExt::new(
                                            &**NoncondexpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_noncondexpression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(303);
                                        if !({ recog.precpred(None, 8) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 8)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(304);
                                        recog
                                            .base
                                            .match_token(INSTANCEOF, &mut recog.err_handler)?;

                                        /*InvokeRule decltype*/
                                        recog.base.set_state(305);
                                        recog.decltype()?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(310);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(25, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- expression ----------------
#[derive(Debug)]
pub enum ExpressionContextAll<'input> {
    ConditionalContext(ConditionalContext<'input>),
    AssignmentContext(AssignmentContext<'input>),
    NonconditionalContext(NonconditionalContext<'input>),
    Error(ExpressionContext<'input>),
}
antlr_rust::tid! {ExpressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExpressionContextAll<'input> {}

impl<'input> PainlessParserContext<'input> for ExpressionContextAll<'input> {}

impl<'input> Deref for ExpressionContextAll<'input> {
    type Target = dyn ExpressionContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use ExpressionContextAll::*;
        match self {
            ConditionalContext(inner) => inner,
            AssignmentContext(inner) => inner,
            NonconditionalContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for ExpressionContextAll<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for ExpressionContextAll<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type ExpressionContext<'input> = BaseParserRuleContext<'input, ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for ExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ExpressionContext<'input> {}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ExpressionContext<'input> {}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid! {ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                ExpressionContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait ExpressionContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>
{
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input> {}

pub type ConditionalContext<'input> = BaseParserRuleContext<'input, ConditionalContextExt<'input>>;

pub trait ConditionalContextAttrs<'input>: PainlessParserContext<'input> {
    fn noncondexpression(&self) -> Option<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COND
    /// Returns `None` if there is no child corresponding to token COND
    fn COND(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COND, 0)
    }
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
}

impl<'input> ConditionalContextAttrs<'input> for ConditionalContext<'input> {}

pub struct ConditionalContextExt<'input> {
    base: ExpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ConditionalContextExt<'a>}

impl<'input> PainlessParserContext<'input> for ConditionalContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for ConditionalContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_conditional(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_conditional(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ConditionalContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_conditional(self);
    }
}

impl<'input> CustomRuleContext<'input> for ConditionalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ConditionalContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ConditionalContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ConditionalContext<'input> {}

impl<'input> ConditionalContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ConditionalContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ConditionalContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type AssignmentContext<'input> = BaseParserRuleContext<'input, AssignmentContextExt<'input>>;

pub trait AssignmentContextAttrs<'input>: PainlessParserContext<'input> {
    fn noncondexpression(&self) -> Option<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AADD
    /// Returns `None` if there is no child corresponding to token AADD
    fn AADD(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AADD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASUB
    /// Returns `None` if there is no child corresponding to token ASUB
    fn ASUB(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASUB, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AMUL
    /// Returns `None` if there is no child corresponding to token AMUL
    fn AMUL(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AMUL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ADIV
    /// Returns `None` if there is no child corresponding to token ADIV
    fn ADIV(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ADIV, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AREM
    /// Returns `None` if there is no child corresponding to token AREM
    fn AREM(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AREM, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AAND
    /// Returns `None` if there is no child corresponding to token AAND
    fn AAND(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AAND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AXOR
    /// Returns `None` if there is no child corresponding to token AXOR
    fn AXOR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AXOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AOR
    /// Returns `None` if there is no child corresponding to token AOR
    fn AOR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ALSH
    /// Returns `None` if there is no child corresponding to token ALSH
    fn ALSH(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ALSH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ARSH
    /// Returns `None` if there is no child corresponding to token ARSH
    fn ARSH(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ARSH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AUSH
    /// Returns `None` if there is no child corresponding to token AUSH
    fn AUSH(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AUSH, 0)
    }
}

impl<'input> AssignmentContextAttrs<'input> for AssignmentContext<'input> {}

pub struct AssignmentContextExt<'input> {
    base: ExpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {AssignmentContextExt<'a>}

impl<'input> PainlessParserContext<'input> for AssignmentContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for AssignmentContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_assignment(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_assignment(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for AssignmentContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_assignment(self);
    }
}

impl<'input> CustomRuleContext<'input> for AssignmentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for AssignmentContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for AssignmentContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for AssignmentContext<'input> {}

impl<'input> AssignmentContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::AssignmentContext(
            BaseParserRuleContext::copy_from(
                ctx,
                AssignmentContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type NonconditionalContext<'input> =
    BaseParserRuleContext<'input, NonconditionalContextExt<'input>>;

pub trait NonconditionalContextAttrs<'input>: PainlessParserContext<'input> {
    fn noncondexpression(&self) -> Option<Rc<NoncondexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> NonconditionalContextAttrs<'input> for NonconditionalContext<'input> {}

pub struct NonconditionalContextExt<'input> {
    base: ExpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {NonconditionalContextExt<'a>}

impl<'input> PainlessParserContext<'input> for NonconditionalContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for NonconditionalContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_nonconditional(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_nonconditional(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for NonconditionalContext<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_nonconditional(self);
    }
}

impl<'input> CustomRuleContext<'input> for NonconditionalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for NonconditionalContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for NonconditionalContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for NonconditionalContext<'input> {}

impl<'input> NonconditionalContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::NonconditionalContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NonconditionalContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn expression(&mut self) -> Result<Rc<ExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 34, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(322);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(26, &mut recog.base)? {
                1 => {
                    let tmp = NonconditionalContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        /*InvokeRule noncondexpression*/
                        recog.base.set_state(311);
                        recog.noncondexpression_rec(0)?;
                    }
                }
                2 => {
                    let tmp = ConditionalContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        /*InvokeRule noncondexpression*/
                        recog.base.set_state(312);
                        recog.noncondexpression_rec(0)?;

                        recog.base.set_state(313);
                        recog.base.match_token(COND, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(314);
                        recog.expression()?;

                        recog.base.set_state(315);
                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(316);
                        recog.expression()?;
                    }
                }
                3 => {
                    let tmp = AssignmentContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        /*InvokeRule noncondexpression*/
                        recog.base.set_state(318);
                        recog.noncondexpression_rec(0)?;

                        recog.base.set_state(319);
                        _la = recog.base.input.la(1);
                        if {
                            !(((_la - 61) & !0x3f) == 0
                                && ((1usize << (_la - 61))
                                    & ((1usize << (ASSIGN - 61))
                                        | (1usize << (AADD - 61))
                                        | (1usize << (ASUB - 61))
                                        | (1usize << (AMUL - 61))
                                        | (1usize << (ADIV - 61))
                                        | (1usize << (AREM - 61))
                                        | (1usize << (AAND - 61))
                                        | (1usize << (AXOR - 61))
                                        | (1usize << (AOR - 61))
                                        | (1usize << (ALSH - 61))
                                        | (1usize << (ARSH - 61))
                                        | (1usize << (AUSH - 61))))
                                    != 0)
                        } {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                        /*InvokeRule expression*/
                        recog.base.set_state(320);
                        recog.expression()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- unary ----------------
#[derive(Debug)]
pub enum UnaryContextAll<'input> {
    NotaddsubContext(NotaddsubContext<'input>),
    PreContext(PreContext<'input>),
    AddsubContext(AddsubContext<'input>),
    Error(UnaryContext<'input>),
}
antlr_rust::tid! {UnaryContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for UnaryContextAll<'input> {}

impl<'input> PainlessParserContext<'input> for UnaryContextAll<'input> {}

impl<'input> Deref for UnaryContextAll<'input> {
    type Target = dyn UnaryContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use UnaryContextAll::*;
        match self {
            NotaddsubContext(inner) => inner,
            PreContext(inner) => inner,
            AddsubContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for UnaryContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for UnaryContextAll<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type UnaryContext<'input> = BaseParserRuleContext<'input, UnaryContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for UnaryContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for UnaryContext<'input> {}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for UnaryContext<'input> {}

impl<'input> CustomRuleContext<'input> for UnaryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unary }
}
antlr_rust::tid! {UnaryContextExt<'a>}

impl<'input> UnaryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<UnaryContextAll<'input>> {
        Rc::new(UnaryContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                UnaryContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait UnaryContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<UnaryContextExt<'input>>
{
}

impl<'input> UnaryContextAttrs<'input> for UnaryContext<'input> {}

pub type NotaddsubContext<'input> = BaseParserRuleContext<'input, NotaddsubContextExt<'input>>;

pub trait NotaddsubContextAttrs<'input>: PainlessParserContext<'input> {
    fn unarynotaddsub(&self) -> Option<Rc<UnarynotaddsubContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> NotaddsubContextAttrs<'input> for NotaddsubContext<'input> {}

pub struct NotaddsubContextExt<'input> {
    base: UnaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {NotaddsubContextExt<'a>}

impl<'input> PainlessParserContext<'input> for NotaddsubContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for NotaddsubContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_notaddsub(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_notaddsub(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for NotaddsubContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_notaddsub(self);
    }
}

impl<'input> CustomRuleContext<'input> for NotaddsubContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unary }
}

impl<'input> Borrow<UnaryContextExt<'input>> for NotaddsubContext<'input> {
    fn borrow(&self) -> &UnaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<UnaryContextExt<'input>> for NotaddsubContext<'input> {
    fn borrow_mut(&mut self) -> &mut UnaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> UnaryContextAttrs<'input> for NotaddsubContext<'input> {}

impl<'input> NotaddsubContextExt<'input> {
    fn new(ctx: &dyn UnaryContextAttrs<'input>) -> Rc<UnaryContextAll<'input>> {
        Rc::new(UnaryContextAll::NotaddsubContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NotaddsubContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type PreContext<'input> = BaseParserRuleContext<'input, PreContextExt<'input>>;

pub trait PreContextAttrs<'input>: PainlessParserContext<'input> {
    fn chain(&self) -> Option<Rc<ChainContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INCR
    /// Returns `None` if there is no child corresponding to token INCR
    fn INCR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INCR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DECR
    /// Returns `None` if there is no child corresponding to token DECR
    fn DECR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DECR, 0)
    }
}

impl<'input> PreContextAttrs<'input> for PreContext<'input> {}

pub struct PreContextExt<'input> {
    base: UnaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {PreContextExt<'a>}

impl<'input> PainlessParserContext<'input> for PreContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for PreContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_pre(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_pre(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for PreContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_pre(self);
    }
}

impl<'input> CustomRuleContext<'input> for PreContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unary }
}

impl<'input> Borrow<UnaryContextExt<'input>> for PreContext<'input> {
    fn borrow(&self) -> &UnaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<UnaryContextExt<'input>> for PreContext<'input> {
    fn borrow_mut(&mut self) -> &mut UnaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> UnaryContextAttrs<'input> for PreContext<'input> {}

impl<'input> PreContextExt<'input> {
    fn new(ctx: &dyn UnaryContextAttrs<'input>) -> Rc<UnaryContextAll<'input>> {
        Rc::new(UnaryContextAll::PreContext(
            BaseParserRuleContext::copy_from(
                ctx,
                PreContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type AddsubContext<'input> = BaseParserRuleContext<'input, AddsubContextExt<'input>>;

pub trait AddsubContextAttrs<'input>: PainlessParserContext<'input> {
    fn unary(&self) -> Option<Rc<UnaryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ADD
    /// Returns `None` if there is no child corresponding to token ADD
    fn ADD(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ADD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SUB
    /// Returns `None` if there is no child corresponding to token SUB
    fn SUB(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SUB, 0)
    }
}

impl<'input> AddsubContextAttrs<'input> for AddsubContext<'input> {}

pub struct AddsubContextExt<'input> {
    base: UnaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {AddsubContextExt<'a>}

impl<'input> PainlessParserContext<'input> for AddsubContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for AddsubContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_addsub(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_addsub(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for AddsubContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_addsub(self);
    }
}

impl<'input> CustomRuleContext<'input> for AddsubContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unary }
}

impl<'input> Borrow<UnaryContextExt<'input>> for AddsubContext<'input> {
    fn borrow(&self) -> &UnaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<UnaryContextExt<'input>> for AddsubContext<'input> {
    fn borrow_mut(&mut self) -> &mut UnaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> UnaryContextAttrs<'input> for AddsubContext<'input> {}

impl<'input> AddsubContextExt<'input> {
    fn new(ctx: &dyn UnaryContextAttrs<'input>) -> Rc<UnaryContextAll<'input>> {
        Rc::new(UnaryContextAll::AddsubContext(
            BaseParserRuleContext::copy_from(
                ctx,
                AddsubContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn unary(&mut self) -> Result<Rc<UnaryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = UnaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_unary);
        let mut _localctx: Rc<UnaryContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(329);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                INCR | DECR => {
                    let tmp = PreContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(324);
                        _la = recog.base.input.la(1);
                        if { !(_la == INCR || _la == DECR) } {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                        /*InvokeRule chain*/
                        recog.base.set_state(325);
                        recog.chain()?;
                    }
                }

                ADD | SUB => {
                    let tmp = AddsubContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(326);
                        _la = recog.base.input.la(1);
                        if { !(_la == ADD || _la == SUB) } {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                        /*InvokeRule unary*/
                        recog.base.set_state(327);
                        recog.unary()?;
                    }
                }

                LBRACE | LP | DOLLAR | NEW | BOOLNOT | BWNOT | OCTAL | HEX | INTEGER | DECIMAL
                | STRING | REGEX | TRUE | FALSE | NULL | ID => {
                    let tmp = NotaddsubContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        /*InvokeRule unarynotaddsub*/
                        recog.base.set_state(328);
                        recog.unarynotaddsub()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- unarynotaddsub ----------------
#[derive(Debug)]
pub enum UnarynotaddsubContextAll<'input> {
    CastContext(CastContext<'input>),
    NotContext(NotContext<'input>),
    ReadContext(ReadContext<'input>),
    PostContext(PostContext<'input>),
    Error(UnarynotaddsubContext<'input>),
}
antlr_rust::tid! {UnarynotaddsubContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for UnarynotaddsubContextAll<'input> {}

impl<'input> PainlessParserContext<'input> for UnarynotaddsubContextAll<'input> {}

impl<'input> Deref for UnarynotaddsubContextAll<'input> {
    type Target = dyn UnarynotaddsubContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use UnarynotaddsubContextAll::*;
        match self {
            CastContext(inner) => inner,
            NotContext(inner) => inner,
            ReadContext(inner) => inner,
            PostContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for UnarynotaddsubContextAll<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for UnarynotaddsubContextAll<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type UnarynotaddsubContext<'input> =
    BaseParserRuleContext<'input, UnarynotaddsubContextExt<'input>>;

#[derive(Clone)]
pub struct UnarynotaddsubContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for UnarynotaddsubContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for UnarynotaddsubContext<'input>
{
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for UnarynotaddsubContext<'input>
{
}

impl<'input> CustomRuleContext<'input> for UnarynotaddsubContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unarynotaddsub
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unarynotaddsub }
}
antlr_rust::tid! {UnarynotaddsubContextExt<'a>}

impl<'input> UnarynotaddsubContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<UnarynotaddsubContextAll<'input>> {
        Rc::new(UnarynotaddsubContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                UnarynotaddsubContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait UnarynotaddsubContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<UnarynotaddsubContextExt<'input>>
{
}

impl<'input> UnarynotaddsubContextAttrs<'input> for UnarynotaddsubContext<'input> {}

pub type CastContext<'input> = BaseParserRuleContext<'input, CastContextExt<'input>>;

pub trait CastContextAttrs<'input>: PainlessParserContext<'input> {
    fn castexpression(&self) -> Option<Rc<CastexpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> CastContextAttrs<'input> for CastContext<'input> {}

pub struct CastContextExt<'input> {
    base: UnarynotaddsubContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {CastContextExt<'a>}

impl<'input> PainlessParserContext<'input> for CastContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for CastContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_cast(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_cast(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for CastContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_cast(self);
    }
}

impl<'input> CustomRuleContext<'input> for CastContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unarynotaddsub
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unarynotaddsub }
}

impl<'input> Borrow<UnarynotaddsubContextExt<'input>> for CastContext<'input> {
    fn borrow(&self) -> &UnarynotaddsubContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<UnarynotaddsubContextExt<'input>> for CastContext<'input> {
    fn borrow_mut(&mut self) -> &mut UnarynotaddsubContextExt<'input> {
        &mut self.base
    }
}

impl<'input> UnarynotaddsubContextAttrs<'input> for CastContext<'input> {}

impl<'input> CastContextExt<'input> {
    fn new(ctx: &dyn UnarynotaddsubContextAttrs<'input>) -> Rc<UnarynotaddsubContextAll<'input>> {
        Rc::new(UnarynotaddsubContextAll::CastContext(
            BaseParserRuleContext::copy_from(
                ctx,
                CastContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type NotContext<'input> = BaseParserRuleContext<'input, NotContextExt<'input>>;

pub trait NotContextAttrs<'input>: PainlessParserContext<'input> {
    fn unary(&self) -> Option<Rc<UnaryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token BOOLNOT
    /// Returns `None` if there is no child corresponding to token BOOLNOT
    fn BOOLNOT(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BOOLNOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BWNOT
    /// Returns `None` if there is no child corresponding to token BWNOT
    fn BWNOT(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BWNOT, 0)
    }
}

impl<'input> NotContextAttrs<'input> for NotContext<'input> {}

pub struct NotContextExt<'input> {
    base: UnarynotaddsubContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {NotContextExt<'a>}

impl<'input> PainlessParserContext<'input> for NotContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for NotContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_not(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_not(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for NotContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_not(self);
    }
}

impl<'input> CustomRuleContext<'input> for NotContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unarynotaddsub
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unarynotaddsub }
}

impl<'input> Borrow<UnarynotaddsubContextExt<'input>> for NotContext<'input> {
    fn borrow(&self) -> &UnarynotaddsubContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<UnarynotaddsubContextExt<'input>> for NotContext<'input> {
    fn borrow_mut(&mut self) -> &mut UnarynotaddsubContextExt<'input> {
        &mut self.base
    }
}

impl<'input> UnarynotaddsubContextAttrs<'input> for NotContext<'input> {}

impl<'input> NotContextExt<'input> {
    fn new(ctx: &dyn UnarynotaddsubContextAttrs<'input>) -> Rc<UnarynotaddsubContextAll<'input>> {
        Rc::new(UnarynotaddsubContextAll::NotContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NotContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ReadContext<'input> = BaseParserRuleContext<'input, ReadContextExt<'input>>;

pub trait ReadContextAttrs<'input>: PainlessParserContext<'input> {
    fn chain(&self) -> Option<Rc<ChainContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ReadContextAttrs<'input> for ReadContext<'input> {}

pub struct ReadContextExt<'input> {
    base: UnarynotaddsubContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ReadContextExt<'a>}

impl<'input> PainlessParserContext<'input> for ReadContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ReadContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_read(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_read(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ReadContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_read(self);
    }
}

impl<'input> CustomRuleContext<'input> for ReadContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unarynotaddsub
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unarynotaddsub }
}

impl<'input> Borrow<UnarynotaddsubContextExt<'input>> for ReadContext<'input> {
    fn borrow(&self) -> &UnarynotaddsubContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<UnarynotaddsubContextExt<'input>> for ReadContext<'input> {
    fn borrow_mut(&mut self) -> &mut UnarynotaddsubContextExt<'input> {
        &mut self.base
    }
}

impl<'input> UnarynotaddsubContextAttrs<'input> for ReadContext<'input> {}

impl<'input> ReadContextExt<'input> {
    fn new(ctx: &dyn UnarynotaddsubContextAttrs<'input>) -> Rc<UnarynotaddsubContextAll<'input>> {
        Rc::new(UnarynotaddsubContextAll::ReadContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ReadContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type PostContext<'input> = BaseParserRuleContext<'input, PostContextExt<'input>>;

pub trait PostContextAttrs<'input>: PainlessParserContext<'input> {
    fn chain(&self) -> Option<Rc<ChainContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INCR
    /// Returns `None` if there is no child corresponding to token INCR
    fn INCR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INCR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DECR
    /// Returns `None` if there is no child corresponding to token DECR
    fn DECR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DECR, 0)
    }
}

impl<'input> PostContextAttrs<'input> for PostContext<'input> {}

pub struct PostContextExt<'input> {
    base: UnarynotaddsubContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {PostContextExt<'a>}

impl<'input> PainlessParserContext<'input> for PostContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for PostContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_post(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_post(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for PostContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_post(self);
    }
}

impl<'input> CustomRuleContext<'input> for PostContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unarynotaddsub
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unarynotaddsub }
}

impl<'input> Borrow<UnarynotaddsubContextExt<'input>> for PostContext<'input> {
    fn borrow(&self) -> &UnarynotaddsubContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<UnarynotaddsubContextExt<'input>> for PostContext<'input> {
    fn borrow_mut(&mut self) -> &mut UnarynotaddsubContextExt<'input> {
        &mut self.base
    }
}

impl<'input> UnarynotaddsubContextAttrs<'input> for PostContext<'input> {}

impl<'input> PostContextExt<'input> {
    fn new(ctx: &dyn UnarynotaddsubContextAttrs<'input>) -> Rc<UnarynotaddsubContextAll<'input>> {
        Rc::new(UnarynotaddsubContextAll::PostContext(
            BaseParserRuleContext::copy_from(
                ctx,
                PostContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn unarynotaddsub(&mut self) -> Result<Rc<UnarynotaddsubContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            UnarynotaddsubContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 38, RULE_unarynotaddsub);
        let mut _localctx: Rc<UnarynotaddsubContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(338);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(28, &mut recog.base)? {
                1 => {
                    let tmp = ReadContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        /*InvokeRule chain*/
                        recog.base.set_state(331);
                        recog.chain()?;
                    }
                }
                2 => {
                    let tmp = PostContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        /*InvokeRule chain*/
                        recog.base.set_state(332);
                        recog.chain()?;

                        recog.base.set_state(333);
                        _la = recog.base.input.la(1);
                        if { !(_la == INCR || _la == DECR) } {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }
                3 => {
                    let tmp = NotContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(335);
                        _la = recog.base.input.la(1);
                        if { !(_la == BOOLNOT || _la == BWNOT) } {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                        /*InvokeRule unary*/
                        recog.base.set_state(336);
                        recog.unary()?;
                    }
                }
                4 => {
                    let tmp = CastContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        /*InvokeRule castexpression*/
                        recog.base.set_state(337);
                        recog.castexpression()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- castexpression ----------------
#[derive(Debug)]
pub enum CastexpressionContextAll<'input> {
    RefcastContext(RefcastContext<'input>),
    PrimordefcastContext(PrimordefcastContext<'input>),
    Error(CastexpressionContext<'input>),
}
antlr_rust::tid! {CastexpressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for CastexpressionContextAll<'input> {}

impl<'input> PainlessParserContext<'input> for CastexpressionContextAll<'input> {}

impl<'input> Deref for CastexpressionContextAll<'input> {
    type Target = dyn CastexpressionContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use CastexpressionContextAll::*;
        match self {
            RefcastContext(inner) => inner,
            PrimordefcastContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for CastexpressionContextAll<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for CastexpressionContextAll<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type CastexpressionContext<'input> =
    BaseParserRuleContext<'input, CastexpressionContextExt<'input>>;

#[derive(Clone)]
pub struct CastexpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for CastexpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for CastexpressionContext<'input>
{
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for CastexpressionContext<'input>
{
}

impl<'input> CustomRuleContext<'input> for CastexpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_castexpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_castexpression }
}
antlr_rust::tid! {CastexpressionContextExt<'a>}

impl<'input> CastexpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<CastexpressionContextAll<'input>> {
        Rc::new(CastexpressionContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                CastexpressionContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait CastexpressionContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<CastexpressionContextExt<'input>>
{
}

impl<'input> CastexpressionContextAttrs<'input> for CastexpressionContext<'input> {}

pub type RefcastContext<'input> = BaseParserRuleContext<'input, RefcastContextExt<'input>>;

pub trait RefcastContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    fn refcasttype(&self) -> Option<Rc<RefcasttypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
    fn unarynotaddsub(&self) -> Option<Rc<UnarynotaddsubContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> RefcastContextAttrs<'input> for RefcastContext<'input> {}

pub struct RefcastContextExt<'input> {
    base: CastexpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {RefcastContextExt<'a>}

impl<'input> PainlessParserContext<'input> for RefcastContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for RefcastContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_refcast(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_refcast(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for RefcastContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_refcast(self);
    }
}

impl<'input> CustomRuleContext<'input> for RefcastContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_castexpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_castexpression }
}

impl<'input> Borrow<CastexpressionContextExt<'input>> for RefcastContext<'input> {
    fn borrow(&self) -> &CastexpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<CastexpressionContextExt<'input>> for RefcastContext<'input> {
    fn borrow_mut(&mut self) -> &mut CastexpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> CastexpressionContextAttrs<'input> for RefcastContext<'input> {}

impl<'input> RefcastContextExt<'input> {
    fn new(ctx: &dyn CastexpressionContextAttrs<'input>) -> Rc<CastexpressionContextAll<'input>> {
        Rc::new(CastexpressionContextAll::RefcastContext(
            BaseParserRuleContext::copy_from(
                ctx,
                RefcastContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type PrimordefcastContext<'input> =
    BaseParserRuleContext<'input, PrimordefcastContextExt<'input>>;

pub trait PrimordefcastContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    fn primordefcasttype(&self) -> Option<Rc<PrimordefcasttypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
    fn unary(&self) -> Option<Rc<UnaryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> PrimordefcastContextAttrs<'input> for PrimordefcastContext<'input> {}

pub struct PrimordefcastContextExt<'input> {
    base: CastexpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {PrimordefcastContextExt<'a>}

impl<'input> PainlessParserContext<'input> for PrimordefcastContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for PrimordefcastContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_primordefcast(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_primordefcast(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for PrimordefcastContext<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_primordefcast(self);
    }
}

impl<'input> CustomRuleContext<'input> for PrimordefcastContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_castexpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_castexpression }
}

impl<'input> Borrow<CastexpressionContextExt<'input>> for PrimordefcastContext<'input> {
    fn borrow(&self) -> &CastexpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<CastexpressionContextExt<'input>> for PrimordefcastContext<'input> {
    fn borrow_mut(&mut self) -> &mut CastexpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> CastexpressionContextAttrs<'input> for PrimordefcastContext<'input> {}

impl<'input> PrimordefcastContextExt<'input> {
    fn new(ctx: &dyn CastexpressionContextAttrs<'input>) -> Rc<CastexpressionContextAll<'input>> {
        Rc::new(CastexpressionContextAll::PrimordefcastContext(
            BaseParserRuleContext::copy_from(
                ctx,
                PrimordefcastContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn castexpression(&mut self) -> Result<Rc<CastexpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            CastexpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 40, RULE_castexpression);
        let mut _localctx: Rc<CastexpressionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(350);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(29, &mut recog.base)? {
                1 => {
                    let tmp = PrimordefcastContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(340);
                        recog.base.match_token(LP, &mut recog.err_handler)?;

                        /*InvokeRule primordefcasttype*/
                        recog.base.set_state(341);
                        recog.primordefcasttype()?;

                        recog.base.set_state(342);
                        recog.base.match_token(RP, &mut recog.err_handler)?;

                        /*InvokeRule unary*/
                        recog.base.set_state(343);
                        recog.unary()?;
                    }
                }
                2 => {
                    let tmp = RefcastContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(345);
                        recog.base.match_token(LP, &mut recog.err_handler)?;

                        /*InvokeRule refcasttype*/
                        recog.base.set_state(346);
                        recog.refcasttype()?;

                        recog.base.set_state(347);
                        recog.base.match_token(RP, &mut recog.err_handler)?;

                        /*InvokeRule unarynotaddsub*/
                        recog.base.set_state(348);
                        recog.unarynotaddsub()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- primordefcasttype ----------------
pub type PrimordefcasttypeContextAll<'input> = PrimordefcasttypeContext<'input>;

pub type PrimordefcasttypeContext<'input> =
    BaseParserRuleContext<'input, PrimordefcasttypeContextExt<'input>>;

#[derive(Clone)]
pub struct PrimordefcasttypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for PrimordefcasttypeContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for PrimordefcasttypeContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_primordefcasttype(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_primordefcasttype(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for PrimordefcasttypeContext<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_primordefcasttype(self);
    }
}

impl<'input> CustomRuleContext<'input> for PrimordefcasttypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primordefcasttype
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primordefcasttype }
}
antlr_rust::tid! {PrimordefcasttypeContextExt<'a>}

impl<'input> PrimordefcasttypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PrimordefcasttypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PrimordefcasttypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait PrimordefcasttypeContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<PrimordefcasttypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DEF
    /// Returns `None` if there is no child corresponding to token DEF
    fn DEF(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DEF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PRIMITIVE
    /// Returns `None` if there is no child corresponding to token PRIMITIVE
    fn PRIMITIVE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PRIMITIVE, 0)
    }
}

impl<'input> PrimordefcasttypeContextAttrs<'input> for PrimordefcasttypeContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn primordefcasttype(
        &mut self,
    ) -> Result<Rc<PrimordefcasttypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            PrimordefcasttypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 42, RULE_primordefcasttype);
        let mut _localctx: Rc<PrimordefcasttypeContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(352);
                _la = recog.base.input.la(1);
                if { !(_la == PRIMITIVE || _la == DEF) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- refcasttype ----------------
pub type RefcasttypeContextAll<'input> = RefcasttypeContext<'input>;

pub type RefcasttypeContext<'input> = BaseParserRuleContext<'input, RefcasttypeContextExt<'input>>;

#[derive(Clone)]
pub struct RefcasttypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for RefcasttypeContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for RefcasttypeContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_refcasttype(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_refcasttype(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for RefcasttypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_refcasttype(self);
    }
}

impl<'input> CustomRuleContext<'input> for RefcasttypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_refcasttype
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_refcasttype }
}
antlr_rust::tid! {RefcasttypeContextExt<'a>}

impl<'input> RefcasttypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<RefcasttypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            RefcasttypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait RefcasttypeContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<RefcasttypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DEF
    /// Returns `None` if there is no child corresponding to token DEF
    fn DEF(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DEF, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token LBRACE in current rule
    fn LBRACE_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token LBRACE, starting from 0.
    /// Returns `None` if number of children corresponding to token LBRACE is less or equal than `i`.
    fn LBRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token RBRACE in current rule
    fn RBRACE_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token RBRACE, starting from 0.
    /// Returns `None` if number of children corresponding to token RBRACE is less or equal than `i`.
    fn RBRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, i)
    }
    /// Retrieves first TerminalNode corresponding to token PRIMITIVE
    /// Returns `None` if there is no child corresponding to token PRIMITIVE
    fn PRIMITIVE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PRIMITIVE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOTID in current rule
    fn DOTID_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOTID, starting from 0.
    /// Returns `None` if number of children corresponding to token DOTID is less or equal than `i`.
    fn DOTID(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOTID, i)
    }
}

impl<'input> RefcasttypeContextAttrs<'input> for RefcasttypeContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn refcasttype(&mut self) -> Result<Rc<RefcasttypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = RefcasttypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 44, RULE_refcasttype);
        let mut _localctx: Rc<RefcasttypeContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(383);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                DEF => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(354);
                        recog.base.match_token(DEF, &mut recog.err_handler)?;

                        recog.base.set_state(357);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    recog.base.set_state(355);
                                    recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                                    recog.base.set_state(356);
                                    recog.base.match_token(RBRACE, &mut recog.err_handler)?;
                                }
                            }
                            recog.base.set_state(359);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == LBRACE) {
                                break;
                            }
                        }
                    }
                }

                PRIMITIVE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(361);
                        recog.base.match_token(PRIMITIVE, &mut recog.err_handler)?;

                        recog.base.set_state(364);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    recog.base.set_state(362);
                                    recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                                    recog.base.set_state(363);
                                    recog.base.match_token(RBRACE, &mut recog.err_handler)?;
                                }
                            }
                            recog.base.set_state(366);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !(_la == LBRACE) {
                                break;
                            }
                        }
                    }
                }

                ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(368);
                        recog.base.match_token(ID, &mut recog.err_handler)?;

                        recog.base.set_state(373);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == DOT {
                            {
                                {
                                    recog.base.set_state(369);
                                    recog.base.match_token(DOT, &mut recog.err_handler)?;

                                    recog.base.set_state(370);
                                    recog.base.match_token(DOTID, &mut recog.err_handler)?;
                                }
                            }
                            recog.base.set_state(375);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(380);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == LBRACE {
                            {
                                {
                                    recog.base.set_state(376);
                                    recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                                    recog.base.set_state(377);
                                    recog.base.match_token(RBRACE, &mut recog.err_handler)?;
                                }
                            }
                            recog.base.set_state(382);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- chain ----------------
#[derive(Debug)]
pub enum ChainContextAll<'input> {
    DynamicContext(DynamicContext<'input>),
    NewarrayContext(NewarrayContext<'input>),
    Error(ChainContext<'input>),
}
antlr_rust::tid! {ChainContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ChainContextAll<'input> {}

impl<'input> PainlessParserContext<'input> for ChainContextAll<'input> {}

impl<'input> Deref for ChainContextAll<'input> {
    type Target = dyn ChainContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use ChainContextAll::*;
        match self {
            DynamicContext(inner) => inner,
            NewarrayContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ChainContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ChainContextAll<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type ChainContext<'input> = BaseParserRuleContext<'input, ChainContextExt<'input>>;

#[derive(Clone)]
pub struct ChainContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for ChainContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ChainContext<'input> {}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ChainContext<'input> {}

impl<'input> CustomRuleContext<'input> for ChainContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_chain
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_chain }
}
antlr_rust::tid! {ChainContextExt<'a>}

impl<'input> ChainContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ChainContextAll<'input>> {
        Rc::new(ChainContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                ChainContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait ChainContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<ChainContextExt<'input>>
{
}

impl<'input> ChainContextAttrs<'input> for ChainContext<'input> {}

pub type DynamicContext<'input> = BaseParserRuleContext<'input, DynamicContextExt<'input>>;

pub trait DynamicContextAttrs<'input>: PainlessParserContext<'input> {
    fn primary(&self) -> Option<Rc<PrimaryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn postfix_all(&self) -> Vec<Rc<PostfixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn postfix(&self, i: usize) -> Option<Rc<PostfixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> DynamicContextAttrs<'input> for DynamicContext<'input> {}

pub struct DynamicContextExt<'input> {
    base: ChainContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {DynamicContextExt<'a>}

impl<'input> PainlessParserContext<'input> for DynamicContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for DynamicContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_dynamic(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_dynamic(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for DynamicContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_dynamic(self);
    }
}

impl<'input> CustomRuleContext<'input> for DynamicContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_chain
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_chain }
}

impl<'input> Borrow<ChainContextExt<'input>> for DynamicContext<'input> {
    fn borrow(&self) -> &ChainContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ChainContextExt<'input>> for DynamicContext<'input> {
    fn borrow_mut(&mut self) -> &mut ChainContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ChainContextAttrs<'input> for DynamicContext<'input> {}

impl<'input> DynamicContextExt<'input> {
    fn new(ctx: &dyn ChainContextAttrs<'input>) -> Rc<ChainContextAll<'input>> {
        Rc::new(ChainContextAll::DynamicContext(
            BaseParserRuleContext::copy_from(
                ctx,
                DynamicContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type NewarrayContext<'input> = BaseParserRuleContext<'input, NewarrayContextExt<'input>>;

pub trait NewarrayContextAttrs<'input>: PainlessParserContext<'input> {
    fn arrayinitializer(&self) -> Option<Rc<ArrayinitializerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> NewarrayContextAttrs<'input> for NewarrayContext<'input> {}

pub struct NewarrayContextExt<'input> {
    base: ChainContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {NewarrayContextExt<'a>}

impl<'input> PainlessParserContext<'input> for NewarrayContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for NewarrayContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_newarray(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_newarray(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for NewarrayContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_newarray(self);
    }
}

impl<'input> CustomRuleContext<'input> for NewarrayContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_chain
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_chain }
}

impl<'input> Borrow<ChainContextExt<'input>> for NewarrayContext<'input> {
    fn borrow(&self) -> &ChainContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ChainContextExt<'input>> for NewarrayContext<'input> {
    fn borrow_mut(&mut self) -> &mut ChainContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ChainContextAttrs<'input> for NewarrayContext<'input> {}

impl<'input> NewarrayContextExt<'input> {
    fn new(ctx: &dyn ChainContextAttrs<'input>) -> Rc<ChainContextAll<'input>> {
        Rc::new(ChainContextAll::NewarrayContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NewarrayContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn chain(&mut self) -> Result<Rc<ChainContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ChainContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_chain);
        let mut _localctx: Rc<ChainContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            recog.base.set_state(393);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(36, &mut recog.base)? {
                1 => {
                    let tmp = DynamicContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        /*InvokeRule primary*/
                        recog.base.set_state(385);
                        recog.primary()?;

                        recog.base.set_state(389);
                        recog.err_handler.sync(&mut recog.base)?;
                        _alt = recog.interpreter.adaptive_predict(35, &mut recog.base)?;
                        while { _alt != 2 && _alt != INVALID_ALT } {
                            if _alt == 1 {
                                {
                                    {
                                        /*InvokeRule postfix*/
                                        recog.base.set_state(386);
                                        recog.postfix()?;
                                    }
                                }
                            }
                            recog.base.set_state(391);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(35, &mut recog.base)?;
                        }
                    }
                }
                2 => {
                    let tmp = NewarrayContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        /*InvokeRule arrayinitializer*/
                        recog.base.set_state(392);
                        recog.arrayinitializer()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- primary ----------------
#[derive(Debug)]
pub enum PrimaryContextAll<'input> {
    ListinitContext(ListinitContext<'input>),
    RegexContext(RegexContext<'input>),
    NullContext(NullContext<'input>),
    StringContext(StringContext<'input>),
    MapinitContext(MapinitContext<'input>),
    CalllocalContext(CalllocalContext<'input>),
    TrueContext(TrueContext<'input>),
    FalseContext(FalseContext<'input>),
    VariableContext(VariableContext<'input>),
    NumericContext(NumericContext<'input>),
    NewobjectContext(NewobjectContext<'input>),
    PrecedenceContext(PrecedenceContext<'input>),
    Error(PrimaryContext<'input>),
}
antlr_rust::tid! {PrimaryContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for PrimaryContextAll<'input> {}

impl<'input> PainlessParserContext<'input> for PrimaryContextAll<'input> {}

impl<'input> Deref for PrimaryContextAll<'input> {
    type Target = dyn PrimaryContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use PrimaryContextAll::*;
        match self {
            ListinitContext(inner) => inner,
            RegexContext(inner) => inner,
            NullContext(inner) => inner,
            StringContext(inner) => inner,
            MapinitContext(inner) => inner,
            CalllocalContext(inner) => inner,
            TrueContext(inner) => inner,
            FalseContext(inner) => inner,
            VariableContext(inner) => inner,
            NumericContext(inner) => inner,
            NewobjectContext(inner) => inner,
            PrecedenceContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for PrimaryContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for PrimaryContextAll<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type PrimaryContext<'input> = BaseParserRuleContext<'input, PrimaryContextExt<'input>>;

#[derive(Clone)]
pub struct PrimaryContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for PrimaryContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for PrimaryContext<'input> {}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for PrimaryContext<'input> {}

impl<'input> CustomRuleContext<'input> for PrimaryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}
antlr_rust::tid! {PrimaryContextExt<'a>}

impl<'input> PrimaryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                PrimaryContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait PrimaryContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<PrimaryContextExt<'input>>
{
}

impl<'input> PrimaryContextAttrs<'input> for PrimaryContext<'input> {}

pub type ListinitContext<'input> = BaseParserRuleContext<'input, ListinitContextExt<'input>>;

pub trait ListinitContextAttrs<'input>: PainlessParserContext<'input> {
    fn listinitializer(&self) -> Option<Rc<ListinitializerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ListinitContextAttrs<'input> for ListinitContext<'input> {}

pub struct ListinitContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ListinitContextExt<'a>}

impl<'input> PainlessParserContext<'input> for ListinitContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ListinitContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_listinit(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_listinit(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ListinitContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_listinit(self);
    }
}

impl<'input> CustomRuleContext<'input> for ListinitContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for ListinitContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for ListinitContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for ListinitContext<'input> {}

impl<'input> ListinitContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::ListinitContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ListinitContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type RegexContext<'input> = BaseParserRuleContext<'input, RegexContextExt<'input>>;

pub trait RegexContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token REGEX
    /// Returns `None` if there is no child corresponding to token REGEX
    fn REGEX(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REGEX, 0)
    }
}

impl<'input> RegexContextAttrs<'input> for RegexContext<'input> {}

pub struct RegexContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {RegexContextExt<'a>}

impl<'input> PainlessParserContext<'input> for RegexContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for RegexContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_regex(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_regex(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for RegexContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_regex(self);
    }
}

impl<'input> CustomRuleContext<'input> for RegexContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for RegexContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for RegexContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for RegexContext<'input> {}

impl<'input> RegexContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::RegexContext(
            BaseParserRuleContext::copy_from(
                ctx,
                RegexContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type NullContext<'input> = BaseParserRuleContext<'input, NullContextExt<'input>>;

pub trait NullContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token NULL
    /// Returns `None` if there is no child corresponding to token NULL
    fn NULL(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NULL, 0)
    }
}

impl<'input> NullContextAttrs<'input> for NullContext<'input> {}

pub struct NullContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {NullContextExt<'a>}

impl<'input> PainlessParserContext<'input> for NullContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for NullContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_null(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_null(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for NullContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_null(self);
    }
}

impl<'input> CustomRuleContext<'input> for NullContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for NullContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for NullContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for NullContext<'input> {}

impl<'input> NullContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::NullContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NullContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type StringContext<'input> = BaseParserRuleContext<'input, StringContextExt<'input>>;

pub trait StringContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
}

impl<'input> StringContextAttrs<'input> for StringContext<'input> {}

pub struct StringContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {StringContextExt<'a>}

impl<'input> PainlessParserContext<'input> for StringContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for StringContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_string(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_string(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for StringContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_string(self);
    }
}

impl<'input> CustomRuleContext<'input> for StringContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for StringContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for StringContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for StringContext<'input> {}

impl<'input> StringContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::StringContext(
            BaseParserRuleContext::copy_from(
                ctx,
                StringContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type MapinitContext<'input> = BaseParserRuleContext<'input, MapinitContextExt<'input>>;

pub trait MapinitContextAttrs<'input>: PainlessParserContext<'input> {
    fn mapinitializer(&self) -> Option<Rc<MapinitializerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> MapinitContextAttrs<'input> for MapinitContext<'input> {}

pub struct MapinitContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {MapinitContextExt<'a>}

impl<'input> PainlessParserContext<'input> for MapinitContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for MapinitContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_mapinit(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_mapinit(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for MapinitContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_mapinit(self);
    }
}

impl<'input> CustomRuleContext<'input> for MapinitContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for MapinitContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for MapinitContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for MapinitContext<'input> {}

impl<'input> MapinitContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::MapinitContext(
            BaseParserRuleContext::copy_from(
                ctx,
                MapinitContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type CalllocalContext<'input> = BaseParserRuleContext<'input, CalllocalContextExt<'input>>;

pub trait CalllocalContextAttrs<'input>: PainlessParserContext<'input> {
    fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DOLLAR
    /// Returns `None` if there is no child corresponding to token DOLLAR
    fn DOLLAR(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOLLAR, 0)
    }
}

impl<'input> CalllocalContextAttrs<'input> for CalllocalContext<'input> {}

pub struct CalllocalContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {CalllocalContextExt<'a>}

impl<'input> PainlessParserContext<'input> for CalllocalContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for CalllocalContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_calllocal(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_calllocal(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for CalllocalContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_calllocal(self);
    }
}

impl<'input> CustomRuleContext<'input> for CalllocalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for CalllocalContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for CalllocalContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for CalllocalContext<'input> {}

impl<'input> CalllocalContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::CalllocalContext(
            BaseParserRuleContext::copy_from(
                ctx,
                CalllocalContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type TrueContext<'input> = BaseParserRuleContext<'input, TrueContextExt<'input>>;

pub trait TrueContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token TRUE
    /// Returns `None` if there is no child corresponding to token TRUE
    fn TRUE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TRUE, 0)
    }
}

impl<'input> TrueContextAttrs<'input> for TrueContext<'input> {}

pub struct TrueContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {TrueContextExt<'a>}

impl<'input> PainlessParserContext<'input> for TrueContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for TrueContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_true(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_true(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for TrueContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_true(self);
    }
}

impl<'input> CustomRuleContext<'input> for TrueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for TrueContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for TrueContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for TrueContext<'input> {}

impl<'input> TrueContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::TrueContext(
            BaseParserRuleContext::copy_from(
                ctx,
                TrueContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type FalseContext<'input> = BaseParserRuleContext<'input, FalseContextExt<'input>>;

pub trait FalseContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token FALSE
    /// Returns `None` if there is no child corresponding to token FALSE
    fn FALSE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FALSE, 0)
    }
}

impl<'input> FalseContextAttrs<'input> for FalseContext<'input> {}

pub struct FalseContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {FalseContextExt<'a>}

impl<'input> PainlessParserContext<'input> for FalseContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for FalseContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_false(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_false(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for FalseContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_false(self);
    }
}

impl<'input> CustomRuleContext<'input> for FalseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for FalseContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for FalseContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for FalseContext<'input> {}

impl<'input> FalseContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::FalseContext(
            BaseParserRuleContext::copy_from(
                ctx,
                FalseContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type VariableContext<'input> = BaseParserRuleContext<'input, VariableContextExt<'input>>;

pub trait VariableContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input> {}

pub struct VariableContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {VariableContextExt<'a>}

impl<'input> PainlessParserContext<'input> for VariableContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for VariableContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_variable(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_variable(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for VariableContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_variable(self);
    }
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for VariableContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for VariableContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for VariableContext<'input> {}

impl<'input> VariableContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::VariableContext(
            BaseParserRuleContext::copy_from(
                ctx,
                VariableContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type NumericContext<'input> = BaseParserRuleContext<'input, NumericContextExt<'input>>;

pub trait NumericContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token OCTAL
    /// Returns `None` if there is no child corresponding to token OCTAL
    fn OCTAL(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OCTAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token HEX
    /// Returns `None` if there is no child corresponding to token HEX
    fn HEX(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(HEX, 0)
    }
    /// Retrieves first TerminalNode corresponding to token INTEGER
    /// Returns `None` if there is no child corresponding to token INTEGER
    fn INTEGER(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INTEGER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DECIMAL
    /// Returns `None` if there is no child corresponding to token DECIMAL
    fn DECIMAL(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DECIMAL, 0)
    }
}

impl<'input> NumericContextAttrs<'input> for NumericContext<'input> {}

pub struct NumericContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {NumericContextExt<'a>}

impl<'input> PainlessParserContext<'input> for NumericContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for NumericContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_numeric(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_numeric(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for NumericContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_numeric(self);
    }
}

impl<'input> CustomRuleContext<'input> for NumericContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for NumericContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for NumericContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for NumericContext<'input> {}

impl<'input> NumericContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::NumericContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NumericContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type NewobjectContext<'input> = BaseParserRuleContext<'input, NewobjectContextExt<'input>>;

pub trait NewobjectContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token NEW
    /// Returns `None` if there is no child corresponding to token NEW
    fn NEW(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEW, 0)
    }
    fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> NewobjectContextAttrs<'input> for NewobjectContext<'input> {}

pub struct NewobjectContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {NewobjectContextExt<'a>}

impl<'input> PainlessParserContext<'input> for NewobjectContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for NewobjectContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_newobject(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_newobject(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for NewobjectContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_newobject(self);
    }
}

impl<'input> CustomRuleContext<'input> for NewobjectContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for NewobjectContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for NewobjectContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for NewobjectContext<'input> {}

impl<'input> NewobjectContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::NewobjectContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NewobjectContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type PrecedenceContext<'input> = BaseParserRuleContext<'input, PrecedenceContextExt<'input>>;

pub trait PrecedenceContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
}

impl<'input> PrecedenceContextAttrs<'input> for PrecedenceContext<'input> {}

pub struct PrecedenceContextExt<'input> {
    base: PrimaryContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {PrecedenceContextExt<'a>}

impl<'input> PainlessParserContext<'input> for PrecedenceContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for PrecedenceContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_precedence(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_precedence(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for PrecedenceContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_precedence(self);
    }
}

impl<'input> CustomRuleContext<'input> for PrecedenceContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_primary
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}

impl<'input> Borrow<PrimaryContextExt<'input>> for PrecedenceContext<'input> {
    fn borrow(&self) -> &PrimaryContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<PrimaryContextExt<'input>> for PrecedenceContext<'input> {
    fn borrow_mut(&mut self) -> &mut PrimaryContextExt<'input> {
        &mut self.base
    }
}

impl<'input> PrimaryContextAttrs<'input> for PrecedenceContext<'input> {}

impl<'input> PrecedenceContextExt<'input> {
    fn new(ctx: &dyn PrimaryContextAttrs<'input>) -> Rc<PrimaryContextAll<'input>> {
        Rc::new(PrimaryContextAll::PrecedenceContext(
            BaseParserRuleContext::copy_from(
                ctx,
                PrecedenceContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn primary(&mut self) -> Result<Rc<PrimaryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = PrimaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_primary);
        let mut _localctx: Rc<PrimaryContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(414);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(37, &mut recog.base)? {
                1 => {
                    let tmp = PrecedenceContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(395);
                        recog.base.match_token(LP, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(396);
                        recog.expression()?;

                        recog.base.set_state(397);
                        recog.base.match_token(RP, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    let tmp = NumericContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(399);
                        _la = recog.base.input.la(1);
                        if {
                            !(((_la - 73) & !0x3f) == 0
                                && ((1usize << (_la - 73))
                                    & ((1usize << (OCTAL - 73))
                                        | (1usize << (HEX - 73))
                                        | (1usize << (INTEGER - 73))
                                        | (1usize << (DECIMAL - 73))))
                                    != 0)
                        } {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }
                3 => {
                    let tmp = TrueContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(400);
                        recog.base.match_token(TRUE, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    let tmp = FalseContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        recog.base.set_state(401);
                        recog.base.match_token(FALSE, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    let tmp = NullContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        recog.base.set_state(402);
                        recog.base.match_token(NULL, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    let tmp = StringContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 6);
                    _localctx = tmp;
                    {
                        recog.base.set_state(403);
                        recog.base.match_token(STRING, &mut recog.err_handler)?;
                    }
                }
                7 => {
                    let tmp = RegexContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 7);
                    _localctx = tmp;
                    {
                        recog.base.set_state(404);
                        recog.base.match_token(REGEX, &mut recog.err_handler)?;
                    }
                }
                8 => {
                    let tmp = ListinitContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 8);
                    _localctx = tmp;
                    {
                        /*InvokeRule listinitializer*/
                        recog.base.set_state(405);
                        recog.listinitializer()?;
                    }
                }
                9 => {
                    let tmp = MapinitContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 9);
                    _localctx = tmp;
                    {
                        /*InvokeRule mapinitializer*/
                        recog.base.set_state(406);
                        recog.mapinitializer()?;
                    }
                }
                10 => {
                    let tmp = VariableContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 10);
                    _localctx = tmp;
                    {
                        recog.base.set_state(407);
                        recog.base.match_token(ID, &mut recog.err_handler)?;
                    }
                }
                11 => {
                    let tmp = CalllocalContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 11);
                    _localctx = tmp;
                    {
                        recog.base.set_state(408);
                        _la = recog.base.input.la(1);
                        if { !(_la == DOLLAR || _la == ID) } {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                        /*InvokeRule arguments*/
                        recog.base.set_state(409);
                        recog.arguments()?;
                    }
                }
                12 => {
                    let tmp = NewobjectContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 12);
                    _localctx = tmp;
                    {
                        recog.base.set_state(410);
                        recog.base.match_token(NEW, &mut recog.err_handler)?;

                        /*InvokeRule type_*/
                        recog.base.set_state(411);
                        recog.type_()?;

                        /*InvokeRule arguments*/
                        recog.base.set_state(412);
                        recog.arguments()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- postfix ----------------
pub type PostfixContextAll<'input> = PostfixContext<'input>;

pub type PostfixContext<'input> = BaseParserRuleContext<'input, PostfixContextExt<'input>>;

#[derive(Clone)]
pub struct PostfixContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for PostfixContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for PostfixContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_postfix(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_postfix(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for PostfixContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_postfix(self);
    }
}

impl<'input> CustomRuleContext<'input> for PostfixContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_postfix
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_postfix }
}
antlr_rust::tid! {PostfixContextExt<'a>}

impl<'input> PostfixContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PostfixContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PostfixContextExt { ph: PhantomData },
        ))
    }
}

pub trait PostfixContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<PostfixContextExt<'input>>
{
    fn callinvoke(&self) -> Option<Rc<CallinvokeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn fieldaccess(&self) -> Option<Rc<FieldaccessContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn braceaccess(&self) -> Option<Rc<BraceaccessContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> PostfixContextAttrs<'input> for PostfixContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn postfix(&mut self) -> Result<Rc<PostfixContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = PostfixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_postfix);
        let mut _localctx: Rc<PostfixContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(419);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(38, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule callinvoke*/
                        recog.base.set_state(416);
                        recog.callinvoke()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule fieldaccess*/
                        recog.base.set_state(417);
                        recog.fieldaccess()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule braceaccess*/
                        recog.base.set_state(418);
                        recog.braceaccess()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- postdot ----------------
pub type PostdotContextAll<'input> = PostdotContext<'input>;

pub type PostdotContext<'input> = BaseParserRuleContext<'input, PostdotContextExt<'input>>;

#[derive(Clone)]
pub struct PostdotContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for PostdotContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for PostdotContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_postdot(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_postdot(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for PostdotContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_postdot(self);
    }
}

impl<'input> CustomRuleContext<'input> for PostdotContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_postdot
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_postdot }
}
antlr_rust::tid! {PostdotContextExt<'a>}

impl<'input> PostdotContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PostdotContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PostdotContextExt { ph: PhantomData },
        ))
    }
}

pub trait PostdotContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<PostdotContextExt<'input>>
{
    fn callinvoke(&self) -> Option<Rc<CallinvokeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn fieldaccess(&self) -> Option<Rc<FieldaccessContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> PostdotContextAttrs<'input> for PostdotContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn postdot(&mut self) -> Result<Rc<PostdotContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = PostdotContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_postdot);
        let mut _localctx: Rc<PostdotContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(423);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(39, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule callinvoke*/
                        recog.base.set_state(421);
                        recog.callinvoke()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule fieldaccess*/
                        recog.base.set_state(422);
                        recog.fieldaccess()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- callinvoke ----------------
pub type CallinvokeContextAll<'input> = CallinvokeContext<'input>;

pub type CallinvokeContext<'input> = BaseParserRuleContext<'input, CallinvokeContextExt<'input>>;

#[derive(Clone)]
pub struct CallinvokeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for CallinvokeContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for CallinvokeContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_callinvoke(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_callinvoke(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for CallinvokeContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_callinvoke(self);
    }
}

impl<'input> CustomRuleContext<'input> for CallinvokeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_callinvoke
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_callinvoke }
}
antlr_rust::tid! {CallinvokeContextExt<'a>}

impl<'input> CallinvokeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<CallinvokeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            CallinvokeContextExt { ph: PhantomData },
        ))
    }
}

pub trait CallinvokeContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<CallinvokeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DOTID
    /// Returns `None` if there is no child corresponding to token DOTID
    fn DOTID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOTID, 0)
    }
    fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NSDOT
    /// Returns `None` if there is no child corresponding to token NSDOT
    fn NSDOT(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NSDOT, 0)
    }
}

impl<'input> CallinvokeContextAttrs<'input> for CallinvokeContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn callinvoke(&mut self) -> Result<Rc<CallinvokeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = CallinvokeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 54, RULE_callinvoke);
        let mut _localctx: Rc<CallinvokeContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(425);
                _la = recog.base.input.la(1);
                if { !(_la == DOT || _la == NSDOT) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
                recog.base.set_state(426);
                recog.base.match_token(DOTID, &mut recog.err_handler)?;

                /*InvokeRule arguments*/
                recog.base.set_state(427);
                recog.arguments()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- fieldaccess ----------------
pub type FieldaccessContextAll<'input> = FieldaccessContext<'input>;

pub type FieldaccessContext<'input> = BaseParserRuleContext<'input, FieldaccessContextExt<'input>>;

#[derive(Clone)]
pub struct FieldaccessContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for FieldaccessContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for FieldaccessContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_fieldaccess(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_fieldaccess(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for FieldaccessContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_fieldaccess(self);
    }
}

impl<'input> CustomRuleContext<'input> for FieldaccessContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_fieldaccess
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_fieldaccess }
}
antlr_rust::tid! {FieldaccessContextExt<'a>}

impl<'input> FieldaccessContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FieldaccessContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FieldaccessContextExt { ph: PhantomData },
        ))
    }
}

pub trait FieldaccessContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<FieldaccessContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NSDOT
    /// Returns `None` if there is no child corresponding to token NSDOT
    fn NSDOT(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NSDOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DOTID
    /// Returns `None` if there is no child corresponding to token DOTID
    fn DOTID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOTID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DOTINTEGER
    /// Returns `None` if there is no child corresponding to token DOTINTEGER
    fn DOTINTEGER(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOTINTEGER, 0)
    }
}

impl<'input> FieldaccessContextAttrs<'input> for FieldaccessContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn fieldaccess(&mut self) -> Result<Rc<FieldaccessContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FieldaccessContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 56, RULE_fieldaccess);
        let mut _localctx: Rc<FieldaccessContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(429);
                _la = recog.base.input.la(1);
                if { !(_la == DOT || _la == NSDOT) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
                recog.base.set_state(430);
                _la = recog.base.input.la(1);
                if { !(_la == DOTINTEGER || _la == DOTID) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- braceaccess ----------------
pub type BraceaccessContextAll<'input> = BraceaccessContext<'input>;

pub type BraceaccessContext<'input> = BaseParserRuleContext<'input, BraceaccessContextExt<'input>>;

#[derive(Clone)]
pub struct BraceaccessContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for BraceaccessContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for BraceaccessContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_braceaccess(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_braceaccess(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for BraceaccessContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_braceaccess(self);
    }
}

impl<'input> CustomRuleContext<'input> for BraceaccessContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_braceaccess
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_braceaccess }
}
antlr_rust::tid! {BraceaccessContextExt<'a>}

impl<'input> BraceaccessContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<BraceaccessContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            BraceaccessContextExt { ph: PhantomData },
        ))
    }
}

pub trait BraceaccessContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<BraceaccessContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
}

impl<'input> BraceaccessContextAttrs<'input> for BraceaccessContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn braceaccess(&mut self) -> Result<Rc<BraceaccessContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BraceaccessContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 58, RULE_braceaccess);
        let mut _localctx: Rc<BraceaccessContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(432);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(433);
                recog.expression()?;

                recog.base.set_state(434);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- arrayinitializer ----------------
#[derive(Debug)]
pub enum ArrayinitializerContextAll<'input> {
    NewstandardarrayContext(NewstandardarrayContext<'input>),
    NewinitializedarrayContext(NewinitializedarrayContext<'input>),
    Error(ArrayinitializerContext<'input>),
}
antlr_rust::tid! {ArrayinitializerContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ArrayinitializerContextAll<'input> {}

impl<'input> PainlessParserContext<'input> for ArrayinitializerContextAll<'input> {}

impl<'input> Deref for ArrayinitializerContextAll<'input> {
    type Target = dyn ArrayinitializerContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use ArrayinitializerContextAll::*;
        match self {
            NewstandardarrayContext(inner) => inner,
            NewinitializedarrayContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for ArrayinitializerContextAll<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for ArrayinitializerContextAll<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type ArrayinitializerContext<'input> =
    BaseParserRuleContext<'input, ArrayinitializerContextExt<'input>>;

#[derive(Clone)]
pub struct ArrayinitializerContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for ArrayinitializerContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for ArrayinitializerContext<'input>
{
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for ArrayinitializerContext<'input>
{
}

impl<'input> CustomRuleContext<'input> for ArrayinitializerContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_arrayinitializer
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_arrayinitializer }
}
antlr_rust::tid! {ArrayinitializerContextExt<'a>}

impl<'input> ArrayinitializerContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ArrayinitializerContextAll<'input>> {
        Rc::new(ArrayinitializerContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                ArrayinitializerContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait ArrayinitializerContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<ArrayinitializerContextExt<'input>>
{
}

impl<'input> ArrayinitializerContextAttrs<'input> for ArrayinitializerContext<'input> {}

pub type NewstandardarrayContext<'input> =
    BaseParserRuleContext<'input, NewstandardarrayContextExt<'input>>;

pub trait NewstandardarrayContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token NEW
    /// Returns `None` if there is no child corresponding to token NEW
    fn NEW(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEW, 0)
    }
    fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token LBRACE in current rule
    fn LBRACE_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token LBRACE, starting from 0.
    /// Returns `None` if number of children corresponding to token LBRACE is less or equal than `i`.
    fn LBRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, i)
    }
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token RBRACE in current rule
    fn RBRACE_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token RBRACE, starting from 0.
    /// Returns `None` if number of children corresponding to token RBRACE is less or equal than `i`.
    fn RBRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, i)
    }
    fn postdot(&self) -> Option<Rc<PostdotContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn postfix_all(&self) -> Vec<Rc<PostfixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn postfix(&self, i: usize) -> Option<Rc<PostfixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> NewstandardarrayContextAttrs<'input> for NewstandardarrayContext<'input> {}

pub struct NewstandardarrayContextExt<'input> {
    base: ArrayinitializerContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {NewstandardarrayContextExt<'a>}

impl<'input> PainlessParserContext<'input> for NewstandardarrayContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for NewstandardarrayContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_newstandardarray(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_newstandardarray(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for NewstandardarrayContext<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_newstandardarray(self);
    }
}

impl<'input> CustomRuleContext<'input> for NewstandardarrayContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_arrayinitializer
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_arrayinitializer }
}

impl<'input> Borrow<ArrayinitializerContextExt<'input>> for NewstandardarrayContext<'input> {
    fn borrow(&self) -> &ArrayinitializerContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ArrayinitializerContextExt<'input>> for NewstandardarrayContext<'input> {
    fn borrow_mut(&mut self) -> &mut ArrayinitializerContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ArrayinitializerContextAttrs<'input> for NewstandardarrayContext<'input> {}

impl<'input> NewstandardarrayContextExt<'input> {
    fn new(
        ctx: &dyn ArrayinitializerContextAttrs<'input>,
    ) -> Rc<ArrayinitializerContextAll<'input>> {
        Rc::new(ArrayinitializerContextAll::NewstandardarrayContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NewstandardarrayContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type NewinitializedarrayContext<'input> =
    BaseParserRuleContext<'input, NewinitializedarrayContextExt<'input>>;

pub trait NewinitializedarrayContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token NEW
    /// Returns `None` if there is no child corresponding to token NEW
    fn NEW(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEW, 0)
    }
    fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACK
    /// Returns `None` if there is no child corresponding to token LBRACK
    fn LBRACK(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACK
    /// Returns `None` if there is no child corresponding to token RBRACK
    fn RBRACK(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACK, 0)
    }
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn postfix_all(&self) -> Vec<Rc<PostfixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn postfix(&self, i: usize) -> Option<Rc<PostfixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> NewinitializedarrayContextAttrs<'input> for NewinitializedarrayContext<'input> {}

pub struct NewinitializedarrayContextExt<'input> {
    base: ArrayinitializerContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {NewinitializedarrayContextExt<'a>}

impl<'input> PainlessParserContext<'input> for NewinitializedarrayContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for NewinitializedarrayContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_newinitializedarray(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_newinitializedarray(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for NewinitializedarrayContext<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_newinitializedarray(self);
    }
}

impl<'input> CustomRuleContext<'input> for NewinitializedarrayContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_arrayinitializer
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_arrayinitializer }
}

impl<'input> Borrow<ArrayinitializerContextExt<'input>> for NewinitializedarrayContext<'input> {
    fn borrow(&self) -> &ArrayinitializerContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ArrayinitializerContextExt<'input>> for NewinitializedarrayContext<'input> {
    fn borrow_mut(&mut self) -> &mut ArrayinitializerContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ArrayinitializerContextAttrs<'input> for NewinitializedarrayContext<'input> {}

impl<'input> NewinitializedarrayContextExt<'input> {
    fn new(
        ctx: &dyn ArrayinitializerContextAttrs<'input>,
    ) -> Rc<ArrayinitializerContextAll<'input>> {
        Rc::new(ArrayinitializerContextAll::NewinitializedarrayContext(
            BaseParserRuleContext::copy_from(
                ctx,
                NewinitializedarrayContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn arrayinitializer(
        &mut self,
    ) -> Result<Rc<ArrayinitializerContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            ArrayinitializerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 60, RULE_arrayinitializer);
        let mut _localctx: Rc<ArrayinitializerContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            recog.base.set_state(477);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(46, &mut recog.base)? {
                1 => {
                    let tmp = NewstandardarrayContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(436);
                        recog.base.match_token(NEW, &mut recog.err_handler)?;

                        /*InvokeRule type_*/
                        recog.base.set_state(437);
                        recog.type_()?;

                        recog.base.set_state(442);
                        recog.err_handler.sync(&mut recog.base)?;
                        _alt = 1;
                        loop {
                            match _alt {
                                x if x == 1 => {
                                    {
                                        recog.base.set_state(438);
                                        recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                                        /*InvokeRule expression*/
                                        recog.base.set_state(439);
                                        recog.expression()?;

                                        recog.base.set_state(440);
                                        recog.base.match_token(RBRACE, &mut recog.err_handler)?;
                                    }
                                }

                                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                    &mut recog.base,
                                )))?,
                            }
                            recog.base.set_state(444);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(40, &mut recog.base)?;
                            if _alt == 2 || _alt == INVALID_ALT {
                                break;
                            }
                        }
                        recog.base.set_state(453);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(42, &mut recog.base)? {
                            x if x == 1 => {
                                {
                                    /*InvokeRule postdot*/
                                    recog.base.set_state(446);
                                    recog.postdot()?;

                                    recog.base.set_state(450);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _alt =
                                        recog.interpreter.adaptive_predict(41, &mut recog.base)?;
                                    while { _alt != 2 && _alt != INVALID_ALT } {
                                        if _alt == 1 {
                                            {
                                                {
                                                    /*InvokeRule postfix*/
                                                    recog.base.set_state(447);
                                                    recog.postfix()?;
                                                }
                                            }
                                        }
                                        recog.base.set_state(452);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _alt = recog
                                            .interpreter
                                            .adaptive_predict(41, &mut recog.base)?;
                                    }
                                }
                            }

                            _ => {}
                        }
                    }
                }
                2 => {
                    let tmp = NewinitializedarrayContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(455);
                        recog.base.match_token(NEW, &mut recog.err_handler)?;

                        /*InvokeRule type_*/
                        recog.base.set_state(456);
                        recog.type_()?;

                        recog.base.set_state(457);
                        recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                        recog.base.set_state(458);
                        recog.base.match_token(RBRACE, &mut recog.err_handler)?;

                        recog.base.set_state(459);
                        recog.base.match_token(LBRACK, &mut recog.err_handler)?;

                        recog.base.set_state(468);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la - 5) & !0x3f) == 0
                            && ((1usize << (_la - 5))
                                & ((1usize << (LBRACE - 5))
                                    | (1usize << (LP - 5))
                                    | (1usize << (DOLLAR - 5))
                                    | (1usize << (NEW - 5))
                                    | (1usize << (BOOLNOT - 5))
                                    | (1usize << (BWNOT - 5))
                                    | (1usize << (ADD - 5))
                                    | (1usize << (SUB - 5))))
                                != 0)
                            || (((_la - 59) & !0x3f) == 0
                                && ((1usize << (_la - 59))
                                    & ((1usize << (INCR - 59))
                                        | (1usize << (DECR - 59))
                                        | (1usize << (OCTAL - 59))
                                        | (1usize << (HEX - 59))
                                        | (1usize << (INTEGER - 59))
                                        | (1usize << (DECIMAL - 59))
                                        | (1usize << (STRING - 59))
                                        | (1usize << (REGEX - 59))
                                        | (1usize << (TRUE - 59))
                                        | (1usize << (FALSE - 59))
                                        | (1usize << (NULL - 59))
                                        | (1usize << (ID - 59))))
                                    != 0)
                        {
                            {
                                /*InvokeRule expression*/
                                recog.base.set_state(460);
                                recog.expression()?;

                                recog.base.set_state(465);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                while _la == COMMA {
                                    {
                                        {
                                            recog.base.set_state(461);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            /*InvokeRule expression*/
                                            recog.base.set_state(462);
                                            recog.expression()?;
                                        }
                                    }
                                    recog.base.set_state(467);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                }
                            }
                        }

                        recog.base.set_state(470);
                        recog.base.match_token(RBRACK, &mut recog.err_handler)?;

                        recog.base.set_state(474);
                        recog.err_handler.sync(&mut recog.base)?;
                        _alt = recog.interpreter.adaptive_predict(45, &mut recog.base)?;
                        while { _alt != 2 && _alt != INVALID_ALT } {
                            if _alt == 1 {
                                {
                                    {
                                        /*InvokeRule postfix*/
                                        recog.base.set_state(471);
                                        recog.postfix()?;
                                    }
                                }
                            }
                            recog.base.set_state(476);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(45, &mut recog.base)?;
                        }
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- listinitializer ----------------
pub type ListinitializerContextAll<'input> = ListinitializerContext<'input>;

pub type ListinitializerContext<'input> =
    BaseParserRuleContext<'input, ListinitializerContextExt<'input>>;

#[derive(Clone)]
pub struct ListinitializerContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for ListinitializerContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for ListinitializerContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_listinitializer(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_listinitializer(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for ListinitializerContext<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_listinitializer(self);
    }
}

impl<'input> CustomRuleContext<'input> for ListinitializerContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_listinitializer
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_listinitializer }
}
antlr_rust::tid! {ListinitializerContextExt<'a>}

impl<'input> ListinitializerContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ListinitializerContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ListinitializerContextExt { ph: PhantomData },
        ))
    }
}

pub trait ListinitializerContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<ListinitializerContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> ListinitializerContextAttrs<'input> for ListinitializerContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn listinitializer(&mut self) -> Result<Rc<ListinitializerContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            ListinitializerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 62, RULE_listinitializer);
        let mut _localctx: Rc<ListinitializerContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(492);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(48, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(479);
                        recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(480);
                        recog.expression()?;

                        recog.base.set_state(485);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(481);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule expression*/
                                    recog.base.set_state(482);
                                    recog.expression()?;
                                }
                            }
                            recog.base.set_state(487);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(488);
                        recog.base.match_token(RBRACE, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(490);
                        recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                        recog.base.set_state(491);
                        recog.base.match_token(RBRACE, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- mapinitializer ----------------
pub type MapinitializerContextAll<'input> = MapinitializerContext<'input>;

pub type MapinitializerContext<'input> =
    BaseParserRuleContext<'input, MapinitializerContextExt<'input>>;

#[derive(Clone)]
pub struct MapinitializerContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for MapinitializerContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for MapinitializerContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_mapinitializer(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_mapinitializer(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for MapinitializerContext<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_mapinitializer(self);
    }
}

impl<'input> CustomRuleContext<'input> for MapinitializerContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_mapinitializer
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_mapinitializer }
}
antlr_rust::tid! {MapinitializerContextExt<'a>}

impl<'input> MapinitializerContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<MapinitializerContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            MapinitializerContextExt { ph: PhantomData },
        ))
    }
}

pub trait MapinitializerContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<MapinitializerContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    fn maptoken_all(&self) -> Vec<Rc<MaptokenContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn maptoken(&self, i: usize) -> Option<Rc<MaptokenContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
}

impl<'input> MapinitializerContextAttrs<'input> for MapinitializerContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn mapinitializer(&mut self) -> Result<Rc<MapinitializerContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            MapinitializerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 64, RULE_mapinitializer);
        let mut _localctx: Rc<MapinitializerContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(508);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(50, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(494);
                        recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                        /*InvokeRule maptoken*/
                        recog.base.set_state(495);
                        recog.maptoken()?;

                        recog.base.set_state(500);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(496);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule maptoken*/
                                    recog.base.set_state(497);
                                    recog.maptoken()?;
                                }
                            }
                            recog.base.set_state(502);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(503);
                        recog.base.match_token(RBRACE, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(505);
                        recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                        recog.base.set_state(506);
                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                        recog.base.set_state(507);
                        recog.base.match_token(RBRACE, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- maptoken ----------------
pub type MaptokenContextAll<'input> = MaptokenContext<'input>;

pub type MaptokenContext<'input> = BaseParserRuleContext<'input, MaptokenContextExt<'input>>;

#[derive(Clone)]
pub struct MaptokenContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for MaptokenContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for MaptokenContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_maptoken(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_maptoken(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for MaptokenContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_maptoken(self);
    }
}

impl<'input> CustomRuleContext<'input> for MaptokenContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_maptoken
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_maptoken }
}
antlr_rust::tid! {MaptokenContextExt<'a>}

impl<'input> MaptokenContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<MaptokenContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            MaptokenContextExt { ph: PhantomData },
        ))
    }
}

pub trait MaptokenContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<MaptokenContextExt<'input>>
{
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
}

impl<'input> MaptokenContextAttrs<'input> for MaptokenContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn maptoken(&mut self) -> Result<Rc<MaptokenContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = MaptokenContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_maptoken);
        let mut _localctx: Rc<MaptokenContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule expression*/
                recog.base.set_state(510);
                recog.expression()?;

                recog.base.set_state(511);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(512);
                recog.expression()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- arguments ----------------
pub type ArgumentsContextAll<'input> = ArgumentsContext<'input>;

pub type ArgumentsContext<'input> = BaseParserRuleContext<'input, ArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for ArgumentsContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ArgumentsContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_arguments(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_arguments(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ArgumentsContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_arguments(self);
    }
}

impl<'input> CustomRuleContext<'input> for ArgumentsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_arguments
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_arguments }
}
antlr_rust::tid! {ArgumentsContextExt<'a>}

impl<'input> ArgumentsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ArgumentsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ArgumentsContextExt { ph: PhantomData },
        ))
    }
}

pub trait ArgumentsContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<ArgumentsContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
    fn argument_all(&self) -> Vec<Rc<ArgumentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn argument(&self, i: usize) -> Option<Rc<ArgumentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> ArgumentsContextAttrs<'input> for ArgumentsContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn arguments(&mut self) -> Result<Rc<ArgumentsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_arguments);
        let mut _localctx: Rc<ArgumentsContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    recog.base.set_state(514);
                    recog.base.match_token(LP, &mut recog.err_handler)?;

                    recog.base.set_state(523);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if (((_la - 5) & !0x3f) == 0
                        && ((1usize << (_la - 5))
                            & ((1usize << (LBRACE - 5))
                                | (1usize << (LP - 5))
                                | (1usize << (DOLLAR - 5))
                                | (1usize << (NEW - 5))
                                | (1usize << (THIS - 5))
                                | (1usize << (BOOLNOT - 5))
                                | (1usize << (BWNOT - 5))
                                | (1usize << (ADD - 5))
                                | (1usize << (SUB - 5))))
                            != 0)
                        || (((_la - 59) & !0x3f) == 0
                            && ((1usize << (_la - 59))
                                & ((1usize << (INCR - 59))
                                    | (1usize << (DECR - 59))
                                    | (1usize << (OCTAL - 59))
                                    | (1usize << (HEX - 59))
                                    | (1usize << (INTEGER - 59))
                                    | (1usize << (DECIMAL - 59))
                                    | (1usize << (STRING - 59))
                                    | (1usize << (REGEX - 59))
                                    | (1usize << (TRUE - 59))
                                    | (1usize << (FALSE - 59))
                                    | (1usize << (NULL - 59))
                                    | (1usize << (PRIMITIVE - 59))
                                    | (1usize << (DEF - 59))
                                    | (1usize << (ID - 59))))
                                != 0)
                    {
                        {
                            /*InvokeRule argument*/
                            recog.base.set_state(515);
                            recog.argument()?;

                            recog.base.set_state(520);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while _la == COMMA {
                                {
                                    {
                                        recog.base.set_state(516);
                                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                        /*InvokeRule argument*/
                                        recog.base.set_state(517);
                                        recog.argument()?;
                                    }
                                }
                                recog.base.set_state(522);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                        }
                    }

                    recog.base.set_state(525);
                    recog.base.match_token(RP, &mut recog.err_handler)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- argument ----------------
pub type ArgumentContextAll<'input> = ArgumentContext<'input>;

pub type ArgumentContext<'input> = BaseParserRuleContext<'input, ArgumentContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for ArgumentContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for ArgumentContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_argument(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_argument(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ArgumentContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_argument(self);
    }
}

impl<'input> CustomRuleContext<'input> for ArgumentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_argument
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_argument }
}
antlr_rust::tid! {ArgumentContextExt<'a>}

impl<'input> ArgumentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ArgumentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ArgumentContextExt { ph: PhantomData },
        ))
    }
}

pub trait ArgumentContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<ArgumentContextExt<'input>>
{
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn lambda(&self) -> Option<Rc<LambdaContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn funcref(&self) -> Option<Rc<FuncrefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ArgumentContextAttrs<'input> for ArgumentContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn argument(&mut self) -> Result<Rc<ArgumentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ArgumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_argument);
        let mut _localctx: Rc<ArgumentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(530);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(53, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule expression*/
                        recog.base.set_state(527);
                        recog.expression()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule lambda*/
                        recog.base.set_state(528);
                        recog.lambda()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule funcref*/
                        recog.base.set_state(529);
                        recog.funcref()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- lambda ----------------
pub type LambdaContextAll<'input> = LambdaContext<'input>;

pub type LambdaContext<'input> = BaseParserRuleContext<'input, LambdaContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for LambdaContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for LambdaContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_lambda(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_lambda(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for LambdaContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_lambda(self);
    }
}

impl<'input> CustomRuleContext<'input> for LambdaContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_lambda
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_lambda }
}
antlr_rust::tid! {LambdaContextExt<'a>}

impl<'input> LambdaContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<LambdaContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            LambdaContextExt { ph: PhantomData },
        ))
    }
}

pub trait LambdaContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<LambdaContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ARROW
    /// Returns `None` if there is no child corresponding to token ARROW
    fn ARROW(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ARROW, 0)
    }
    fn lamtype_all(&self) -> Vec<Rc<LamtypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn lamtype(&self, i: usize) -> Option<Rc<LamtypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token LP
    /// Returns `None` if there is no child corresponding to token LP
    fn LP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RP
    /// Returns `None` if there is no child corresponding to token RP
    fn RP(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RP, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> LambdaContextAttrs<'input> for LambdaContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn lambda(&mut self) -> Result<Rc<LambdaContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = LambdaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_lambda);
        let mut _localctx: Rc<LambdaContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(545);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    PRIMITIVE | DEF | ID => {
                        {
                            /*InvokeRule lamtype*/
                            recog.base.set_state(532);
                            recog.lamtype()?;
                        }
                    }

                    LP => {
                        {
                            recog.base.set_state(533);
                            recog.base.match_token(LP, &mut recog.err_handler)?;

                            recog.base.set_state(542);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if (((_la - 82) & !0x3f) == 0
                                && ((1usize << (_la - 82))
                                    & ((1usize << (PRIMITIVE - 82))
                                        | (1usize << (DEF - 82))
                                        | (1usize << (ID - 82))))
                                    != 0)
                            {
                                {
                                    /*InvokeRule lamtype*/
                                    recog.base.set_state(534);
                                    recog.lamtype()?;

                                    recog.base.set_state(539);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    while _la == COMMA {
                                        {
                                            {
                                                recog.base.set_state(535);
                                                recog
                                                    .base
                                                    .match_token(COMMA, &mut recog.err_handler)?;

                                                /*InvokeRule lamtype*/
                                                recog.base.set_state(536);
                                                recog.lamtype()?;
                                            }
                                        }
                                        recog.base.set_state(541);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                    }
                                }
                            }

                            recog.base.set_state(544);
                            recog.base.match_token(RP, &mut recog.err_handler)?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
                recog.base.set_state(547);
                recog.base.match_token(ARROW, &mut recog.err_handler)?;

                recog.base.set_state(550);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    LBRACK => {
                        {
                            /*InvokeRule block*/
                            recog.base.set_state(548);
                            recog.block()?;
                        }
                    }

                    LBRACE | LP | DOLLAR | NEW | BOOLNOT | BWNOT | ADD | SUB | INCR | DECR
                    | OCTAL | HEX | INTEGER | DECIMAL | STRING | REGEX | TRUE | FALSE | NULL
                    | ID => {
                        {
                            /*InvokeRule expression*/
                            recog.base.set_state(549);
                            recog.expression()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- lamtype ----------------
pub type LamtypeContextAll<'input> = LamtypeContext<'input>;

pub type LamtypeContext<'input> = BaseParserRuleContext<'input, LamtypeContextExt<'input>>;

#[derive(Clone)]
pub struct LamtypeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for LamtypeContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for LamtypeContext<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_lamtype(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_lamtype(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for LamtypeContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_lamtype(self);
    }
}

impl<'input> CustomRuleContext<'input> for LamtypeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_lamtype
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_lamtype }
}
antlr_rust::tid! {LamtypeContextExt<'a>}

impl<'input> LamtypeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<LamtypeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            LamtypeContextExt { ph: PhantomData },
        ))
    }
}

pub trait LamtypeContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<LamtypeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    fn decltype(&self) -> Option<Rc<DecltypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> LamtypeContextAttrs<'input> for LamtypeContext<'input> {}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn lamtype(&mut self) -> Result<Rc<LamtypeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = LamtypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_lamtype);
        let mut _localctx: Rc<LamtypeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(553);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(58, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule decltype*/
                            recog.base.set_state(552);
                            recog.decltype()?;
                        }
                    }

                    _ => {}
                }
                recog.base.set_state(555);
                recog.base.match_token(ID, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- funcref ----------------
#[derive(Debug)]
pub enum FuncrefContextAll<'input> {
    ClassfuncrefContext(ClassfuncrefContext<'input>),
    ConstructorfuncrefContext(ConstructorfuncrefContext<'input>),
    LocalfuncrefContext(LocalfuncrefContext<'input>),
    Error(FuncrefContext<'input>),
}
antlr_rust::tid! {FuncrefContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for FuncrefContextAll<'input> {}

impl<'input> PainlessParserContext<'input> for FuncrefContextAll<'input> {}

impl<'input> Deref for FuncrefContextAll<'input> {
    type Target = dyn FuncrefContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use FuncrefContextAll::*;
        match self {
            ClassfuncrefContext(inner) => inner,
            ConstructorfuncrefContext(inner) => inner,
            LocalfuncrefContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for FuncrefContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for FuncrefContextAll<'input> {
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type FuncrefContext<'input> = BaseParserRuleContext<'input, FuncrefContextExt<'input>>;

#[derive(Clone)]
pub struct FuncrefContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> PainlessParserContext<'input> for FuncrefContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a> for FuncrefContext<'input> {}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for FuncrefContext<'input> {}

impl<'input> CustomRuleContext<'input> for FuncrefContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_funcref
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_funcref }
}
antlr_rust::tid! {FuncrefContextExt<'a>}

impl<'input> FuncrefContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn PainlessParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FuncrefContextAll<'input>> {
        Rc::new(FuncrefContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                FuncrefContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait FuncrefContextAttrs<'input>:
    PainlessParserContext<'input> + BorrowMut<FuncrefContextExt<'input>>
{
}

impl<'input> FuncrefContextAttrs<'input> for FuncrefContext<'input> {}

pub type ClassfuncrefContext<'input> =
    BaseParserRuleContext<'input, ClassfuncrefContextExt<'input>>;

pub trait ClassfuncrefContextAttrs<'input>: PainlessParserContext<'input> {
    fn decltype(&self) -> Option<Rc<DecltypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token REF
    /// Returns `None` if there is no child corresponding to token REF
    fn REF(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
}

impl<'input> ClassfuncrefContextAttrs<'input> for ClassfuncrefContext<'input> {}

pub struct ClassfuncrefContextExt<'input> {
    base: FuncrefContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ClassfuncrefContextExt<'a>}

impl<'input> PainlessParserContext<'input> for ClassfuncrefContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for ClassfuncrefContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_classfuncref(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_classfuncref(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for ClassfuncrefContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_classfuncref(self);
    }
}

impl<'input> CustomRuleContext<'input> for ClassfuncrefContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_funcref
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_funcref }
}

impl<'input> Borrow<FuncrefContextExt<'input>> for ClassfuncrefContext<'input> {
    fn borrow(&self) -> &FuncrefContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<FuncrefContextExt<'input>> for ClassfuncrefContext<'input> {
    fn borrow_mut(&mut self) -> &mut FuncrefContextExt<'input> {
        &mut self.base
    }
}

impl<'input> FuncrefContextAttrs<'input> for ClassfuncrefContext<'input> {}

impl<'input> ClassfuncrefContextExt<'input> {
    fn new(ctx: &dyn FuncrefContextAttrs<'input>) -> Rc<FuncrefContextAll<'input>> {
        Rc::new(FuncrefContextAll::ClassfuncrefContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ClassfuncrefContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ConstructorfuncrefContext<'input> =
    BaseParserRuleContext<'input, ConstructorfuncrefContextExt<'input>>;

pub trait ConstructorfuncrefContextAttrs<'input>: PainlessParserContext<'input> {
    fn decltype(&self) -> Option<Rc<DecltypeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token REF
    /// Returns `None` if there is no child corresponding to token REF
    fn REF(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NEW
    /// Returns `None` if there is no child corresponding to token NEW
    fn NEW(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEW, 0)
    }
}

impl<'input> ConstructorfuncrefContextAttrs<'input> for ConstructorfuncrefContext<'input> {}

pub struct ConstructorfuncrefContextExt<'input> {
    base: FuncrefContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ConstructorfuncrefContextExt<'a>}

impl<'input> PainlessParserContext<'input> for ConstructorfuncrefContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for ConstructorfuncrefContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_constructorfuncref(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_constructorfuncref(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a>
    for ConstructorfuncrefContext<'input>
{
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_constructorfuncref(self);
    }
}

impl<'input> CustomRuleContext<'input> for ConstructorfuncrefContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_funcref
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_funcref }
}

impl<'input> Borrow<FuncrefContextExt<'input>> for ConstructorfuncrefContext<'input> {
    fn borrow(&self) -> &FuncrefContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<FuncrefContextExt<'input>> for ConstructorfuncrefContext<'input> {
    fn borrow_mut(&mut self) -> &mut FuncrefContextExt<'input> {
        &mut self.base
    }
}

impl<'input> FuncrefContextAttrs<'input> for ConstructorfuncrefContext<'input> {}

impl<'input> ConstructorfuncrefContextExt<'input> {
    fn new(ctx: &dyn FuncrefContextAttrs<'input>) -> Rc<FuncrefContextAll<'input>> {
        Rc::new(FuncrefContextAll::ConstructorfuncrefContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ConstructorfuncrefContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type LocalfuncrefContext<'input> =
    BaseParserRuleContext<'input, LocalfuncrefContextExt<'input>>;

pub trait LocalfuncrefContextAttrs<'input>: PainlessParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token THIS
    /// Returns `None` if there is no child corresponding to token THIS
    fn THIS(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(THIS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token REF
    /// Returns `None` if there is no child corresponding to token REF
    fn REF(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, PainlessParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
}

impl<'input> LocalfuncrefContextAttrs<'input> for LocalfuncrefContext<'input> {}

pub struct LocalfuncrefContextExt<'input> {
    base: FuncrefContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {LocalfuncrefContextExt<'a>}

impl<'input> PainlessParserContext<'input> for LocalfuncrefContext<'input> {}

impl<'input, 'a> Listenable<dyn PainlessParserListener<'input> + 'a>
    for LocalfuncrefContext<'input>
{
    fn enter(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_localfuncref(self);
    }
    fn exit(&self, listener: &mut (dyn PainlessParserListener<'input> + 'a)) {
        listener.exit_localfuncref(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn PainlessParserVisitor<'input> + 'a> for LocalfuncrefContext<'input> {
    fn accept(&self, visitor: &mut (dyn PainlessParserVisitor<'input> + 'a)) {
        visitor.visit_localfuncref(self);
    }
}

impl<'input> CustomRuleContext<'input> for LocalfuncrefContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = PainlessParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_funcref
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_funcref }
}

impl<'input> Borrow<FuncrefContextExt<'input>> for LocalfuncrefContext<'input> {
    fn borrow(&self) -> &FuncrefContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<FuncrefContextExt<'input>> for LocalfuncrefContext<'input> {
    fn borrow_mut(&mut self) -> &mut FuncrefContextExt<'input> {
        &mut self.base
    }
}

impl<'input> FuncrefContextAttrs<'input> for LocalfuncrefContext<'input> {}

impl<'input> LocalfuncrefContextExt<'input> {
    fn new(ctx: &dyn FuncrefContextAttrs<'input>) -> Rc<FuncrefContextAll<'input>> {
        Rc::new(FuncrefContextAll::LocalfuncrefContext(
            BaseParserRuleContext::copy_from(
                ctx,
                LocalfuncrefContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> PainlessParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn funcref(&mut self) -> Result<Rc<FuncrefContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FuncrefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_funcref);
        let mut _localctx: Rc<FuncrefContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(568);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(59, &mut recog.base)? {
                1 => {
                    let tmp = ClassfuncrefContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        /*InvokeRule decltype*/
                        recog.base.set_state(557);
                        recog.decltype()?;

                        recog.base.set_state(558);
                        recog.base.match_token(REF, &mut recog.err_handler)?;

                        recog.base.set_state(559);
                        recog.base.match_token(ID, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    let tmp = ConstructorfuncrefContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        /*InvokeRule decltype*/
                        recog.base.set_state(561);
                        recog.decltype()?;

                        recog.base.set_state(562);
                        recog.base.match_token(REF, &mut recog.err_handler)?;

                        recog.base.set_state(563);
                        recog.base.match_token(NEW, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    let tmp = LocalfuncrefContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(565);
                        recog.base.match_token(THIS, &mut recog.err_handler)?;

                        recog.base.set_state(566);
                        recog.base.match_token(REF, &mut recog.err_handler)?;

                        recog.base.set_state(567);
                        recog.base.match_token(ID, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x58\u{23d}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x03\x02\x07\
	\x02\x52\x0a\x02\x0c\x02\x0e\x02\x55\x0b\x02\x03\x02\x07\x02\x58\x0a\x02\
	\x0c\x02\x0e\x02\x5b\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x07\
	\x04\x6b\x0a\x04\x0c\x04\x0e\x04\x6e\x0b\x04\x05\x04\x70\x0a\x04\x03\x04\
	\x03\x04\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\x78\x0a\x05\x03\x06\x03\
	\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x05\x06\u{81}\x0a\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x05\x06\u{89}\x0a\x06\x03\x06\x03\
	\x06\x03\x06\x05\x06\u{8e}\x0a\x06\x03\x06\x03\x06\x05\x06\u{92}\x0a\x06\
	\x03\x06\x03\x06\x05\x06\u{96}\x0a\x06\x03\x06\x03\x06\x03\x06\x05\x06\u{9b}\
	\x0a\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x06\x06\u{b1}\x0a\x06\x0d\x06\x0e\x06\u{b2}\x05\
	\x06\u{b5}\x0a\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\
	\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\x07\u{c3}\x0a\x07\x03\x07\x03\
	\x07\x03\x07\x05\x07\u{c8}\x0a\x07\x03\x08\x03\x08\x05\x08\u{cc}\x0a\x08\
	\x03\x09\x03\x09\x07\x09\u{d0}\x0a\x09\x0c\x09\x0e\x09\u{d3}\x0b\x09\x03\
	\x09\x05\x09\u{d6}\x0a\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0b\x03\x0b\
	\x05\x0b\u{de}\x0a\x0b\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x07\
	\x0d\u{e6}\x0a\x0d\x0c\x0d\x0e\x0d\u{e9}\x0b\x0d\x03\x0e\x03\x0e\x03\x0e\
	\x07\x0e\u{ee}\x0a\x0e\x0c\x0e\x0e\x0e\u{f1}\x0b\x0e\x03\x0f\x03\x0f\x03\
	\x0f\x03\x0f\x03\x0f\x07\x0f\u{f8}\x0a\x0f\x0c\x0f\x0e\x0f\u{fb}\x0b\x0f\
	\x05\x0f\u{fd}\x0a\x0f\x03\x10\x03\x10\x03\x10\x05\x10\u{102}\x0a\x10\x03\
	\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x07\x12\u{135}\x0a\x12\x0c\x12\x0e\x12\u{138}\
	\x0b\x12\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\
	\x03\x13\x03\x13\x03\x13\x05\x13\u{145}\x0a\x13\x03\x14\x03\x14\x03\x14\
	\x03\x14\x03\x14\x05\x14\u{14c}\x0a\x14\x03\x15\x03\x15\x03\x15\x03\x15\
	\x03\x15\x03\x15\x03\x15\x05\x15\u{155}\x0a\x15\x03\x16\x03\x16\x03\x16\
	\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x05\x16\u{161}\
	\x0a\x16\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x06\x18\u{168}\x0a\x18\
	\x0d\x18\x0e\x18\u{169}\x03\x18\x03\x18\x03\x18\x06\x18\u{16f}\x0a\x18\x0d\
	\x18\x0e\x18\u{170}\x03\x18\x03\x18\x03\x18\x07\x18\u{176}\x0a\x18\x0c\x18\
	\x0e\x18\u{179}\x0b\x18\x03\x18\x03\x18\x07\x18\u{17d}\x0a\x18\x0c\x18\x0e\
	\x18\u{180}\x0b\x18\x05\x18\u{182}\x0a\x18\x03\x19\x03\x19\x07\x19\u{186}\
	\x0a\x19\x0c\x19\x0e\x19\u{189}\x0b\x19\x03\x19\x05\x19\u{18c}\x0a\x19\x03\
	\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\
	\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\
	\x1a\x05\x1a\u{1a1}\x0a\x1a\x03\x1b\x03\x1b\x03\x1b\x05\x1b\u{1a6}\x0a\x1b\
	\x03\x1c\x03\x1c\x05\x1c\u{1aa}\x0a\x1c\x03\x1d\x03\x1d\x03\x1d\x03\x1d\
	\x03\x1e\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x20\x03\x20\
	\x03\x20\x03\x20\x03\x20\x03\x20\x06\x20\u{1bd}\x0a\x20\x0d\x20\x0e\x20\
	\u{1be}\x03\x20\x03\x20\x07\x20\u{1c3}\x0a\x20\x0c\x20\x0e\x20\u{1c6}\x0b\
	\x20\x05\x20\u{1c8}\x0a\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\
	\x20\x03\x20\x03\x20\x07\x20\u{1d2}\x0a\x20\x0c\x20\x0e\x20\u{1d5}\x0b\x20\
	\x05\x20\u{1d7}\x0a\x20\x03\x20\x03\x20\x07\x20\u{1db}\x0a\x20\x0c\x20\x0e\
	\x20\u{1de}\x0b\x20\x05\x20\u{1e0}\x0a\x20\x03\x21\x03\x21\x03\x21\x03\x21\
	\x07\x21\u{1e6}\x0a\x21\x0c\x21\x0e\x21\u{1e9}\x0b\x21\x03\x21\x03\x21\x03\
	\x21\x03\x21\x05\x21\u{1ef}\x0a\x21\x03\x22\x03\x22\x03\x22\x03\x22\x07\
	\x22\u{1f5}\x0a\x22\x0c\x22\x0e\x22\u{1f8}\x0b\x22\x03\x22\x03\x22\x03\x22\
	\x03\x22\x03\x22\x05\x22\u{1ff}\x0a\x22\x03\x23\x03\x23\x03\x23\x03\x23\
	\x03\x24\x03\x24\x03\x24\x03\x24\x07\x24\u{209}\x0a\x24\x0c\x24\x0e\x24\
	\u{20c}\x0b\x24\x05\x24\u{20e}\x0a\x24\x03\x24\x03\x24\x03\x25\x03\x25\x03\
	\x25\x05\x25\u{215}\x0a\x25\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x07\
	\x26\u{21c}\x0a\x26\x0c\x26\x0e\x26\u{21f}\x0b\x26\x05\x26\u{221}\x0a\x26\
	\x03\x26\x05\x26\u{224}\x0a\x26\x03\x26\x03\x26\x03\x26\x05\x26\u{229}\x0a\
	\x26\x03\x27\x05\x27\u{22c}\x0a\x27\x03\x27\x03\x27\x03\x28\x03\x28\x03\
	\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x05\
	\x28\u{23b}\x0a\x28\x03\x28\x02\x03\x22\x29\x02\x04\x06\x08\x0a\x0c\x0e\
	\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\
	\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x02\x11\x03\x03\
	\x0f\x0f\x03\x02\x21\x23\x03\x02\x24\x25\x03\x02\x3b\x3c\x03\x02\x26\x28\
	\x03\x02\x29\x2c\x03\x02\x2d\x30\x03\x02\x3f\x4a\x03\x02\x3d\x3e\x03\x02\
	\x1f\x20\x03\x02\x54\x55\x03\x02\x4b\x4e\x04\x02\x0b\x0b\x56\x56\x03\x02\
	\x0c\x0d\x03\x02\x57\x58\x02\u{278}\x02\x53\x03\x02\x02\x02\x04\x5e\x03\
	\x02\x02\x02\x06\x63\x03\x02\x02\x02\x08\x77\x03\x02\x02\x02\x0a\u{b4}\x03\
	\x02\x02\x02\x0c\u{c7}\x03\x02\x02\x02\x0e\u{cb}\x03\x02\x02\x02\x10\u{cd}\
	\x03\x02\x02\x02\x12\u{d9}\x03\x02\x02\x02\x14\u{dd}\x03\x02\x02\x02\x16\
	\u{df}\x03\x02\x02\x02\x18\u{e1}\x03\x02\x02\x02\x1a\u{ea}\x03\x02\x02\x02\
	\x1c\u{fc}\x03\x02\x02\x02\x1e\u{fe}\x03\x02\x02\x02\x20\u{103}\x03\x02\
	\x02\x02\x22\u{10a}\x03\x02\x02\x02\x24\u{144}\x03\x02\x02\x02\x26\u{14b}\
	\x03\x02\x02\x02\x28\u{154}\x03\x02\x02\x02\x2a\u{160}\x03\x02\x02\x02\x2c\
	\u{162}\x03\x02\x02\x02\x2e\u{181}\x03\x02\x02\x02\x30\u{18b}\x03\x02\x02\
	\x02\x32\u{1a0}\x03\x02\x02\x02\x34\u{1a5}\x03\x02\x02\x02\x36\u{1a9}\x03\
	\x02\x02\x02\x38\u{1ab}\x03\x02\x02\x02\x3a\u{1af}\x03\x02\x02\x02\x3c\u{1b2}\
	\x03\x02\x02\x02\x3e\u{1df}\x03\x02\x02\x02\x40\u{1ee}\x03\x02\x02\x02\x42\
	\u{1fe}\x03\x02\x02\x02\x44\u{200}\x03\x02\x02\x02\x46\u{204}\x03\x02\x02\
	\x02\x48\u{214}\x03\x02\x02\x02\x4a\u{223}\x03\x02\x02\x02\x4c\u{22b}\x03\
	\x02\x02\x02\x4e\u{23a}\x03\x02\x02\x02\x50\x52\x05\x04\x03\x02\x51\x50\
	\x03\x02\x02\x02\x52\x55\x03\x02\x02\x02\x53\x51\x03\x02\x02\x02\x53\x54\
	\x03\x02\x02\x02\x54\x59\x03\x02\x02\x02\x55\x53\x03\x02\x02\x02\x56\x58\
	\x05\x08\x05\x02\x57\x56\x03\x02\x02\x02\x58\x5b\x03\x02\x02\x02\x59\x57\
	\x03\x02\x02\x02\x59\x5a\x03\x02\x02\x02\x5a\x5c\x03\x02\x02\x02\x5b\x59\
	\x03\x02\x02\x02\x5c\x5d\x07\x02\x02\x03\x5d\x03\x03\x02\x02\x02\x5e\x5f\
	\x05\x1a\x0e\x02\x5f\x60\x07\x56\x02\x02\x60\x61\x05\x06\x04\x02\x61\x62\
	\x05\x10\x09\x02\x62\x05\x03\x02\x02\x02\x63\x6f\x07\x09\x02\x02\x64\x65\
	\x05\x1a\x0e\x02\x65\x6c\x07\x56\x02\x02\x66\x67\x07\x0e\x02\x02\x67\x68\
	\x05\x1a\x0e\x02\x68\x69\x07\x56\x02\x02\x69\x6b\x03\x02\x02\x02\x6a\x66\
	\x03\x02\x02\x02\x6b\x6e\x03\x02\x02\x02\x6c\x6a\x03\x02\x02\x02\x6c\x6d\
	\x03\x02\x02\x02\x6d\x70\x03\x02\x02\x02\x6e\x6c\x03\x02\x02\x02\x6f\x64\
	\x03\x02\x02\x02\x6f\x70\x03\x02\x02\x02\x70\x71\x03\x02\x02\x02\x71\x72\
	\x07\x0a\x02\x02\x72\x07\x03\x02\x02\x02\x73\x78\x05\x0a\x06\x02\x74\x75\
	\x05\x0c\x07\x02\x75\x76\x09\x02\x02\x02\x76\x78\x03\x02\x02\x02\x77\x73\
	\x03\x02\x02\x02\x77\x74\x03\x02\x02\x02\x78\x09\x03\x02\x02\x02\x79\x7a\
	\x07\x10\x02\x02\x7a\x7b\x07\x09\x02\x02\x7b\x7c\x05\x24\x13\x02\x7c\x7d\
	\x07\x0a\x02\x02\x7d\u{80}\x05\x0e\x08\x02\x7e\x7f\x07\x12\x02\x02\x7f\u{81}\
	\x05\x0e\x08\x02\u{80}\x7e\x03\x02\x02\x02\u{80}\u{81}\x03\x02\x02\x02\u{81}\
	\u{b5}\x03\x02\x02\x02\u{82}\u{83}\x07\x13\x02\x02\u{83}\u{84}\x07\x09\x02\
	\x02\u{84}\u{85}\x05\x24\x13\x02\u{85}\u{88}\x07\x0a\x02\x02\u{86}\u{89}\
	\x05\x0e\x08\x02\u{87}\u{89}\x05\x12\x0a\x02\u{88}\u{86}\x03\x02\x02\x02\
	\u{88}\u{87}\x03\x02\x02\x02\u{89}\u{b5}\x03\x02\x02\x02\u{8a}\u{8b}\x07\
	\x15\x02\x02\u{8b}\u{8d}\x07\x09\x02\x02\u{8c}\u{8e}\x05\x14\x0b\x02\u{8d}\
	\u{8c}\x03\x02\x02\x02\u{8d}\u{8e}\x03\x02\x02\x02\u{8e}\u{8f}\x03\x02\x02\
	\x02\u{8f}\u{91}\x07\x0f\x02\x02\u{90}\u{92}\x05\x24\x13\x02\u{91}\u{90}\
	\x03\x02\x02\x02\u{91}\u{92}\x03\x02\x02\x02\u{92}\u{93}\x03\x02\x02\x02\
	\u{93}\u{95}\x07\x0f\x02\x02\u{94}\u{96}\x05\x16\x0c\x02\u{95}\u{94}\x03\
	\x02\x02\x02\u{95}\u{96}\x03\x02\x02\x02\u{96}\u{97}\x03\x02\x02\x02\u{97}\
	\u{9a}\x07\x0a\x02\x02\u{98}\u{9b}\x05\x0e\x08\x02\u{99}\u{9b}\x05\x12\x0a\
	\x02\u{9a}\u{98}\x03\x02\x02\x02\u{9a}\u{99}\x03\x02\x02\x02\u{9b}\u{b5}\
	\x03\x02\x02\x02\u{9c}\u{9d}\x07\x15\x02\x02\u{9d}\u{9e}\x07\x09\x02\x02\
	\u{9e}\u{9f}\x05\x1a\x0e\x02\u{9f}\u{a0}\x07\x56\x02\x02\u{a0}\u{a1}\x07\
	\x37\x02\x02\u{a1}\u{a2}\x05\x24\x13\x02\u{a2}\u{a3}\x07\x0a\x02\x02\u{a3}\
	\u{a4}\x05\x0e\x08\x02\u{a4}\u{b5}\x03\x02\x02\x02\u{a5}\u{a6}\x07\x15\x02\
	\x02\u{a6}\u{a7}\x07\x09\x02\x02\u{a7}\u{a8}\x07\x56\x02\x02\u{a8}\u{a9}\
	\x07\x11\x02\x02\u{a9}\u{aa}\x05\x24\x13\x02\u{aa}\u{ab}\x07\x0a\x02\x02\
	\u{ab}\u{ac}\x05\x0e\x08\x02\u{ac}\u{b5}\x03\x02\x02\x02\u{ad}\u{ae}\x07\
	\x1a\x02\x02\u{ae}\u{b0}\x05\x10\x09\x02\u{af}\u{b1}\x05\x20\x11\x02\u{b0}\
	\u{af}\x03\x02\x02\x02\u{b1}\u{b2}\x03\x02\x02\x02\u{b2}\u{b0}\x03\x02\x02\
	\x02\u{b2}\u{b3}\x03\x02\x02\x02\u{b3}\u{b5}\x03\x02\x02\x02\u{b4}\x79\x03\
	\x02\x02\x02\u{b4}\u{82}\x03\x02\x02\x02\u{b4}\u{8a}\x03\x02\x02\x02\u{b4}\
	\u{9c}\x03\x02\x02\x02\u{b4}\u{a5}\x03\x02\x02\x02\u{b4}\u{ad}\x03\x02\x02\
	\x02\u{b5}\x0b\x03\x02\x02\x02\u{b6}\u{b7}\x07\x14\x02\x02\u{b7}\u{b8}\x05\
	\x10\x09\x02\u{b8}\u{b9}\x07\x13\x02\x02\u{b9}\u{ba}\x07\x09\x02\x02\u{ba}\
	\u{bb}\x05\x24\x13\x02\u{bb}\u{bc}\x07\x0a\x02\x02\u{bc}\u{c8}\x03\x02\x02\
	\x02\u{bd}\u{c8}\x05\x18\x0d\x02\u{be}\u{c8}\x07\x16\x02\x02\u{bf}\u{c8}\
	\x07\x17\x02\x02\u{c0}\u{c2}\x07\x18\x02\x02\u{c1}\u{c3}\x05\x24\x13\x02\
	\u{c2}\u{c1}\x03\x02\x02\x02\u{c2}\u{c3}\x03\x02\x02\x02\u{c3}\u{c8}\x03\
	\x02\x02\x02\u{c4}\u{c5}\x07\x1c\x02\x02\u{c5}\u{c8}\x05\x24\x13\x02\u{c6}\
	\u{c8}\x05\x24\x13\x02\u{c7}\u{b6}\x03\x02\x02\x02\u{c7}\u{bd}\x03\x02\x02\
	\x02\u{c7}\u{be}\x03\x02\x02\x02\u{c7}\u{bf}\x03\x02\x02\x02\u{c7}\u{c0}\
	\x03\x02\x02\x02\u{c7}\u{c4}\x03\x02\x02\x02\u{c7}\u{c6}\x03\x02\x02\x02\
	\u{c8}\x0d\x03\x02\x02\x02\u{c9}\u{cc}\x05\x10\x09\x02\u{ca}\u{cc}\x05\x08\
	\x05\x02\u{cb}\u{c9}\x03\x02\x02\x02\u{cb}\u{ca}\x03\x02\x02\x02\u{cc}\x0f\
	\x03\x02\x02\x02\u{cd}\u{d1}\x07\x05\x02\x02\u{ce}\u{d0}\x05\x08\x05\x02\
	\u{cf}\u{ce}\x03\x02\x02\x02\u{d0}\u{d3}\x03\x02\x02\x02\u{d1}\u{cf}\x03\
	\x02\x02\x02\u{d1}\u{d2}\x03\x02\x02\x02\u{d2}\u{d5}\x03\x02\x02\x02\u{d3}\
	\u{d1}\x03\x02\x02\x02\u{d4}\u{d6}\x05\x0c\x07\x02\u{d5}\u{d4}\x03\x02\x02\
	\x02\u{d5}\u{d6}\x03\x02\x02\x02\u{d6}\u{d7}\x03\x02\x02\x02\u{d7}\u{d8}\
	\x07\x06\x02\x02\u{d8}\x11\x03\x02\x02\x02\u{d9}\u{da}\x07\x0f\x02\x02\u{da}\
	\x13\x03\x02\x02\x02\u{db}\u{de}\x05\x18\x0d\x02\u{dc}\u{de}\x05\x24\x13\
	\x02\u{dd}\u{db}\x03\x02\x02\x02\u{dd}\u{dc}\x03\x02\x02\x02\u{de}\x15\x03\
	\x02\x02\x02\u{df}\u{e0}\x05\x24\x13\x02\u{e0}\x17\x03\x02\x02\x02\u{e1}\
	\u{e2}\x05\x1a\x0e\x02\u{e2}\u{e7}\x05\x1e\x10\x02\u{e3}\u{e4}\x07\x0e\x02\
	\x02\u{e4}\u{e6}\x05\x1e\x10\x02\u{e5}\u{e3}\x03\x02\x02\x02\u{e6}\u{e9}\
	\x03\x02\x02\x02\u{e7}\u{e5}\x03\x02\x02\x02\u{e7}\u{e8}\x03\x02\x02\x02\
	\u{e8}\x19\x03\x02\x02\x02\u{e9}\u{e7}\x03\x02\x02\x02\u{ea}\u{ef}\x05\x1c\
	\x0f\x02\u{eb}\u{ec}\x07\x07\x02\x02\u{ec}\u{ee}\x07\x08\x02\x02\u{ed}\u{eb}\
	\x03\x02\x02\x02\u{ee}\u{f1}\x03\x02\x02\x02\u{ef}\u{ed}\x03\x02\x02\x02\
	\u{ef}\u{f0}\x03\x02\x02\x02\u{f0}\x1b\x03\x02\x02\x02\u{f1}\u{ef}\x03\x02\
	\x02\x02\u{f2}\u{fd}\x07\x55\x02\x02\u{f3}\u{fd}\x07\x54\x02\x02\u{f4}\u{f9}\
	\x07\x56\x02\x02\u{f5}\u{f6}\x07\x0c\x02\x02\u{f6}\u{f8}\x07\x58\x02\x02\
	\u{f7}\u{f5}\x03\x02\x02\x02\u{f8}\u{fb}\x03\x02\x02\x02\u{f9}\u{f7}\x03\
	\x02\x02\x02\u{f9}\u{fa}\x03\x02\x02\x02\u{fa}\u{fd}\x03\x02\x02\x02\u{fb}\
	\u{f9}\x03\x02\x02\x02\u{fc}\u{f2}\x03\x02\x02\x02\u{fc}\u{f3}\x03\x02\x02\
	\x02\u{fc}\u{f4}\x03\x02\x02\x02\u{fd}\x1d\x03\x02\x02\x02\u{fe}\u{101}\
	\x07\x56\x02\x02\u{ff}\u{100}\x07\x3f\x02\x02\u{100}\u{102}\x05\x24\x13\
	\x02\u{101}\u{ff}\x03\x02\x02\x02\u{101}\u{102}\x03\x02\x02\x02\u{102}\x1f\
	\x03\x02\x02\x02\u{103}\u{104}\x07\x1b\x02\x02\u{104}\u{105}\x07\x09\x02\
	\x02\u{105}\u{106}\x05\x1c\x0f\x02\u{106}\u{107}\x07\x56\x02\x02\u{107}\
	\u{108}\x07\x0a\x02\x02\u{108}\u{109}\x05\x10\x09\x02\u{109}\x21\x03\x02\
	\x02\x02\u{10a}\u{10b}\x08\x12\x01\x02\u{10b}\u{10c}\x05\x26\x14\x02\u{10c}\
	\u{136}\x03\x02\x02\x02\u{10d}\u{10e}\x0c\x0f\x02\x02\u{10e}\u{10f}\x09\
	\x03\x02\x02\u{10f}\u{135}\x05\x22\x12\x10\u{110}\u{111}\x0c\x0e\x02\x02\
	\u{111}\u{112}\x09\x04\x02\x02\u{112}\u{135}\x05\x22\x12\x0f\u{113}\u{114}\
	\x0c\x0d\x02\x02\u{114}\u{115}\x09\x05\x02\x02\u{115}\u{135}\x05\x22\x12\
	\x0e\u{116}\u{117}\x0c\x0c\x02\x02\u{117}\u{118}\x09\x06\x02\x02\u{118}\
	\u{135}\x05\x22\x12\x0d\u{119}\u{11a}\x0c\x0b\x02\x02\u{11a}\u{11b}\x09\
	\x07\x02\x02\u{11b}\u{135}\x05\x22\x12\x0c\u{11c}\u{11d}\x0c\x09\x02\x02\
	\u{11d}\u{11e}\x09\x08\x02\x02\u{11e}\u{135}\x05\x22\x12\x0a\u{11f}\u{120}\
	\x0c\x08\x02\x02\u{120}\u{121}\x07\x31\x02\x02\u{121}\u{135}\x05\x22\x12\
	\x09\u{122}\u{123}\x0c\x07\x02\x02\u{123}\u{124}\x07\x32\x02\x02\u{124}\
	\u{135}\x05\x22\x12\x08\u{125}\u{126}\x0c\x06\x02\x02\u{126}\u{127}\x07\
	\x33\x02\x02\u{127}\u{135}\x05\x22\x12\x07\u{128}\u{129}\x0c\x05\x02\x02\
	\u{129}\u{12a}\x07\x34\x02\x02\u{12a}\u{135}\x05\x22\x12\x06\u{12b}\u{12c}\
	\x0c\x04\x02\x02\u{12c}\u{12d}\x07\x35\x02\x02\u{12d}\u{135}\x05\x22\x12\
	\x05\u{12e}\u{12f}\x0c\x03\x02\x02\u{12f}\u{130}\x07\x38\x02\x02\u{130}\
	\u{135}\x05\x22\x12\x03\u{131}\u{132}\x0c\x0a\x02\x02\u{132}\u{133}\x07\
	\x1e\x02\x02\u{133}\u{135}\x05\x1a\x0e\x02\u{134}\u{10d}\x03\x02\x02\x02\
	\u{134}\u{110}\x03\x02\x02\x02\u{134}\u{113}\x03\x02\x02\x02\u{134}\u{116}\
	\x03\x02\x02\x02\u{134}\u{119}\x03\x02\x02\x02\u{134}\u{11c}\x03\x02\x02\
	\x02\u{134}\u{11f}\x03\x02\x02\x02\u{134}\u{122}\x03\x02\x02\x02\u{134}\
	\u{125}\x03\x02\x02\x02\u{134}\u{128}\x03\x02\x02\x02\u{134}\u{12b}\x03\
	\x02\x02\x02\u{134}\u{12e}\x03\x02\x02\x02\u{134}\u{131}\x03\x02\x02\x02\
	\u{135}\u{138}\x03\x02\x02\x02\u{136}\u{134}\x03\x02\x02\x02\u{136}\u{137}\
	\x03\x02\x02\x02\u{137}\x23\x03\x02\x02\x02\u{138}\u{136}\x03\x02\x02\x02\
	\u{139}\u{145}\x05\x22\x12\x02\u{13a}\u{13b}\x05\x22\x12\x02\u{13b}\u{13c}\
	\x07\x36\x02\x02\u{13c}\u{13d}\x05\x24\x13\x02\u{13d}\u{13e}\x07\x37\x02\
	\x02\u{13e}\u{13f}\x05\x24\x13\x02\u{13f}\u{145}\x03\x02\x02\x02\u{140}\
	\u{141}\x05\x22\x12\x02\u{141}\u{142}\x09\x09\x02\x02\u{142}\u{143}\x05\
	\x24\x13\x02\u{143}\u{145}\x03\x02\x02\x02\u{144}\u{139}\x03\x02\x02\x02\
	\u{144}\u{13a}\x03\x02\x02\x02\u{144}\u{140}\x03\x02\x02\x02\u{145}\x25\
	\x03\x02\x02\x02\u{146}\u{147}\x09\x0a\x02\x02\u{147}\u{14c}\x05\x30\x19\
	\x02\u{148}\u{149}\x09\x04\x02\x02\u{149}\u{14c}\x05\x26\x14\x02\u{14a}\
	\u{14c}\x05\x28\x15\x02\u{14b}\u{146}\x03\x02\x02\x02\u{14b}\u{148}\x03\
	\x02\x02\x02\u{14b}\u{14a}\x03\x02\x02\x02\u{14c}\x27\x03\x02\x02\x02\u{14d}\
	\u{155}\x05\x30\x19\x02\u{14e}\u{14f}\x05\x30\x19\x02\u{14f}\u{150}\x09\
	\x0a\x02\x02\u{150}\u{155}\x03\x02\x02\x02\u{151}\u{152}\x09\x0b\x02\x02\
	\u{152}\u{155}\x05\x26\x14\x02\u{153}\u{155}\x05\x2a\x16\x02\u{154}\u{14d}\
	\x03\x02\x02\x02\u{154}\u{14e}\x03\x02\x02\x02\u{154}\u{151}\x03\x02\x02\
	\x02\u{154}\u{153}\x03\x02\x02\x02\u{155}\x29\x03\x02\x02\x02\u{156}\u{157}\
	\x07\x09\x02\x02\u{157}\u{158}\x05\x2c\x17\x02\u{158}\u{159}\x07\x0a\x02\
	\x02\u{159}\u{15a}\x05\x26\x14\x02\u{15a}\u{161}\x03\x02\x02\x02\u{15b}\
	\u{15c}\x07\x09\x02\x02\u{15c}\u{15d}\x05\x2e\x18\x02\u{15d}\u{15e}\x07\
	\x0a\x02\x02\u{15e}\u{15f}\x05\x28\x15\x02\u{15f}\u{161}\x03\x02\x02\x02\
	\u{160}\u{156}\x03\x02\x02\x02\u{160}\u{15b}\x03\x02\x02\x02\u{161}\x2b\
	\x03\x02\x02\x02\u{162}\u{163}\x09\x0c\x02\x02\u{163}\x2d\x03\x02\x02\x02\
	\u{164}\u{167}\x07\x55\x02\x02\u{165}\u{166}\x07\x07\x02\x02\u{166}\u{168}\
	\x07\x08\x02\x02\u{167}\u{165}\x03\x02\x02\x02\u{168}\u{169}\x03\x02\x02\
	\x02\u{169}\u{167}\x03\x02\x02\x02\u{169}\u{16a}\x03\x02\x02\x02\u{16a}\
	\u{182}\x03\x02\x02\x02\u{16b}\u{16e}\x07\x54\x02\x02\u{16c}\u{16d}\x07\
	\x07\x02\x02\u{16d}\u{16f}\x07\x08\x02\x02\u{16e}\u{16c}\x03\x02\x02\x02\
	\u{16f}\u{170}\x03\x02\x02\x02\u{170}\u{16e}\x03\x02\x02\x02\u{170}\u{171}\
	\x03\x02\x02\x02\u{171}\u{182}\x03\x02\x02\x02\u{172}\u{177}\x07\x56\x02\
	\x02\u{173}\u{174}\x07\x0c\x02\x02\u{174}\u{176}\x07\x58\x02\x02\u{175}\
	\u{173}\x03\x02\x02\x02\u{176}\u{179}\x03\x02\x02\x02\u{177}\u{175}\x03\
	\x02\x02\x02\u{177}\u{178}\x03\x02\x02\x02\u{178}\u{17e}\x03\x02\x02\x02\
	\u{179}\u{177}\x03\x02\x02\x02\u{17a}\u{17b}\x07\x07\x02\x02\u{17b}\u{17d}\
	\x07\x08\x02\x02\u{17c}\u{17a}\x03\x02\x02\x02\u{17d}\u{180}\x03\x02\x02\
	\x02\u{17e}\u{17c}\x03\x02\x02\x02\u{17e}\u{17f}\x03\x02\x02\x02\u{17f}\
	\u{182}\x03\x02\x02\x02\u{180}\u{17e}\x03\x02\x02\x02\u{181}\u{164}\x03\
	\x02\x02\x02\u{181}\u{16b}\x03\x02\x02\x02\u{181}\u{172}\x03\x02\x02\x02\
	\u{182}\x2f\x03\x02\x02\x02\u{183}\u{187}\x05\x32\x1a\x02\u{184}\u{186}\
	\x05\x34\x1b\x02\u{185}\u{184}\x03\x02\x02\x02\u{186}\u{189}\x03\x02\x02\
	\x02\u{187}\u{185}\x03\x02\x02\x02\u{187}\u{188}\x03\x02\x02\x02\u{188}\
	\u{18c}\x03\x02\x02\x02\u{189}\u{187}\x03\x02\x02\x02\u{18a}\u{18c}\x05\
	\x3e\x20\x02\u{18b}\u{183}\x03\x02\x02\x02\u{18b}\u{18a}\x03\x02\x02\x02\
	\u{18c}\x31\x03\x02\x02\x02\u{18d}\u{18e}\x07\x09\x02\x02\u{18e}\u{18f}\
	\x05\x24\x13\x02\u{18f}\u{190}\x07\x0a\x02\x02\u{190}\u{1a1}\x03\x02\x02\
	\x02\u{191}\u{1a1}\x09\x0d\x02\x02\u{192}\u{1a1}\x07\x51\x02\x02\u{193}\
	\u{1a1}\x07\x52\x02\x02\u{194}\u{1a1}\x07\x53\x02\x02\u{195}\u{1a1}\x07\
	\x4f\x02\x02\u{196}\u{1a1}\x07\x50\x02\x02\u{197}\u{1a1}\x05\x40\x21\x02\
	\u{198}\u{1a1}\x05\x42\x22\x02\u{199}\u{1a1}\x07\x56\x02\x02\u{19a}\u{19b}\
	\x09\x0e\x02\x02\u{19b}\u{1a1}\x05\x46\x24\x02\u{19c}\u{19d}\x07\x19\x02\
	\x02\u{19d}\u{19e}\x05\x1c\x0f\x02\u{19e}\u{19f}\x05\x46\x24\x02\u{19f}\
	\u{1a1}\x03\x02\x02\x02\u{1a0}\u{18d}\x03\x02\x02\x02\u{1a0}\u{191}\x03\
	\x02\x02\x02\u{1a0}\u{192}\x03\x02\x02\x02\u{1a0}\u{193}\x03\x02\x02\x02\
	\u{1a0}\u{194}\x03\x02\x02\x02\u{1a0}\u{195}\x03\x02\x02\x02\u{1a0}\u{196}\
	\x03\x02\x02\x02\u{1a0}\u{197}\x03\x02\x02\x02\u{1a0}\u{198}\x03\x02\x02\
	\x02\u{1a0}\u{199}\x03\x02\x02\x02\u{1a0}\u{19a}\x03\x02\x02\x02\u{1a0}\
	\u{19c}\x03\x02\x02\x02\u{1a1}\x33\x03\x02\x02\x02\u{1a2}\u{1a6}\x05\x38\
	\x1d\x02\u{1a3}\u{1a6}\x05\x3a\x1e\x02\u{1a4}\u{1a6}\x05\x3c\x1f\x02\u{1a5}\
	\u{1a2}\x03\x02\x02\x02\u{1a5}\u{1a3}\x03\x02\x02\x02\u{1a5}\u{1a4}\x03\
	\x02\x02\x02\u{1a6}\x35\x03\x02\x02\x02\u{1a7}\u{1aa}\x05\x38\x1d\x02\u{1a8}\
	\u{1aa}\x05\x3a\x1e\x02\u{1a9}\u{1a7}\x03\x02\x02\x02\u{1a9}\u{1a8}\x03\
	\x02\x02\x02\u{1aa}\x37\x03\x02\x02\x02\u{1ab}\u{1ac}\x09\x0f\x02\x02\u{1ac}\
	\u{1ad}\x07\x58\x02\x02\u{1ad}\u{1ae}\x05\x46\x24\x02\u{1ae}\x39\x03\x02\
	\x02\x02\u{1af}\u{1b0}\x09\x0f\x02\x02\u{1b0}\u{1b1}\x09\x10\x02\x02\u{1b1}\
	\x3b\x03\x02\x02\x02\u{1b2}\u{1b3}\x07\x07\x02\x02\u{1b3}\u{1b4}\x05\x24\
	\x13\x02\u{1b4}\u{1b5}\x07\x08\x02\x02\u{1b5}\x3d\x03\x02\x02\x02\u{1b6}\
	\u{1b7}\x07\x19\x02\x02\u{1b7}\u{1bc}\x05\x1c\x0f\x02\u{1b8}\u{1b9}\x07\
	\x07\x02\x02\u{1b9}\u{1ba}\x05\x24\x13\x02\u{1ba}\u{1bb}\x07\x08\x02\x02\
	\u{1bb}\u{1bd}\x03\x02\x02\x02\u{1bc}\u{1b8}\x03\x02\x02\x02\u{1bd}\u{1be}\
	\x03\x02\x02\x02\u{1be}\u{1bc}\x03\x02\x02\x02\u{1be}\u{1bf}\x03\x02\x02\
	\x02\u{1bf}\u{1c7}\x03\x02\x02\x02\u{1c0}\u{1c4}\x05\x36\x1c\x02\u{1c1}\
	\u{1c3}\x05\x34\x1b\x02\u{1c2}\u{1c1}\x03\x02\x02\x02\u{1c3}\u{1c6}\x03\
	\x02\x02\x02\u{1c4}\u{1c2}\x03\x02\x02\x02\u{1c4}\u{1c5}\x03\x02\x02\x02\
	\u{1c5}\u{1c8}\x03\x02\x02\x02\u{1c6}\u{1c4}\x03\x02\x02\x02\u{1c7}\u{1c0}\
	\x03\x02\x02\x02\u{1c7}\u{1c8}\x03\x02\x02\x02\u{1c8}\u{1e0}\x03\x02\x02\
	\x02\u{1c9}\u{1ca}\x07\x19\x02\x02\u{1ca}\u{1cb}\x05\x1c\x0f\x02\u{1cb}\
	\u{1cc}\x07\x07\x02\x02\u{1cc}\u{1cd}\x07\x08\x02\x02\u{1cd}\u{1d6}\x07\
	\x05\x02\x02\u{1ce}\u{1d3}\x05\x24\x13\x02\u{1cf}\u{1d0}\x07\x0e\x02\x02\
	\u{1d0}\u{1d2}\x05\x24\x13\x02\u{1d1}\u{1cf}\x03\x02\x02\x02\u{1d2}\u{1d5}\
	\x03\x02\x02\x02\u{1d3}\u{1d1}\x03\x02\x02\x02\u{1d3}\u{1d4}\x03\x02\x02\
	\x02\u{1d4}\u{1d7}\x03\x02\x02\x02\u{1d5}\u{1d3}\x03\x02\x02\x02\u{1d6}\
	\u{1ce}\x03\x02\x02\x02\u{1d6}\u{1d7}\x03\x02\x02\x02\u{1d7}\u{1d8}\x03\
	\x02\x02\x02\u{1d8}\u{1dc}\x07\x06\x02\x02\u{1d9}\u{1db}\x05\x34\x1b\x02\
	\u{1da}\u{1d9}\x03\x02\x02\x02\u{1db}\u{1de}\x03\x02\x02\x02\u{1dc}\u{1da}\
	\x03\x02\x02\x02\u{1dc}\u{1dd}\x03\x02\x02\x02\u{1dd}\u{1e0}\x03\x02\x02\
	\x02\u{1de}\u{1dc}\x03\x02\x02\x02\u{1df}\u{1b6}\x03\x02\x02\x02\u{1df}\
	\u{1c9}\x03\x02\x02\x02\u{1e0}\x3f\x03\x02\x02\x02\u{1e1}\u{1e2}\x07\x07\
	\x02\x02\u{1e2}\u{1e7}\x05\x24\x13\x02\u{1e3}\u{1e4}\x07\x0e\x02\x02\u{1e4}\
	\u{1e6}\x05\x24\x13\x02\u{1e5}\u{1e3}\x03\x02\x02\x02\u{1e6}\u{1e9}\x03\
	\x02\x02\x02\u{1e7}\u{1e5}\x03\x02\x02\x02\u{1e7}\u{1e8}\x03\x02\x02\x02\
	\u{1e8}\u{1ea}\x03\x02\x02\x02\u{1e9}\u{1e7}\x03\x02\x02\x02\u{1ea}\u{1eb}\
	\x07\x08\x02\x02\u{1eb}\u{1ef}\x03\x02\x02\x02\u{1ec}\u{1ed}\x07\x07\x02\
	\x02\u{1ed}\u{1ef}\x07\x08\x02\x02\u{1ee}\u{1e1}\x03\x02\x02\x02\u{1ee}\
	\u{1ec}\x03\x02\x02\x02\u{1ef}\x41\x03\x02\x02\x02\u{1f0}\u{1f1}\x07\x07\
	\x02\x02\u{1f1}\u{1f6}\x05\x44\x23\x02\u{1f2}\u{1f3}\x07\x0e\x02\x02\u{1f3}\
	\u{1f5}\x05\x44\x23\x02\u{1f4}\u{1f2}\x03\x02\x02\x02\u{1f5}\u{1f8}\x03\
	\x02\x02\x02\u{1f6}\u{1f4}\x03\x02\x02\x02\u{1f6}\u{1f7}\x03\x02\x02\x02\
	\u{1f7}\u{1f9}\x03\x02\x02\x02\u{1f8}\u{1f6}\x03\x02\x02\x02\u{1f9}\u{1fa}\
	\x07\x08\x02\x02\u{1fa}\u{1ff}\x03\x02\x02\x02\u{1fb}\u{1fc}\x07\x07\x02\
	\x02\u{1fc}\u{1fd}\x07\x37\x02\x02\u{1fd}\u{1ff}\x07\x08\x02\x02\u{1fe}\
	\u{1f0}\x03\x02\x02\x02\u{1fe}\u{1fb}\x03\x02\x02\x02\u{1ff}\x43\x03\x02\
	\x02\x02\u{200}\u{201}\x05\x24\x13\x02\u{201}\u{202}\x07\x37\x02\x02\u{202}\
	\u{203}\x05\x24\x13\x02\u{203}\x45\x03\x02\x02\x02\u{204}\u{20d}\x07\x09\
	\x02\x02\u{205}\u{20a}\x05\x48\x25\x02\u{206}\u{207}\x07\x0e\x02\x02\u{207}\
	\u{209}\x05\x48\x25\x02\u{208}\u{206}\x03\x02\x02\x02\u{209}\u{20c}\x03\
	\x02\x02\x02\u{20a}\u{208}\x03\x02\x02\x02\u{20a}\u{20b}\x03\x02\x02\x02\
	\u{20b}\u{20e}\x03\x02\x02\x02\u{20c}\u{20a}\x03\x02\x02\x02\u{20d}\u{205}\
	\x03\x02\x02\x02\u{20d}\u{20e}\x03\x02\x02\x02\u{20e}\u{20f}\x03\x02\x02\
	\x02\u{20f}\u{210}\x07\x0a\x02\x02\u{210}\x47\x03\x02\x02\x02\u{211}\u{215}\
	\x05\x24\x13\x02\u{212}\u{215}\x05\x4a\x26\x02\u{213}\u{215}\x05\x4e\x28\
	\x02\u{214}\u{211}\x03\x02\x02\x02\u{214}\u{212}\x03\x02\x02\x02\u{214}\
	\u{213}\x03\x02\x02\x02\u{215}\x49\x03\x02\x02\x02\u{216}\u{224}\x05\x4c\
	\x27\x02\u{217}\u{220}\x07\x09\x02\x02\u{218}\u{21d}\x05\x4c\x27\x02\u{219}\
	\u{21a}\x07\x0e\x02\x02\u{21a}\u{21c}\x05\x4c\x27\x02\u{21b}\u{219}\x03\
	\x02\x02\x02\u{21c}\u{21f}\x03\x02\x02\x02\u{21d}\u{21b}\x03\x02\x02\x02\
	\u{21d}\u{21e}\x03\x02\x02\x02\u{21e}\u{221}\x03\x02\x02\x02\u{21f}\u{21d}\
	\x03\x02\x02\x02\u{220}\u{218}\x03\x02\x02\x02\u{220}\u{221}\x03\x02\x02\
	\x02\u{221}\u{222}\x03\x02\x02\x02\u{222}\u{224}\x07\x0a\x02\x02\u{223}\
	\u{216}\x03\x02\x02\x02\u{223}\u{217}\x03\x02\x02\x02\u{224}\u{225}\x03\
	\x02\x02\x02\u{225}\u{228}\x07\x3a\x02\x02\u{226}\u{229}\x05\x10\x09\x02\
	\u{227}\u{229}\x05\x24\x13\x02\u{228}\u{226}\x03\x02\x02\x02\u{228}\u{227}\
	\x03\x02\x02\x02\u{229}\x4b\x03\x02\x02\x02\u{22a}\u{22c}\x05\x1a\x0e\x02\
	\u{22b}\u{22a}\x03\x02\x02\x02\u{22b}\u{22c}\x03\x02\x02\x02\u{22c}\u{22d}\
	\x03\x02\x02\x02\u{22d}\u{22e}\x07\x56\x02\x02\u{22e}\x4d\x03\x02\x02\x02\
	\u{22f}\u{230}\x05\x1a\x0e\x02\u{230}\u{231}\x07\x39\x02\x02\u{231}\u{232}\
	\x07\x56\x02\x02\u{232}\u{23b}\x03\x02\x02\x02\u{233}\u{234}\x05\x1a\x0e\
	\x02\u{234}\u{235}\x07\x39\x02\x02\u{235}\u{236}\x07\x19\x02\x02\u{236}\
	\u{23b}\x03\x02\x02\x02\u{237}\u{238}\x07\x1d\x02\x02\u{238}\u{239}\x07\
	\x39\x02\x02\u{239}\u{23b}\x07\x56\x02\x02\u{23a}\u{22f}\x03\x02\x02\x02\
	\u{23a}\u{233}\x03\x02\x02\x02\u{23a}\u{237}\x03\x02\x02\x02\u{23b}\x4f\
	\x03\x02\x02\x02\x3e\x53\x59\x6c\x6f\x77\u{80}\u{88}\u{8d}\u{91}\u{95}\u{9a}\
	\u{b2}\u{b4}\u{c2}\u{c7}\u{cb}\u{d1}\u{d5}\u{dd}\u{e7}\u{ef}\u{f9}\u{fc}\
	\u{101}\u{134}\u{136}\u{144}\u{14b}\u{154}\u{160}\u{169}\u{170}\u{177}\u{17e}\
	\u{181}\u{187}\u{18b}\u{1a0}\u{1a5}\u{1a9}\u{1be}\u{1c4}\u{1c7}\u{1d3}\u{1d6}\
	\u{1dc}\u{1df}\u{1e7}\u{1ee}\u{1f6}\u{1fe}\u{20a}\u{20d}\u{214}\u{21d}\u{220}\
	\u{223}\u{228}\u{22b}\u{23a}";
