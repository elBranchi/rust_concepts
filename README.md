# Rust concepts
Collection of miscellaneous Rust programs and snippets showcasing/using Rust features.

Coments on the code reflect my understanding of the different concepts being used, in some situations my understanding might 
be incomplete so take the information with a grain of salt.

## Programs/binaries

Under src/bin there a series of programs showing different Rust features:

### basic_traits.rs

Shows the usage of some basic Rust traits and how they are implemented for custom types.

Some of the traits shown are:
* Debug: implemented automatically by compiler using derive directive, allows writing the content of a variable via {:?} formatting option
* Copy, Clone: Also implemented automatically by compiler, those traits define methods that provide a copy/clone of an original instance/variable.
* PartialEq, Eq: Also automatically implemented by compiler, they provide a way to compare instances of a type and thus allow them to be ordered.
* Iterator: trait that allows the iteration over a set of values.
* IntoIterator: Trait that allows getting an Iterator from an instance of a type.
* Default: Trait defining a method to obtain a default value for a type.
* fmt::Display: Trait allowing the formatting the contents of an instance into a string
* From: Trait allowing the conversion of an instance of a type into an instance of another type
* Add, Sub: Traits allowing the implementation of addition and subtraction operations.

[Source](src/bin/basic_traits.rs)

### iterator_use.rs

Sample basic usage of iterators over arrays and Vec types.

Topics that are explored there:

* Methods to obtain an Iterator, the method used or how the original collection is used might imply the source collection is consumed by the end of the iteration.
* Available methods for iterators, usually already implemented (new Iterators only require implementing _next_ method):
    * step_by : Iterate the collection advancing n elements at each step
    * chain: concatenate/combine an initial iterator with other collection(s)/iterator(s)
    * zip: combine the elements of 2 collections/iterators into an iterator which each element is a tuple containing one element from each initial collection.
    * partition: divide the elements of an Iterator into 2 collections based on a predicate being true or not
    * position/rposition: obtain the position of an element from "left"/start or "right"/end that fulfills a predicate
    * filter: transform an iterator into one that only return the elements that satisfy a predicate
    * collect: convert/store the resulting elements of an Iterator into a collection.


[Source](src/bin/iterator_use.rs)

### min_max.rs

Sample implementation of generic min & max functions working on slices of elements implementing PartialOrd trait


[Source](src/bin/min_max.rs)

### shared.rs

As Rust ownership model enforces certain restrictions:
* It's not possible to have a mutable and non-mutable reference to an object at the same time (with overlapping lifetimes)
* It's not possible to modify a mutable instance while a non-mutable reference to it is still "alive"

Part of those restrictions makes non-trivial using a shared value on different places. This can be alleviated on single threaded applications via the
Rc type that after initialization/instantiation can be cloned and multiple "pointers" to the same instance can exist.

So by using Rc, it's possible to share a value multiple times, but it's not possible to make changes on the value unless only a single reference to the value 
exists (Rc can be read as "reference counted", so it keeps track of how many live references exist).

In order to be able to perform changes on a seemingly inmutable value RefCell type is used, so a shareable & mutable value gets wrapped as Rc\<RefCell\<_Type_\>\>

The previously mentioned types are only usable on single-threaded applications/contexts as they are not meant to be thread-safe.

[Source](src/bin/shared.rs)


### directory utils [utils module]
Under utils there is a sample module utils where a pair of iterator related functionalities have been implemented:
- weave: combine iterators in a single iterator that produces the elements from the weaved iterators in a rotating approach
- interleave: similar to previous one, but simpler. Producing same result as weave when only 2 iterators are involved.

The way it works is as follows, having 3 iterators like these:

it1 = A1, A2, A3, A4, A5
it2 = B1, B2, B3, B4
it3 = C1, C2, C3, C4, C5, C6, C7

The results of **it1** *interleave*/*weave* **it2** would be:

A1, B1, A2, B2, A3, B3, A4, B4, A5

The results of **it1** *interleave* **it2** *interleave* **it3**:

A1, C1, B1, C2, A2, C3, B2, C4, A3, C5, B3, C6, A4, C7, B4, A5

[utils](src/utils/)
### module_use.rs

Binary/program using functionalities defined on utils module

[module_use](src/bin/module_use.rs)


### borrow_own.rs

Some examples showing basic details about ownership, kind of references (mutable/non-mutable) and in which 
kind of scenarios the compiler might not let you much further.

[borrow_own.rs](src/bin/borrow_own.rs)