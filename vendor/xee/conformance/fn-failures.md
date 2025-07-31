# fn test failures that we cannot explain due to lack of implementation

## fn/data.xml 

### K2-DataFunc-6

I think this test failure has something to do with the lack of schema support.

## fn:distinct-values

### fn-distinct-values-mixed-args-005

I'm not sure whether this test failure is correct. We do get a 0 and a 1,
just that the zero is a float. Needs more analysis to figure out whether
the test isn't accurate or whether the implementation has problems.

## fn:doc

### fn-doc-37

Could be due to missing implementation of `id` function and any of not giving
good feedback.

### K2-SeqDocFunc-3

This fails with a XPTY0004. Why? Is it because untypedAtomic is not allowed as an argument?

## fn:lower-case

### fn-lower-case-19

There's a difference between the value 1011 (greek yot) that we get and a 895
(greek and coptic) that it expects. Everything else is correct. I expect this
is due to unicode table differences, but who knows.

Here's some debugging code that can be installed in assert permutation in
the test runner to help debug this:

```
    for (a, b) in sequence
        .items()
        .unwrap()
        .zip(expected_sequence.items().unwrap())
    {
        if a != b {
            println!("WHOAH a: {:?} b: {:?}", a, b);
        } else {
            println!("a: {:?} b: {:?}", a, b);
        }
    }
```

## fn:sort

### fn-sort-17

We get back $b, $a, $b, $a

In the assertions, it expects entry 1 and 2 to be $a (with 1), and entry
3 and 4 to be $b (with 2), so $a, $a, $b, $b. Seems to be a problem with NaN
in array sorting.

### fn-sort-23

This tests doesn't return true OR false, so it must be returning something else.

Probably a missing trace implementation?

### fn-sort-spec-6

No parse-xml implementation.

## fn:substring

### fn-substring-22

This gives back a runtime error, why? It appears like substring isn't even
being called, so what gives?

## fn:parse-xml-fragment

### parse-xml-fragment-010

The problem is with the processing-instruction rule:

```
parse-xml-fragment("<a/><!--comment--><?PI?><b/>")/node()[3][self::processing-instruction(PI)]
```

is not matching, and it should.

## parse-xml-fragment-021

This appears to be a type error because we're passing the result of
parse-xml-fragment, a docuemnt node, into string-to-codepoints. This should
presumably do a cast to the string value, but this isn't happening.

Why it doesn't happen is unclear; perhaps the function conversion rules behave
differently from the macro-based built-in functions than with inline functions?
