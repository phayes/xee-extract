# XML Path Language (XPath) 3.1

https://www.w3.org/TR/xpath-31/

# 1 Introduction

# 2 Basics

- [x] EQName - _but not in all cases handling namespaces properly yet_

## 2.1 Expression Context

### 2.1.1 Static Context

- [ ] XPath 1.0 compatibility mode. _No implementation plans_

- [x] Statically known namespaces

- [x] Default element/type namespace

- [x] Default function namespace

- [ ] In-scope schema definitions. _No schema support at all yet_

- [x] In-scope variables

- [ ] Context item static type. _No static typing at all yet_

- [x] Statically known function signatures. _Only built-in functions_

- [x] Statically known collations

- [x] Default collation

- [ ] Statically known documents.

- [ ] Statically known collections.

- [ ] Statically known default collection type

- [ ] Statically known decimal formats

### 2.1.2 Dynamic Context

- [x] Context item

- [x] Context position

- [x] Context size

- [x] Variable values

- [x] Named functions. _Not sure how this is dynamic?_

- [x] Current dateTime

- [x] Implicit timezone

- [x] Default language

- [ ] Default calendar

- [ ] Default place

- [ ] Available documents

- [ ] Available text resources

- [ ] Available collections

- [ ] Default collection

- [ ] Available URI collections

- [ ] Default URI collection

- [ ] Environment variables

## 2.2. Processing Model

- [ ] Type annotation in XDM. _No schema support yet, everything untypedAtomic_

### 2.2.2 Schema Import Processing

### 2.2.3 Expression Processing

#### 2.2.3.1 Static Analysis Phase

- [x] Static error if name is unknown.

- [ ] Normalization of operation tree to make atomization and effective boolean value extraction explicit. _We don't do it this way, but we do extract this information. We may consider making this explicit in the IR if it helps with static analysis_.

- [ ] Static typing feature. _No static typing yet. Would be nice, but not required for conformance. Saxon doesn't have it and it would introduce compatibility problems if implemented._

#### 2.2.3.2 Dynamic Evaluation Phase

- [x] Raise type error if operand has dynamic type that is incorrect.

### 2.2.4 Consistency Constraints

## 2.3 Error Handling

### 2.3.1 Kinds of errors

- [x] Static errors

- [x] Dynamic errors

### 2.3.2 Identifying and Reporting Errors

- [x] Unique error codes

### 2.3.3 Handling Dynamic Errors

- [x] Dynamic errors are raised

- [ ] error function. _Not complete yet_

### 2.3.4 Errors and Optimization

_No optimization yet_

## 2.4 Concepts

### 2.4.1 Document Order

- [x] Nodes have document order

### 2.4.2 Atomization

- [x] Atomic value is returned

- [x] Typed value of node is returned _Only untypedAtomic without schema support_

- [x] Function raises error _May be wrong error code_

- [x] If item is array, atomize each item

Atomization is applied to:

- [x] Arithmetic expressions

- [x] Comparison expressions

- [x] Inline function call arguments

- [x] Inline function call returns

- [x] Built-in function calls

- [x] Cast expressions

### 2.4.3 Effective Boolean Value

- [x] Empty sequence returns false

- [x] Sequence with first item node, true

- [x] Singleton of type `xs:boolean` returns boolean

- [x] Singleton derived from `xs:boolean` returns boolean

- [x] Singleton of type `xs:string`, `xs:untypedAtomic` returns true if length > 0

- [x] Singleton of `xs:anyURI` is true if length > 0

- [x] Singleton of numeric type, returns true if not zero

- [x] Singleton of numeric type returns true if not NaN

- [x] Type error in other cases

Used in:

- [x] Logical expressions

- [x] `fn:not`

- [x] In certain predicates such as `a[b]`

- [x] Conditional expressions `if`

- [x] Quantified expressions

- [ ] XPath 1.0 mode for general comparisons

### 2.4.4 Input sources

- [ ] Input sources support via variety of functions

- [x] Input via variable or context item

### 2.4.5 URI Literals

- [x] Verification of valid URI in BracedURILiteral

- [x] Whitespace normalization of URI literal

### 2.4.6 Resolving a Relative URI Reference

- [x] Resolving a relative URI reference

## 2.5 Types

- [ ] Schema types

- [x] Generalized atomic type

- [ ] pure union type

### 2.5.1 Predefined Schema Types

- [x] `xs:untyped`

- [x] `xs:untypedAtomic` _But not yet in inline function definitions_

- [x] xs:dayTimeDuration

- [x] xs:yearMonthDuration

- [x] `xs:anyAtomicType` _But not yet in inline function definitions_

- [x] xs:error

### 2.5.2 Namespace-sensitive types

- [x] `xs:QName`

- [ ] `xs:NOTATION`

### 2.5.3 Typed Value and String Value

- [x] Untyped nodes are xs:untypedAtomic

- [ ] Typed value for node besides xs:untypedAtomic

- [x] String value for node

- [ ] Detailed rules of getting typed value for nodes as described here

### 2.5.4 SequenceType Syntax

- [x] Parsing sequence types

### 2.5.5 SequenceType Matching

derives-from pseudo function

- [x] error if ET not present in in scope-schema definitions

- [x] AT is ET

- [x] ET is base type of AT

- [x] ET is a pure union type of which AT is a member type

- [x] Recursion via intermediate type MT

#### 2.5.5.1 Matching a SequenceType and a Value

- [x] no postfix, xs:int

- [x] optional postfix, xs:int?

- [x] 0 or more items postfix, xs:int\*

- [x] 1 or more items postfix, xs:int+

#### 2.5.5.2 Matching an ItemType and an Item

- [x] Matching with EQName

- [x] `item()`

- [x] `node()`

- [x] `text()`

- [x] `processing-instruction()`

- [x] `processing-instruction(N)`

- [x] `comment()`

- [ ] `namespace-node()`

- [x] `document-node()`

- [x] item type that is a test

- [x] `map(K, V)`

- [x] `map(*)`

- [x] `array(T)`

- [x] `array(*)`

#### 2.5.5.3 Element Test

- [x] Use in type system

- [x] Use in NodeTest

- [x] `element()` and `element(*)`

- [x] `element(ElementName)`

- [x] `element(ElementName, TypeName)`

- [x] `element(ElementName, TypeName?)`

- [x] `element(*, TypeName)`

- [x] `element(*, TypeName?)`

#### 2.5.5.4 Schema Element test

- [ ] Schema element test

#### 2.5.5.5 Attribute test

- [x] `attribute()` and `attribute(*)`

- [x] `attribute(AttributeName)`

- [x] `attribute(AttributeName, TypeName)`

- [x] `attribute(*, TypeName)`

#### 2.5.5.6 Schema attribute test

- [ ] Schema attribute test

#### 2.5.5.7 Function test

- [x] `function(*)`

- [x] `function()` with argument types and return value

#### 2.5.5.8 Map Test

- [x] Map test _But details for instance of still to be done_

#### 2.5.5.9 Array Test

- [x] Array test

### 2.5.6 SequenceType Subtype Relationships

#### 2.5.6.1 The judgement `subtype(A, B)`

- [x] The judgement `subtype(A, B)`

#### 2.5.6.2 The judgement `subtype-itemtype(Ai, Bi)`

Note: detailed rules of 36 items, may spell it out when implementing.

- [x] The judgement `subtype-itemtype(Ai, Bi)`

### 2.5.6 xs:error

- [x] `xs:error` type

## 2.6 Comments

- [x] Comments are parsed and ignored

# 3 Expressions

## 3.1 Primary Expressions

### 3.1.1 Literals

- [x] Integer literals

- [x] Decimal literals

- [x] Double literals

- [x] String literals

### 3.1.2 Variable references

- [x] Variable references

- [x] Variable scoping

### 3.1.3 Parenthesized Expressions

- [x] Parenthesized expressions

### 3.1.4 Context Item Expression

- [x] Context item expression `.`

### 3.1.5 Static Function Calls

- [x] Static function calls

- [x] Static function call argument type checking

- [x] Partial function application

#### 3.1.5.1 Evaluation Static and Dynamic Function Calls

- [x] Static function lookup

- [x] Dynamic function lookup

- [x] Application of function conversion rules for inline function arguments

- [x] Application of function conversion rules for built-in functions. _In as much as implemented_

- [x] Partial function application

- [x] map function

- [x] inline function evaluation

- [x] inline function evaluation conversion rules for return value

- [x] non-local variable bindings for inline functions

- [x] argument values for built-in functions

- [x] non-local variable bindings for built-in functions

- [x] static and dynamic context for built-in functions

#### 3.1.5.2 Function Conversion Rules

- [ ] XPath 1.0 compatibility mode

- [x] Atomization for built-in functions

- [x] `untypedAtomic` cast to expected function. _Casting still limited_

- [x] numeric item type promotion

- [x] `anyURI` type promotion

- [ ] `TypedFunctionTest` causes function coercion

- [x] Type error if coercion fails

#### 3.1.5.3 Function Coercion

- [x] Function coercion

### 3.1.6 Named Function References

- [x] Named function references

### 3.1.7 Inline Function Expressions

- [x] Inline function expressions

- [x] Non-local variable bindings

- [x] Type signature support

- [x] Function coercion

### 3.1.8 Enclosed Expressions

- [x] Enclosed expressions

## 3.2 Postfix expressions

### 3.2.1 Filter expressions

- [x] Filter expressions

- [x] Predicate if numeric is compared to context position

- [x] Predicate as boolean value otherwise

### 3.2.2 Dynamic Function Calls

- [x] Dynamic function calls

## 3.3 Path Expressions

- [x] `/` at beginning

- [x] `//` at beginning

- [ ] `treat as` in `/` and `//` (but only needed for static typing)

### 3.3.1 Relative Path Expressions

- [x] Relative path expressions

#### 3.3.1.1 Path operator (`/`)

- [x] Path operator `/`

### 3.3.2 Steps

#### 3.3.2.1

- [x] `child`

- [x] `descendant`

- [x] `parent`

- [x] `ancestor`

- [x] `following-sibling`

- [x] `preceding-sibling`

- [x] `following`

- [x] `preceding`

- [x] `attribute`

- [x] `self`

- [x] `descendant-or-self`

- [x] `ancestor-or-self`

- [ ] `namespace` _But not required for conformance_

#### 3.3.2.2 Node tests

- [x] Name test

- [x] Wildcard name test

- [x] Kind test (see 2.5.4 and 2.5.5)

### 3.3.3 Predicates within Steps

- [x] Predicates within steps

### 3.3.4 Unabbreviated Syntax

- [x] Unabbreviated syntax

### 3.3.5 Abbreviated Syntax

- [x] Abbreviated syntax

## 3.4. Sequence Expressions

- [x] sequence constructions

- [x] range expressions

### 3.4.2 Combining node sequences

- [x] Union

- [x] Intersection

### 3.5 Arithmetic Expressions

- [x] `+`

- [x] `-`

- [x] `*`

- [x] `div`

- [x] `idiv`

- [x] `mod`

- [x] unary `+`

- [x] unary `-`

- [ ] XPath 1.0 compatibility mode operand evaluation

- [x] Atomization during operand evaluation

## 3.6 String concatenation

- [x] String concatenation `||`

## 3.7 Comparison Expressions

### 3.7.1. Value Comparisons

- [x] Atomization

- [x] Empty sequence means empty sequence result

- [x] Atomization length > 1 means type error

- [x] `untypedAtomic` cast to string

- [x] Values are of different types: `xs:string`/`xs:anyURI`

- [x] Values are of different types: `xs:decimal`/`xs:float`

- [x] Values are of different types `xs:decimal`, `xs:float`, `xs:double`

### 3.7.2 General Comparisons

- [ ] XPath 1.0 compatibility mode

- [x] Atomization for each operand

- [x] Both `untypedAtomic` are cast to `xs:string`

- [x] `untypedAtomic` cast to `xs:double`

- [x] `untypedAtomic` cast to `xs:daytimeDuration`

- [x] `untypedAtomic` cast to `xs:yearMonthDuration`

- [x] `untypedAtomic` cast from primitive base type

### 3.7.3 Node Comparisons

- [x] `is` operator

- [x] `<<` operator

- [x] `>>` operator

## 3.8 Logical epxressions

- [x] `or`

- [x] `and`

- [ ] XPath 1.0 compatibility mode

## 3.9 For Expressions

- [x] For expressions

## 3.10 Let Expressions

- [x] Let expressions

## 3.11 Maps and Arrays

### 3.11.1 Maps

#### 3.11.1.1 Map Constructors

- [x] Map constructors

#### 3.11.1.2 Map lookup using Function Call Syntax

- [x] Map lookup using function call syntax

### 3.11.2 Arrays

#### 3.11.2.1 Array Constructors

- [x] Array constructors

#### 3.11.2.2 Array Lookup using Function Call Syntax

- [x] Array lookup using function call syntax

### 3.11.3 The Lookup Operator `?` for Maps and Arrays

#### 3.11.3.1 Unary Lookup

- [x] Unary Lookup

#### 3.11.3.2 Postfix Lookup

- [x] Postfix Lookup

## 3.12 Conditional Expressions

- [x] Conditional expressions

## 3.13 Quantified Expressions

- [x] `some`

- [x] `every`

## 3.14 Expressions on SequenceTypes

### 3.14.1 Instance of

- [x] `instance of`

### 3.14.2 Cast

- [x] `cast as`

### 3.14.3 Castable

- [x] `castable as`

### 3.14.4 Constructor functions

- [x] Constructor functions

### 3.14.5 Treat

- [ ] `treat as` _no effect without stating typing, maybe should do instance of_

## 3.15 Simple map operator (`!`)

- [x] Simple map operator

## 3.16 Arrow operator (`=>`)

- [x] Arrow operator

# A XPath 3.1 Grammar

## A.1 EBNF

- [x] Grammar (a few details remain)

### A.1.1 Notation

### A.1.2 Extra grammatical constraints

- [x] Leading-lone-slash

- [ ] xml-version

- [x] reserved-function-names

- [x] occurrence-indicators

### A.1.3 Grammar notes

- [x] parens

- [x] comments

## A.2 Lexical structure

### A.2.1 Terminal Symbols

- [x] Terminal symbols

### A.2.2 Terminal Delimination

- [x] distinguish delimiting terminal symbols from non-delimiting ones

- [x] comment and whitespace symbol separators

- [x] symbol separator between two non-delimiting terminal symbols

- [x] symbol separator after NCName and "." and "-"

- [x] symbol separator between numeric literal and "." and vice versa

### A.2.3 End of line handling

TDB

#### A.2.3.1 XML 1.0 end of line handling

#### A.2.3.2 XML 1.1 end of line handling

### A.2.4 Whitespace rules

- [x] whitespace rules

#### A.2.4.2 Explicit whitespace handling

- [x] explicit whitespace handling inside various literals

## A.3 Reserved function names

- [x] reserved function names

## A.4. Precedence order (non-normative)

- [x] Implemented grammar so should follow precedence order

# B Type Promotion and Operator Mapping

## B.1 Type Promotion

- [x] Numeric type promotion for operators

- [x] Numeric type promotion for function calls

- [ ] URI type promotion for operators

- [ ] URI type promotion for function calls

- [ ] subtype substitution, see 2.5.5.1

### B.2 Operator Mapping

- [x] Operator mapping for numeric arithmetic

- [x] Operator mapping for date time arithmetic

- [x] Complete handling of all operator mapping in big table

# C Context Components

## C.1 Static Context Components

See static context components in 2.1.1

## C.2 Dynamic Context Components

See dynamic context components in 2.1.2

# D Implementation-Defined items
