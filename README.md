# Advent of code Rust

Working on Advent of code challenges 2020.
`cargo watch -x run --clear`

"Link advent of code: https://adventofcode.com/ (append the year, starting from 2015). Solutions in rust (from 2020): https://github.com/lmammino/rust-advent/tree/main/"

If formatter on autoSave is enabled and the autosave is not working, run `cargo fmt` and check what is wrong: if you fix it the formatter should work again

You can use for loops just with ***iterators***
Common methods for iterator:
- ***.collect()***: convert an iterator into a collection
- ***map()***
- ***for_each()***
- ***nth()***
- ***flatten()***
- ***partition()***: it creates two vectors according to the function argument. The first vector is populated by the elements satisfying the condition
- ***find()***: find element that satisfy the predicate (=function predicate)
- ***sum()***
- ***product()***
- ***any()***: returns true if the predicate is true at least once over the iteration
- ***all()*** returns true if the predicate returns always true duiring the iteration

Always use ***&str***, except for manipulating it, in that case use ***String***

In order to apply good logic on list of items, use ***HashSet***:
- ***intersection()*** -> check also ***symmetric_difference()***
- ***union()***
- ***drain()***: empties an hashset and return its values as an iterator -> is different from ***clear()*** for this second feature
- ***retain()***: filters out all the elements that do not respect the argument of the function.
- ***a.difference(&b)*** returns the element of "a" that are not present in "b"
- ***a.symmetric_difference(&b)*** the opposite of intersection
- ***contains()***
. ***is_disjoint()*** true if the two hashsets do not any item in common
- ***take()*** removes the item and returns it -> this is what makes take() different from ***remove()***

Cool methods for ***HashMap***:
- ***keys()***
- ***values()***
- ***drain()***: returns a tuple for keys and value
