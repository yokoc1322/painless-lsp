#![allow(nonstandard_style)]
// Generated from PainlessParser.g4 by ANTLR 4.8
use super::painlessparser::*;
use antlr_rust::tree::{ParseTreeVisitor, ParseTreeVisitorCompat};

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link PainlessParser}.
 */
pub trait PainlessParserVisitor<'input>:
    ParseTreeVisitor<'input, PainlessParserContextType>
{
    /**
     * Visit a parse tree produced by {@link PainlessParser#source}.
     * @param ctx the parse tree
     */
    fn visit_source(&mut self, ctx: &SourceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#function}.
     * @param ctx the parse tree
     */
    fn visit_function(&mut self, ctx: &FunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#parameters}.
     * @param ctx the parse tree
     */
    fn visit_parameters(&mut self, ctx: &ParametersContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#statement}.
     * @param ctx the parse tree
     */
    fn visit_statement(&mut self, ctx: &StatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code if}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_if(&mut self, ctx: &IfContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code while}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_while(&mut self, ctx: &WhileContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code for}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_for(&mut self, ctx: &ForContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code each}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_each(&mut self, ctx: &EachContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ineach}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_ineach(&mut self, ctx: &IneachContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code try}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_try(&mut self, ctx: &TryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code do}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_do(&mut self, ctx: &DoContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code decl}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_decl(&mut self, ctx: &DeclContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code continue}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_continue(&mut self, ctx: &ContinueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code break}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_break(&mut self, ctx: &BreakContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code return}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_return(&mut self, ctx: &ReturnContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code throw}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_throw(&mut self, ctx: &ThrowContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code expr}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_expr(&mut self, ctx: &ExprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#trailer}.
     * @param ctx the parse tree
     */
    fn visit_trailer(&mut self, ctx: &TrailerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#block}.
     * @param ctx the parse tree
     */
    fn visit_block(&mut self, ctx: &BlockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#empty}.
     * @param ctx the parse tree
     */
    fn visit_empty(&mut self, ctx: &EmptyContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#initializer}.
     * @param ctx the parse tree
     */
    fn visit_initializer(&mut self, ctx: &InitializerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#afterthought}.
     * @param ctx the parse tree
     */
    fn visit_afterthought(&mut self, ctx: &AfterthoughtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#declaration}.
     * @param ctx the parse tree
     */
    fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#decltype}.
     * @param ctx the parse tree
     */
    fn visit_decltype(&mut self, ctx: &DecltypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#type_}.
     * @param ctx the parse tree
     */
    fn visit_type_(&mut self, ctx: &Type_Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#declvar}.
     * @param ctx the parse tree
     */
    fn visit_declvar(&mut self, ctx: &DeclvarContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#trap}.
     * @param ctx the parse tree
     */
    fn visit_trap(&mut self, ctx: &TrapContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code single}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_single(&mut self, ctx: &SingleContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code comp}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_comp(&mut self, ctx: &CompContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code bool}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_bool(&mut self, ctx: &BoolContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code binary}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_binary(&mut self, ctx: &BinaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code elvis}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_elvis(&mut self, ctx: &ElvisContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code instanceof}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_instanceof(&mut self, ctx: &InstanceofContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code nonconditional}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_nonconditional(&mut self, ctx: &NonconditionalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code conditional}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_conditional(&mut self, ctx: &ConditionalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code assignment}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code pre}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn visit_pre(&mut self, ctx: &PreContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code addsub}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn visit_addsub(&mut self, ctx: &AddsubContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code notaddsub}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn visit_notaddsub(&mut self, ctx: &NotaddsubContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code read}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn visit_read(&mut self, ctx: &ReadContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code post}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn visit_post(&mut self, ctx: &PostContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code not}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn visit_not(&mut self, ctx: &NotContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code cast}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn visit_cast(&mut self, ctx: &CastContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code primordefcast}
     * labeled alternative in {@link PainlessParser#castexpression}.
     * @param ctx the parse tree
     */
    fn visit_primordefcast(&mut self, ctx: &PrimordefcastContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code refcast}
     * labeled alternative in {@link PainlessParser#castexpression}.
     * @param ctx the parse tree
     */
    fn visit_refcast(&mut self, ctx: &RefcastContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#primordefcasttype}.
     * @param ctx the parse tree
     */
    fn visit_primordefcasttype(&mut self, ctx: &PrimordefcasttypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#refcasttype}.
     * @param ctx the parse tree
     */
    fn visit_refcasttype(&mut self, ctx: &RefcasttypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code dynamic}
     * labeled alternative in {@link PainlessParser#chain}.
     * @param ctx the parse tree
     */
    fn visit_dynamic(&mut self, ctx: &DynamicContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code newarray}
     * labeled alternative in {@link PainlessParser#chain}.
     * @param ctx the parse tree
     */
    fn visit_newarray(&mut self, ctx: &NewarrayContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code precedence}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_precedence(&mut self, ctx: &PrecedenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code numeric}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_numeric(&mut self, ctx: &NumericContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code true}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_true(&mut self, ctx: &TrueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code false}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_false(&mut self, ctx: &FalseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code null}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_null(&mut self, ctx: &NullContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code string}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_string(&mut self, ctx: &StringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code regex}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_regex(&mut self, ctx: &RegexContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code listinit}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_listinit(&mut self, ctx: &ListinitContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code mapinit}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_mapinit(&mut self, ctx: &MapinitContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code variable}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code calllocal}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_calllocal(&mut self, ctx: &CalllocalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code newobject}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_newobject(&mut self, ctx: &NewobjectContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#postfix}.
     * @param ctx the parse tree
     */
    fn visit_postfix(&mut self, ctx: &PostfixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#postdot}.
     * @param ctx the parse tree
     */
    fn visit_postdot(&mut self, ctx: &PostdotContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#callinvoke}.
     * @param ctx the parse tree
     */
    fn visit_callinvoke(&mut self, ctx: &CallinvokeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#fieldaccess}.
     * @param ctx the parse tree
     */
    fn visit_fieldaccess(&mut self, ctx: &FieldaccessContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#braceaccess}.
     * @param ctx the parse tree
     */
    fn visit_braceaccess(&mut self, ctx: &BraceaccessContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code newstandardarray}
     * labeled alternative in {@link PainlessParser#arrayinitializer}.
     * @param ctx the parse tree
     */
    fn visit_newstandardarray(&mut self, ctx: &NewstandardarrayContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code newinitializedarray}
     * labeled alternative in {@link PainlessParser#arrayinitializer}.
     * @param ctx the parse tree
     */
    fn visit_newinitializedarray(&mut self, ctx: &NewinitializedarrayContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#listinitializer}.
     * @param ctx the parse tree
     */
    fn visit_listinitializer(&mut self, ctx: &ListinitializerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#mapinitializer}.
     * @param ctx the parse tree
     */
    fn visit_mapinitializer(&mut self, ctx: &MapinitializerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#maptoken}.
     * @param ctx the parse tree
     */
    fn visit_maptoken(&mut self, ctx: &MaptokenContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#arguments}.
     * @param ctx the parse tree
     */
    fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#argument}.
     * @param ctx the parse tree
     */
    fn visit_argument(&mut self, ctx: &ArgumentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#lambda}.
     * @param ctx the parse tree
     */
    fn visit_lambda(&mut self, ctx: &LambdaContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#lamtype}.
     * @param ctx the parse tree
     */
    fn visit_lamtype(&mut self, ctx: &LamtypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code classfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn visit_classfuncref(&mut self, ctx: &ClassfuncrefContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code constructorfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn visit_constructorfuncref(&mut self, ctx: &ConstructorfuncrefContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code localfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn visit_localfuncref(&mut self, ctx: &LocalfuncrefContext<'input>) {
        self.visit_children(ctx)
    }
}

pub trait PainlessParserVisitorCompat<'input>:
    ParseTreeVisitorCompat<'input, Node = PainlessParserContextType>
{
    /**
     * Visit a parse tree produced by {@link PainlessParser#source}.
     * @param ctx the parse tree
     */
    fn visit_source(&mut self, ctx: &SourceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#function}.
     * @param ctx the parse tree
     */
    fn visit_function(&mut self, ctx: &FunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#parameters}.
     * @param ctx the parse tree
     */
    fn visit_parameters(&mut self, ctx: &ParametersContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#statement}.
     * @param ctx the parse tree
     */
    fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code if}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_if(&mut self, ctx: &IfContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code while}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_while(&mut self, ctx: &WhileContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code for}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_for(&mut self, ctx: &ForContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code each}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_each(&mut self, ctx: &EachContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ineach}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_ineach(&mut self, ctx: &IneachContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code try}
     * labeled alternative in {@link PainlessParser#rstatement}.
     * @param ctx the parse tree
     */
    fn visit_try(&mut self, ctx: &TryContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code do}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_do(&mut self, ctx: &DoContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code decl}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_decl(&mut self, ctx: &DeclContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code continue}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_continue(&mut self, ctx: &ContinueContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code break}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_break(&mut self, ctx: &BreakContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code return}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_return(&mut self, ctx: &ReturnContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code throw}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_throw(&mut self, ctx: &ThrowContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code expr}
     * labeled alternative in {@link PainlessParser#dstatement}.
     * @param ctx the parse tree
     */
    fn visit_expr(&mut self, ctx: &ExprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#trailer}.
     * @param ctx the parse tree
     */
    fn visit_trailer(&mut self, ctx: &TrailerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#block}.
     * @param ctx the parse tree
     */
    fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#empty}.
     * @param ctx the parse tree
     */
    fn visit_empty(&mut self, ctx: &EmptyContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#initializer}.
     * @param ctx the parse tree
     */
    fn visit_initializer(&mut self, ctx: &InitializerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#afterthought}.
     * @param ctx the parse tree
     */
    fn visit_afterthought(&mut self, ctx: &AfterthoughtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#declaration}.
     * @param ctx the parse tree
     */
    fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#decltype}.
     * @param ctx the parse tree
     */
    fn visit_decltype(&mut self, ctx: &DecltypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#type_}.
     * @param ctx the parse tree
     */
    fn visit_type_(&mut self, ctx: &Type_Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#declvar}.
     * @param ctx the parse tree
     */
    fn visit_declvar(&mut self, ctx: &DeclvarContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#trap}.
     * @param ctx the parse tree
     */
    fn visit_trap(&mut self, ctx: &TrapContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code single}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_single(&mut self, ctx: &SingleContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code comp}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_comp(&mut self, ctx: &CompContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code bool}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_bool(&mut self, ctx: &BoolContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code binary}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_binary(&mut self, ctx: &BinaryContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code elvis}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_elvis(&mut self, ctx: &ElvisContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code instanceof}
     * labeled alternative in {@link PainlessParser#noncondexpression}.
     * @param ctx the parse tree
     */
    fn visit_instanceof(&mut self, ctx: &InstanceofContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code nonconditional}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_nonconditional(&mut self, ctx: &NonconditionalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code conditional}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_conditional(&mut self, ctx: &ConditionalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code assignment}
     * labeled alternative in {@link PainlessParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code pre}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn visit_pre(&mut self, ctx: &PreContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code addsub}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn visit_addsub(&mut self, ctx: &AddsubContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code notaddsub}
     * labeled alternative in {@link PainlessParser#unary}.
     * @param ctx the parse tree
     */
    fn visit_notaddsub(&mut self, ctx: &NotaddsubContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code read}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn visit_read(&mut self, ctx: &ReadContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code post}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn visit_post(&mut self, ctx: &PostContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code not}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn visit_not(&mut self, ctx: &NotContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code cast}
     * labeled alternative in {@link PainlessParser#unarynotaddsub}.
     * @param ctx the parse tree
     */
    fn visit_cast(&mut self, ctx: &CastContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code primordefcast}
     * labeled alternative in {@link PainlessParser#castexpression}.
     * @param ctx the parse tree
     */
    fn visit_primordefcast(&mut self, ctx: &PrimordefcastContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code refcast}
     * labeled alternative in {@link PainlessParser#castexpression}.
     * @param ctx the parse tree
     */
    fn visit_refcast(&mut self, ctx: &RefcastContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#primordefcasttype}.
     * @param ctx the parse tree
     */
    fn visit_primordefcasttype(&mut self, ctx: &PrimordefcasttypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#refcasttype}.
     * @param ctx the parse tree
     */
    fn visit_refcasttype(&mut self, ctx: &RefcasttypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code dynamic}
     * labeled alternative in {@link PainlessParser#chain}.
     * @param ctx the parse tree
     */
    fn visit_dynamic(&mut self, ctx: &DynamicContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code newarray}
     * labeled alternative in {@link PainlessParser#chain}.
     * @param ctx the parse tree
     */
    fn visit_newarray(&mut self, ctx: &NewarrayContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code precedence}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_precedence(&mut self, ctx: &PrecedenceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code numeric}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_numeric(&mut self, ctx: &NumericContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code true}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_true(&mut self, ctx: &TrueContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code false}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_false(&mut self, ctx: &FalseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code null}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_null(&mut self, ctx: &NullContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code string}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_string(&mut self, ctx: &StringContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code regex}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_regex(&mut self, ctx: &RegexContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code listinit}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_listinit(&mut self, ctx: &ListinitContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code mapinit}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_mapinit(&mut self, ctx: &MapinitContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code variable}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code calllocal}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_calllocal(&mut self, ctx: &CalllocalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code newobject}
     * labeled alternative in {@link PainlessParser#primary}.
     * @param ctx the parse tree
     */
    fn visit_newobject(&mut self, ctx: &NewobjectContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#postfix}.
     * @param ctx the parse tree
     */
    fn visit_postfix(&mut self, ctx: &PostfixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#postdot}.
     * @param ctx the parse tree
     */
    fn visit_postdot(&mut self, ctx: &PostdotContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#callinvoke}.
     * @param ctx the parse tree
     */
    fn visit_callinvoke(&mut self, ctx: &CallinvokeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#fieldaccess}.
     * @param ctx the parse tree
     */
    fn visit_fieldaccess(&mut self, ctx: &FieldaccessContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#braceaccess}.
     * @param ctx the parse tree
     */
    fn visit_braceaccess(&mut self, ctx: &BraceaccessContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code newstandardarray}
     * labeled alternative in {@link PainlessParser#arrayinitializer}.
     * @param ctx the parse tree
     */
    fn visit_newstandardarray(&mut self, ctx: &NewstandardarrayContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code newinitializedarray}
     * labeled alternative in {@link PainlessParser#arrayinitializer}.
     * @param ctx the parse tree
     */
    fn visit_newinitializedarray(
        &mut self,
        ctx: &NewinitializedarrayContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#listinitializer}.
     * @param ctx the parse tree
     */
    fn visit_listinitializer(&mut self, ctx: &ListinitializerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#mapinitializer}.
     * @param ctx the parse tree
     */
    fn visit_mapinitializer(&mut self, ctx: &MapinitializerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#maptoken}.
     * @param ctx the parse tree
     */
    fn visit_maptoken(&mut self, ctx: &MaptokenContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#arguments}.
     * @param ctx the parse tree
     */
    fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#argument}.
     * @param ctx the parse tree
     */
    fn visit_argument(&mut self, ctx: &ArgumentContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#lambda}.
     * @param ctx the parse tree
     */
    fn visit_lambda(&mut self, ctx: &LambdaContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link PainlessParser#lamtype}.
     * @param ctx the parse tree
     */
    fn visit_lamtype(&mut self, ctx: &LamtypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code classfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn visit_classfuncref(&mut self, ctx: &ClassfuncrefContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code constructorfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn visit_constructorfuncref(
        &mut self,
        ctx: &ConstructorfuncrefContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code localfuncref}
     * labeled alternative in {@link PainlessParser#funcref}.
     * @param ctx the parse tree
     */
    fn visit_localfuncref(&mut self, ctx: &LocalfuncrefContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }
}

impl<'input, T> PainlessParserVisitor<'input> for T
where
    T: PainlessParserVisitorCompat<'input>,
{
    fn visit_source(&mut self, ctx: &SourceContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_function(&mut self, ctx: &FunctionContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_function(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_parameters(&mut self, ctx: &ParametersContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_parameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_if(&mut self, ctx: &IfContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_if(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_while(&mut self, ctx: &WhileContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_while(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_for(&mut self, ctx: &ForContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_for(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_each(&mut self, ctx: &EachContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_each(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ineach(&mut self, ctx: &IneachContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_ineach(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_try(&mut self, ctx: &TryContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_try(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_do(&mut self, ctx: &DoContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_do(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_decl(&mut self, ctx: &DeclContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_decl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_continue(&mut self, ctx: &ContinueContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_continue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_break(&mut self, ctx: &BreakContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_break(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_return(&mut self, ctx: &ReturnContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_return(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_throw(&mut self, ctx: &ThrowContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_throw(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_expr(&mut self, ctx: &ExprContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trailer(&mut self, ctx: &TrailerContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_trailer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_block(&mut self, ctx: &BlockContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_empty(&mut self, ctx: &EmptyContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_empty(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_initializer(&mut self, ctx: &InitializerContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_initializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_afterthought(&mut self, ctx: &AfterthoughtContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_afterthought(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_declaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_decltype(&mut self, ctx: &DecltypeContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_decltype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_type_(&mut self, ctx: &Type_Context<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_type_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_declvar(&mut self, ctx: &DeclvarContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_declvar(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trap(&mut self, ctx: &TrapContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_trap(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_single(&mut self, ctx: &SingleContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_single(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_comp(&mut self, ctx: &CompContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_comp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bool(&mut self, ctx: &BoolContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_bool(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_binary(&mut self, ctx: &BinaryContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_binary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elvis(&mut self, ctx: &ElvisContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_elvis(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_instanceof(&mut self, ctx: &InstanceofContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_instanceof(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nonconditional(&mut self, ctx: &NonconditionalContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_nonconditional(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_conditional(&mut self, ctx: &ConditionalContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_conditional(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pre(&mut self, ctx: &PreContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_pre(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_addsub(&mut self, ctx: &AddsubContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_addsub(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_notaddsub(&mut self, ctx: &NotaddsubContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_notaddsub(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_read(&mut self, ctx: &ReadContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_read(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_post(&mut self, ctx: &PostContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_post(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_not(&mut self, ctx: &NotContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_not(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_cast(&mut self, ctx: &CastContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_cast(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_primordefcast(&mut self, ctx: &PrimordefcastContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_primordefcast(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_refcast(&mut self, ctx: &RefcastContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_refcast(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_primordefcasttype(&mut self, ctx: &PrimordefcasttypeContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_primordefcasttype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_refcasttype(&mut self, ctx: &RefcasttypeContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_refcasttype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dynamic(&mut self, ctx: &DynamicContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_dynamic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_newarray(&mut self, ctx: &NewarrayContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_newarray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_precedence(&mut self, ctx: &PrecedenceContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_precedence(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numeric(&mut self, ctx: &NumericContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_numeric(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_true(&mut self, ctx: &TrueContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_true(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_false(&mut self, ctx: &FalseContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_false(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_null(&mut self, ctx: &NullContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_null(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_string(&mut self, ctx: &StringContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_string(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_regex(&mut self, ctx: &RegexContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_regex(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listinit(&mut self, ctx: &ListinitContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_listinit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_mapinit(&mut self, ctx: &MapinitContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_mapinit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_calllocal(&mut self, ctx: &CalllocalContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_calllocal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_newobject(&mut self, ctx: &NewobjectContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_newobject(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_postfix(&mut self, ctx: &PostfixContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_postfix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_postdot(&mut self, ctx: &PostdotContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_postdot(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_callinvoke(&mut self, ctx: &CallinvokeContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_callinvoke(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fieldaccess(&mut self, ctx: &FieldaccessContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_fieldaccess(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_braceaccess(&mut self, ctx: &BraceaccessContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_braceaccess(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_newstandardarray(&mut self, ctx: &NewstandardarrayContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_newstandardarray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_newinitializedarray(&mut self, ctx: &NewinitializedarrayContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_newinitializedarray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listinitializer(&mut self, ctx: &ListinitializerContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_listinitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_mapinitializer(&mut self, ctx: &MapinitializerContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_mapinitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_maptoken(&mut self, ctx: &MaptokenContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_maptoken(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_argument(&mut self, ctx: &ArgumentContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_argument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_lambda(&mut self, ctx: &LambdaContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_lambda(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_lamtype(&mut self, ctx: &LamtypeContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_lamtype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_classfuncref(&mut self, ctx: &ClassfuncrefContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_classfuncref(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_constructorfuncref(&mut self, ctx: &ConstructorfuncrefContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_constructorfuncref(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_localfuncref(&mut self, ctx: &LocalfuncrefContext<'input>) {
        let result = <Self as PainlessParserVisitorCompat>::visit_localfuncref(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }
}
