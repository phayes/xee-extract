use chumsky::prelude::SimpleSpan;
use ibig::IBig;
use ordered_float::OrderedFloat;
use rust_decimal::prelude::*;

pub use xee_xpath_type::ast::*;

pub use crate::operator::BinaryOperator;
use crate::span::{Spanned, WithSpan};
pub use crate::Name;

pub type Span = SimpleSpan;

pub type ExprSingleS = Spanned<ExprSingle>;
pub type PrimaryExprS = Spanned<PrimaryExpr>;
pub type StepExprS = Spanned<StepExpr>;
pub type ExprS = Spanned<Expr>;
pub type ExprOrEmpty = Option<Expr>;
pub type ExprOrEmptyS = Spanned<ExprOrEmpty>;
pub type NameS = Spanned<Name>;

impl WithSpan for ExprSingle {}
impl WithSpan for PrimaryExpr {}
impl WithSpan for StepExpr {}
impl WithSpan for Expr {}
impl WithSpan for Name {}
impl WithSpan for ExprOrEmpty {}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Expr(pub Vec<ExprSingleS>);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct XPath(pub ExprS);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ExprSingle {
    // a path expression
    Path(PathExpr),
    // something applied to a path expression
    Apply(ApplyExpr),
    // combine two path expressions
    Let(LetExpr),
    If(IfExpr),
    Binary(BinaryExpr),
    For(ForExpr),
    Quantified(QuantifiedExpr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForExpr {
    pub var_name: NameS,
    pub var_expr: Box<ExprSingleS>,
    pub return_expr: Box<ExprSingleS>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct QuantifiedExpr {
    pub quantifier: Quantifier,
    pub var_name: NameS,
    pub var_expr: Box<ExprSingleS>,
    pub satisfies_expr: Box<ExprSingleS>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LetExpr {
    pub var_name: NameS,
    pub var_expr: Box<ExprSingleS>,
    pub return_expr: Box<ExprSingleS>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IfExpr {
    pub condition: ExprS,
    pub then: Box<ExprSingleS>,
    pub else_: Box<ExprSingleS>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Quantifier {
    Some,
    Every,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PrimaryExpr {
    Literal(Literal),
    VarRef(Name),
    Expr(ExprOrEmptyS),
    ContextItem,
    FunctionCall(FunctionCall),
    NamedFunctionRef(NamedFunctionRef),
    InlineFunction(InlineFunction),
    MapConstructor(MapConstructor),
    ArrayConstructor(ArrayConstructor),
    UnaryLookup(KeySpecifier),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum KeySpecifier {
    NcName(String),
    Integer(IBig),
    Expr(ExprOrEmptyS),
    Star,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BinaryExpr {
    pub operator: BinaryOperator,
    pub left: PathExpr,
    pub right: PathExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ApplyExpr {
    pub path_expr: PathExpr,
    pub operator: ApplyOperator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ApplyOperator {
    SimpleMap(Vec<PathExpr>),
    Unary(Vec<UnaryOperator>),
    Cast(SingleType),
    Castable(SingleType),
    Treat(SequenceType),
    InstanceOf(SequenceType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum UnaryOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SingleType {
    pub name: NameS,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MapConstructor {
    pub entries: Vec<MapConstructorEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MapConstructorEntry {
    pub key: ExprSingleS,
    pub value: ExprSingleS,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ArrayConstructor {
    Square(ExprS),
    Curly(ExprOrEmptyS),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Literal {
    Decimal(Decimal),
    Integer(IBig),
    Double(OrderedFloat<f64>),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FunctionCall {
    pub name: NameS,
    pub arguments: Vec<ExprSingleS>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct NamedFunctionRef {
    pub name: NameS,
    pub arity: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct InlineFunction {
    pub params: Vec<Param>,
    pub return_type: Option<SequenceType>,
    pub body: ExprOrEmptyS,
    pub wrapper: bool,
}

// a function signature as described by:
// https://www.w3.org/TR/xpath-functions-31/#func-signatures
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Signature {
    pub name: NameS,
    pub params: Vec<SignatureParam>,
    pub return_type: SequenceType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Param {
    pub name: Name,
    pub type_: Option<SequenceType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SignatureParam {
    pub name: Name,
    pub type_: SequenceType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Postfix {
    // vec contains at least 1 element
    Predicate(ExprS),
    ArgumentList(Vec<ExprSingleS>),
    Lookup(KeySpecifier),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PathExpr {
    pub steps: Vec<StepExprS>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum StepExpr {
    PrimaryExpr(PrimaryExprS),
    PostfixExpr {
        primary: PrimaryExprS,
        postfixes: Vec<Postfix>,
    },
    AxisStep(AxisStep),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AxisStep {
    pub axis: Axis,
    pub node_test: NodeTest,
    pub predicates: Vec<ExprS>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Axis {
    Ancestor,
    AncestorOrSelf,
    Attribute,
    Child,
    Descendant,
    DescendantOrSelf,
    Following,
    FollowingSibling,
    Namespace,
    Parent,
    Preceding,
    PrecedingSibling,
    Self_,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum NodeTest {
    KindTest(KindTest),
    NameTest(NameTest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum NameTest {
    Name(NameS),
    Star,
    LocalName(String),
    Namespace(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum EQName {
    QName(QName),
    URIQualifiedName(URIQualifiedName),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum QName {
    PrefixedName(PrefixedName),
    UnprefixedName(UnprefixedName),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PrefixedName {
    pub prefix: String,
    pub local_part: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UnprefixedName {
    pub local_part: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct URIQualifiedName {
    pub uri: String,
    pub local_part: String,
}
