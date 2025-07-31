# XQuery and XPath Data Model 3.1

https://www.w3.org/TR/xpath-datamodel-31

# 1 Introduction

# 2 Concepts

## 2.1 Terminology

### 2.1.1 Types adopted from XML Schema

## 2.2 Notation

## 2.3 Node identity

- [x] Node identity

## 2.4 Document order

- [x] Document order

## 2.5 Sequences

- [x] Sequences

## 2.6 Namespace Names

- [ ] Implementations _may_ reject character strings that are not valid URIs or IRIs

## 2.7 Schema Information

### 2.7.1 Representation of Types

### 2.7.2 Predefined types

- [x] `xs:untyped`

- [x] `xs:untypedAtomic`

- [x] `xs:anyAtomicType`

- [x] `xs:dayTimeDuration`

- [x] `xs:yearMonthDuration`

Primitive data types are defined in section 3.2

### 2.7.3 XML and XSD Versions

### 2.7.4 Type System

Some types are implemented, but a lot isn't yet.

### 2.7.5 Atomic Values

- [x] Atomic values

### 2.7.6 String Values

- [x] String values

### 2.7.7 Negative zero

### 2.8 Other items

### 2.8.1 Functions

- [x] Functions

### 2.8.2 Map items

- [x] Map items

### 2.8.3 Array items

- [x] Array items

### 2.8.3.1 `array-size` accessor

### 2.8.3.2 `array-get` accessor

# 3 Data Model Construction

## 3.1 Direct construction

## 3.23 Construction from asn Infoset

The infoset in this case is handled by the Xot library.

- [ ] External parsed entities fully expanded. _Xot doesn't do that yet_

- [x] Xot provides all properties identified as required

### 3.3 Construction from a PSVI

No schema information is known, so a basic PSVI is constructed.

### 3.3.1 Mapping PSVI additions to node properties

#### 3.3.1.1 Element and attribute node types

#### 3.3.12 Typed value determination

- [x] all types treated as `xs:untypedAtomic`

This is what we can do without further schema information.

#### 3.3.1.3 Relationship btween typed-value and string-value

#### 3.3.14 pattern facets

### 3.3.2 dates and times

- [x] dates and times

### 3.3.3 QNames and NOTATIONS

- [x] QNames

- [ ] Notations

# 4 Infoset Mapping

# 5 Accessors

# 5.1 attributes accessor

Lots of details here, not yet elaborated. A lot of this we do access
through Xot.

# 6 Nodes

Similarly lots of details here, and again we use Xot to access this
information.
