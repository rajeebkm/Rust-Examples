# Flow of Control

An essential part of any programming languages are ways to modify control flow: if/else, for, and others. Let's talk about them in Rust.

## if/else

Branching with if-else is similar to other languages. Unlike many of them, the boolean condition doesn't need to be surrounded by parentheses, and each condition is followed by a block. if-else conditionals are expressions, and, all branches must return the same type.

## loop

Rust provides a loop keyword to indicate an infinite loop.

The break statement can be used to exit a loop at anytime, whereas the continue statement can be used to skip the rest of the iteration and start a new one.

### Nesting and labels

It's possible to break or continue outer loops when dealing with nested loops. In these cases, the loops must be annotated with some 'label, and the label must be passed to the break/continue statement.

### Returning from loops

One of the uses of a loop is to retry an operation until it succeeds. If the operation returns a value though, you might need to pass it to the rest of the code: put it after the break, and it will be returned by the loop expression.

## while

The while keyword can be used to run a loop while a condition is true.

Let's write the infamous FizzBuzz using a while loop.

## for loops

### for and range

The for in construct can be used to iterate through an Iterator. One of the easiest ways to create an iterator is to use the range notation a..b. This yields values from a (inclusive) to b (exclusive) in steps of one.

Let's write FizzBuzz using for instead of while.

Alternatively, a..=b can be used for a range that is inclusive on both ends. The above can be written as:

### for and iterators

The for in construct is able to interact with an Iterator in several ways. As discussed in the section on the Iterator trait, by default the for loop will apply the into_iter function to the collection. However, this is not the only means of converting collections into iterators.

into_iter, iter and iter_mut all handle the conversion of a collection into an iterator in different ways, by providing different views on the data within.

iter - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.

into_iter - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.

iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.

In the above snippets note the type of match branch, that is the key difference in the types of iteration. The difference in type then of course implies differing actions that are able to be performed.
