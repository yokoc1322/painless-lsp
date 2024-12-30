use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Serialize)]
pub enum TreeNode<LeafRule, BranchRule>
where
    LeafRule: Debug + PartialEq + Serialize,
    BranchRule: Debug + PartialEq + Serialize,
{
    Leaf(Leaf<LeafRule>),
    Branch(Branch<LeafRule, BranchRule>),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Leaf<LeafRule>
where
    LeafRule: Debug + PartialEq + Serialize,
{
    pub rule: LeafRule,
    pub column_start: usize,
    pub column_end: usize,
    pub line: usize,
    //
    pub token: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Branch<LeafRule, BranchRule>
where
    LeafRule: Debug + PartialEq + Serialize,
    BranchRule: Debug + PartialEq + Serialize,
{
    pub rule: BranchRule,
    pub text_start: usize,
    pub text_end: usize,
    //
    pub text: String,
    pub children: Vec<TreeNode<LeafRule, BranchRule>>,
}
