#![allow(nonstandard_style)]
// Generated from PainlessParser.g4 by ANTLR 4.8
use super::painlessparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait PainlessParserListener<'input>:
    ParseTreeListener<'input, PainlessParserContextType>
{
    /**
     * Enter a parse tree produced by {@link PainlessParser#source}.
     * @param ctx the parse tree
     */
    fn enter_source(&mut self, _ctx: &SourceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#source}.
     * @param ctx the parse tree
     */
    fn exit_source(&mut self, _ctx: &SourceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#function}.
     * @param ctx the parse tree
     */
    fn enter_function(&mut self, _ctx: &FunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#function}.
     * @param ctx the parse tree
     */
    fn exit_function(&mut self, _ctx: &FunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#parameters}.
     * @param ctx the parse tree
     */
    fn enter_parameters(&mut self, _ctx: &ParametersContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#parameters}.
     * @param ctx the parse tree
     */
    fn exit_parameters(&mut self, _ctx: &ParametersContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#statement}.
     * @param ctx the parse tree
     */
    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#statement}.
     * @param ctx the parse tree
     */
    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code if}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn enter_if(&mut self, _ctx: &IfContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code if}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn exit_if(&mut self, _ctx: &IfContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code while}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn enter_while(&mut self, _ctx: &WhileContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code while}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn exit_while(&mut self, _ctx: &WhileContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code for}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn enter_for(&mut self, _ctx: &ForContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code for}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn exit_for(&mut self, _ctx: &ForContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code each}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn enter_each(&mut self, _ctx: &EachContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code each}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn exit_each(&mut self, _ctx: &EachContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ineach}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn enter_ineach(&mut self, _ctx: &IneachContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ineach}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn exit_ineach(&mut self, _ctx: &IneachContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code try}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn enter_try(&mut self, _ctx: &TryContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code try}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn exit_try(&mut self, _ctx: &TryContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code do}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn enter_do(&mut self, _ctx: &DoContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code do}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn exit_do(&mut self, _ctx: &DoContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code decl}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn enter_decl(&mut self, _ctx: &DeclContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code decl}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn exit_decl(&mut self, _ctx: &DeclContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code continue}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn enter_continue(&mut self, _ctx: &ContinueContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code continue}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn exit_continue(&mut self, _ctx: &ContinueContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code break}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn enter_break(&mut self, _ctx: &BreakContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code break}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn exit_break(&mut self, _ctx: &BreakContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code return}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn enter_return(&mut self, _ctx: &ReturnContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code return}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn exit_return(&mut self, _ctx: &ReturnContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code throw}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn enter_throw(&mut self, _ctx: &ThrowContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code throw}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn exit_throw(&mut self, _ctx: &ThrowContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code expr}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn enter_expr(&mut self, _ctx: &ExprContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code expr}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn exit_expr(&mut self, _ctx: &ExprContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#trailer}.
     * @param ctx the parse tree
     */
    fn enter_trailer(&mut self, _ctx: &TrailerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#trailer}.
     * @param ctx the parse tree
     */
    fn exit_trailer(&mut self, _ctx: &TrailerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#block}.
     * @param ctx the parse tree
     */
    fn enter_block(&mut self, _ctx: &BlockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#block}.
     * @param ctx the parse tree
     */
    fn exit_block(&mut self, _ctx: &BlockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#empty}.
     * @param ctx the parse tree
     */
    fn enter_empty(&mut self, _ctx: &EmptyContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#empty}.
     * @param ctx the parse tree
     */
    fn exit_empty(&mut self, _ctx: &EmptyContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#initializer}.
     * @param ctx the parse tree
     */
    fn enter_initializer(&mut self, _ctx: &InitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#initializer}.
     * @param ctx the parse tree
     */
    fn exit_initializer(&mut self, _ctx: &InitializerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#afterthought}.
     * @param ctx the parse tree
     */
    fn enter_afterthought(&mut self, _ctx: &AfterthoughtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#afterthought}.
     * @param ctx the parse tree
     */
    fn exit_afterthought(&mut self, _ctx: &AfterthoughtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#declaration}.
     * @param ctx the parse tree
     */
    fn enter_declaration(&mut self, _ctx: &DeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#declaration}.
     * @param ctx the parse tree
     */
    fn exit_declaration(&mut self, _ctx: &DeclarationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#decltype}.
     * @param ctx the parse tree
     */
    fn enter_decltype(&mut self, _ctx: &DecltypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#decltype}.
     * @param ctx the parse tree
     */
    fn exit_decltype(&mut self, _ctx: &DecltypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#type_}.
     * @param ctx the parse tree
     */
    fn enter_type_(&mut self, _ctx: &Type_Context<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#type_}.
     * @param ctx the parse tree
     */
    fn exit_type_(&mut self, _ctx: &Type_Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#declvar}.
     * @param ctx the parse tree
     */
    fn enter_declvar(&mut self, _ctx: &DeclvarContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#declvar}.
     * @param ctx the parse tree
     */
    fn exit_declvar(&mut self, _ctx: &DeclvarContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#trap}.
     * @param ctx the parse tree
     */
    fn enter_trap(&mut self, _ctx: &TrapContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#trap}.
     * @param ctx the parse tree
     */
    fn exit_trap(&mut self, _ctx: &TrapContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code single}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn enter_single(&mut self, _ctx: &SingleContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code single}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn exit_single(&mut self, _ctx: &SingleContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code comp}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn enter_comp(&mut self, _ctx: &CompContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code comp}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn exit_comp(&mut self, _ctx: &CompContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code bool}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn enter_bool(&mut self, _ctx: &BoolContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code bool}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn exit_bool(&mut self, _ctx: &BoolContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code binary}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn enter_binary(&mut self, _ctx: &BinaryContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code binary}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn exit_binary(&mut self, _ctx: &BinaryContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code elvis}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn enter_elvis(&mut self, _ctx: &ElvisContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code elvis}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn exit_elvis(&mut self, _ctx: &ElvisContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code instanceof}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn enter_instanceof(&mut self, _ctx: &InstanceofContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code instanceof}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn exit_instanceof(&mut self, _ctx: &InstanceofContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code nonconditional}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_nonconditional(&mut self, _ctx: &NonconditionalContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code nonconditional}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_nonconditional(&mut self, _ctx: &NonconditionalContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code conditional}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_conditional(&mut self, _ctx: &ConditionalContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code conditional}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_conditional(&mut self, _ctx: &ConditionalContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code assignment}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_assignment(&mut self, _ctx: &AssignmentContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code assignment}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_assignment(&mut self, _ctx: &AssignmentContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code pre}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn enter_pre(&mut self, _ctx: &PreContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code pre}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn exit_pre(&mut self, _ctx: &PreContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code addsub}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn enter_addsub(&mut self, _ctx: &AddsubContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code addsub}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn exit_addsub(&mut self, _ctx: &AddsubContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code notaddsub}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn enter_notaddsub(&mut self, _ctx: &NotaddsubContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code notaddsub}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn exit_notaddsub(&mut self, _ctx: &NotaddsubContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code read}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn enter_read(&mut self, _ctx: &ReadContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code read}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn exit_read(&mut self, _ctx: &ReadContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code post}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn enter_post(&mut self, _ctx: &PostContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code post}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn exit_post(&mut self, _ctx: &PostContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code not}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn enter_not(&mut self, _ctx: &NotContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code not}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn exit_not(&mut self, _ctx: &NotContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code cast}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn enter_cast(&mut self, _ctx: &CastContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code cast}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn exit_cast(&mut self, _ctx: &CastContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code primordefcast}
     * labeled alternative in {@link PainlessParser#castexpression}.
     * @param ctx the parse tree
     */
    fn enter_primordefcast(&mut self, _ctx: &PrimordefcastContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code primordefcast}
     * labeled alternative in {@link PainlessParser#castexpression}.
     * @param ctx the parse tree
     */
    fn exit_primordefcast(&mut self, _ctx: &PrimordefcastContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code refcast}
     * labeled alternative in {@link PainlessParser#castexpression}.
     * @param ctx the parse tree
     */
    fn enter_refcast(&mut self, _ctx: &RefcastContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code refcast}
     * labeled alternative in {@link PainlessParser#castexpression}.
     * @param ctx the parse tree
     */
    fn exit_refcast(&mut self, _ctx: &RefcastContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#primordefcasttype}.
     * @param ctx the parse tree
     */
    fn enter_primordefcasttype(&mut self, _ctx: &PrimordefcasttypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#primordefcasttype}.
     * @param ctx the parse tree
     */
    fn exit_primordefcasttype(&mut self, _ctx: &PrimordefcasttypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#refcasttype}.
     * @param ctx the parse tree
     */
    fn enter_refcasttype(&mut self, _ctx: &RefcasttypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#refcasttype}.
     * @param ctx the parse tree
     */
    fn exit_refcasttype(&mut self, _ctx: &RefcasttypeContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code dynamic}
     * labeled alternative in {@link PainlessParser#chain}.
     * @param ctx the parse tree
     */
    fn enter_dynamic(&mut self, _ctx: &DynamicContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code dynamic}
     * labeled alternative in {@link PainlessParser#chain}.
     * @param ctx the parse tree
     */
    fn exit_dynamic(&mut self, _ctx: &DynamicContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code newarray}
     * labeled alternative in {@link PainlessParser#chain}.
     * @param ctx the parse tree
     */
    fn enter_newarray(&mut self, _ctx: &NewarrayContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code newarray}
     * labeled alternative in {@link PainlessParser#chain}.
     * @param ctx the parse tree
     */
    fn exit_newarray(&mut self, _ctx: &NewarrayContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code precedence}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_precedence(&mut self, _ctx: &PrecedenceContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code precedence}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_precedence(&mut self, _ctx: &PrecedenceContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code numeric}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_numeric(&mut self, _ctx: &NumericContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code numeric}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_numeric(&mut self, _ctx: &NumericContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code true}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_true(&mut self, _ctx: &TrueContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code true}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_true(&mut self, _ctx: &TrueContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code false}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_false(&mut self, _ctx: &FalseContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code false}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_false(&mut self, _ctx: &FalseContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code null}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_null(&mut self, _ctx: &NullContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code null}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_null(&mut self, _ctx: &NullContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code string}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_string(&mut self, _ctx: &StringContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code string}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_string(&mut self, _ctx: &StringContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code regex}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_regex(&mut self, _ctx: &RegexContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code regex}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_regex(&mut self, _ctx: &RegexContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code listinit}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_listinit(&mut self, _ctx: &ListinitContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code listinit}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_listinit(&mut self, _ctx: &ListinitContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code mapinit}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_mapinit(&mut self, _ctx: &MapinitContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code mapinit}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_mapinit(&mut self, _ctx: &MapinitContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code variable}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_variable(&mut self, _ctx: &VariableContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code variable}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_variable(&mut self, _ctx: &VariableContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code calllocal}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_calllocal(&mut self, _ctx: &CalllocalContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code calllocal}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_calllocal(&mut self, _ctx: &CalllocalContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code newobject}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn enter_newobject(&mut self, _ctx: &NewobjectContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code newobject}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn exit_newobject(&mut self, _ctx: &NewobjectContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#postfix}.
     * @param ctx the parse tree
     */
    fn enter_postfix(&mut self, _ctx: &PostfixContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#postfix}.
     * @param ctx the parse tree
     */
    fn exit_postfix(&mut self, _ctx: &PostfixContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#postdot}.
     * @param ctx the parse tree
     */
    fn enter_postdot(&mut self, _ctx: &PostdotContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#postdot}.
     * @param ctx the parse tree
     */
    fn exit_postdot(&mut self, _ctx: &PostdotContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#callinvoke}.
     * @param ctx the parse tree
     */
    fn enter_callinvoke(&mut self, _ctx: &CallinvokeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#callinvoke}.
     * @param ctx the parse tree
     */
    fn exit_callinvoke(&mut self, _ctx: &CallinvokeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#fieldaccess}.
     * @param ctx the parse tree
     */
    fn enter_fieldaccess(&mut self, _ctx: &FieldaccessContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#fieldaccess}.
     * @param ctx the parse tree
     */
    fn exit_fieldaccess(&mut self, _ctx: &FieldaccessContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#braceaccess}.
     * @param ctx the parse tree
     */
    fn enter_braceaccess(&mut self, _ctx: &BraceaccessContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#braceaccess}.
     * @param ctx the parse tree
     */
    fn exit_braceaccess(&mut self, _ctx: &BraceaccessContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code newstandardarray}
     * labeled alternative in {@link PainlessParser#arrayinitializer}.
     * @param ctx the parse tree
     */
    fn enter_newstandardarray(&mut self, _ctx: &NewstandardarrayContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code newstandardarray}
     * labeled alternative in {@link PainlessParser#arrayinitializer}.
     * @param ctx the parse tree
     */
    fn exit_newstandardarray(&mut self, _ctx: &NewstandardarrayContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code newinitializedarray}
     * labeled alternative in {@link PainlessParser#arrayinitializer}.
     * @param ctx the parse tree
     */
    fn enter_newinitializedarray(&mut self, _ctx: &NewinitializedarrayContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code newinitializedarray}
     * labeled alternative in {@link PainlessParser#arrayinitializer}.
     * @param ctx the parse tree
     */
    fn exit_newinitializedarray(&mut self, _ctx: &NewinitializedarrayContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#listinitializer}.
     * @param ctx the parse tree
     */
    fn enter_listinitializer(&mut self, _ctx: &ListinitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#listinitializer}.
     * @param ctx the parse tree
     */
    fn exit_listinitializer(&mut self, _ctx: &ListinitializerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#mapinitializer}.
     * @param ctx the parse tree
     */
    fn enter_mapinitializer(&mut self, _ctx: &MapinitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#mapinitializer}.
     * @param ctx the parse tree
     */
    fn exit_mapinitializer(&mut self, _ctx: &MapinitializerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#maptoken}.
     * @param ctx the parse tree
     */
    fn enter_maptoken(&mut self, _ctx: &MaptokenContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#maptoken}.
     * @param ctx the parse tree
     */
    fn exit_maptoken(&mut self, _ctx: &MaptokenContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#arguments}.
     * @param ctx the parse tree
     */
    fn enter_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#arguments}.
     * @param ctx the parse tree
     */
    fn exit_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#argument}.
     * @param ctx the parse tree
     */
    fn enter_argument(&mut self, _ctx: &ArgumentContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#argument}.
     * @param ctx the parse tree
     */
    fn exit_argument(&mut self, _ctx: &ArgumentContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#lambda}.
     * @param ctx the parse tree
     */
    fn enter_lambda(&mut self, _ctx: &LambdaContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#lambda}.
     * @param ctx the parse tree
     */
    fn exit_lambda(&mut self, _ctx: &LambdaContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link PainlessParser#lamtype}.
     * @param ctx the parse tree
     */
    fn enter_lamtype(&mut self, _ctx: &LamtypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link PainlessParser#lamtype}.
     * @param ctx the parse tree
     */
    fn exit_lamtype(&mut self, _ctx: &LamtypeContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code classfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn enter_classfuncref(&mut self, _ctx: &ClassfuncrefContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code classfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn exit_classfuncref(&mut self, _ctx: &ClassfuncrefContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code constructorfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn enter_constructorfuncref(&mut self, _ctx: &ConstructorfuncrefContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code constructorfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn exit_constructorfuncref(&mut self, _ctx: &ConstructorfuncrefContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code localfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn enter_localfuncref(&mut self, _ctx: &LocalfuncrefContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code localfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn exit_localfuncref(&mut self, _ctx: &LocalfuncrefContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : PainlessParserListener<'input> }
