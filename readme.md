# Rust Enums

- Enums allows us to define a type by enumerating its possible variants.
- Each data property in enum can be termed as variant
- Structs gave us the way of grouping related data together whereas Enums will give us a way to define that the value of some variable is one of the possible set of values.
- Rust enums are more powerful compared to any other language.
- Each variant in enum can store values along with them. These values can be of any type.

## Option Enums

- The Option type encodes the very common scenario in which a value could be something or it could be nothing.
- Rust doesn’t have the null feature that many other languages have.
- Null is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.
- The problem with null values is that if you try to use a null value as a not-null value or something. But it's extremely easy to make errors.
- However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.
- So, Rust doesnot have a null value instead has this Option enum that can encode the concept of a value being present or absent.
- This enum Option<T> is defined by the rust standard library as follows. And this Option enum is included into our program scope by default.

```
enum Option<T> {
    Some(T),
    None,
}
```

- By this Option enum, type system in rust enforces that we handle the None case when a value doesnot exist and guarantee that in some case our value is present.

## Match Statement

- This match statement is similar to switch case
- This allows us to compare a value against the set of patterns and these patterns could be literals, variables, wild cards and other. Due to having expressive patterns makes thi match statement powerful. Only big thing is that we need to list down and match all the possible cases or else we can use "_" as which is similar to default if we not wanted to implement for every case.
- This match expression is very useful for enums.
- We can use the match statement along with enums to do certain operations based on the value by matching it with one of the cases we list down in the match expression.

## If let syntax
- Incase of match expression, we need to mention all the possible cases or else we need to write default case which is verbose.
- To overcome that, we can use "if let" syntax. In the if let syntax, instead of matching everything we can only care about the case we need.
- But it is a little bit confusing syntax.
