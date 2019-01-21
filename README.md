# ternary-tree

A Rust implementation of Ternary Search Trees, with no unsafe blocks

[![Build Status](https://travis-ci.com/julien-montmartin/ternary-tree.svg?branch=master)](https://travis-ci.com/julien-montmartin/ternary-tree)
[![codecov](https://codecov.io/gh/julien-montmartin/ternary-tree/branch/master/graph/badge.svg)](https://codecov.io/gh/julien-montmartin/ternary-tree)

A Ternary Search Tree (TST) is a data structure which stores key/value pairs in a tree. The key is a string, and its characters are placed in the tree nodes. Each node may have three children (hence the name) : a _left_ child, a _middle_ child and a _right_ child.

A search in a TST compares the current character in the key with the character of the current node :

* If both matches, the search traverse the middle child, and proceed to the next character in the key
* If the key character is less than the node one, the search simply goes through the left child, and keep looking for the same key character
* Respectively, if the key character is greater than the node one, the search simply goes through the right child

The data structure and its algorithm are explained very well in [Dr.Dobb's Ternary Search Trees](http://www.drdobbs.com/database/ternary-search-trees/184410528) article.

The following tree is the TST we get after inserting the following keys in order : "aba", "ab", "bc", "ac", "abc", "a", "b", "aca", "caa", "cbc", "bac", "c", "cca", "aab", "abb", "aa" (see `tst.dot` produced by code below)

![An example of a Ternary Search Tree](http://files.jmontmartin.net/crates_io_sample_tst.png "An example of a Ternary Search Tree")

A checked box "☑" denotes a node  which stores a value (it corresponds to the last character of a key). An empty box "☐" means that the node has no value.

A TST can be used as a map, but it allows more flexible ways to retrieve values associated with keys. This crate provides four ways to iterate over the values of a TST :

* get all values (same as a regular map), with `visit_values` or `iter`
* get all values whose keys begin with some prefix (i.e. _complete_ some prefix), with `visit_complete_values` or `iter_complete`
* get all values whose keys are _close_ to some string ([Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)), with `visit_neighbor_values` or `iter_neighbor`
* get all values whose keys match a string with some joker (e.g. "a?c"), with `visit_crossword_values` or `iter_crossword`

Visit methods are recursive and apply a closure to found values. They exist in immutable and mutable version (i.e. `visit_neighbor_values_mut`). But once a value is found (based on its key), they offer no way to know what the actual key is.

Iterators, on the other hand, save their context in a `Vec` and only work on immutable trees. However they are double ended, and support `next` and `next_back` methods to walk the tree from both ends. Moreover, once a value is found, they offer the `current_key` and `current_key_back` methods to retrieve the key associated with the last value.

The following lines may give you a foretaste of this crate and TSTs

```rust
extern crate ternary_tree;

use ternary_tree::Tst;
use std::fs::File;
use std::error::Error;

const SOME_KEYS : [&str; 16] = ["aba", "ab", "bc", "ac",
"abc", "a", "b", "aca", "caa", "cbc", "bac", "c", "cca",
"aab", "abb", "aa"];

let mut map = Tst::new();

for key in &SOME_KEYS {

    //Say the value is the same as the key,
    //it makes the example easier !
    let some_value = *key;

    map.insert(key, some_value);
}

//Use Graphviz to convert tst.dot to tst.png:
//dot -T png -o tst.png tst.dot
let mut file = File::create("tst.dot").unwrap();
map.pretty_print(&mut file);

let mut v = Vec::new();

//Recursively get all values whose keys match "a?a" pattern
map.visit_crossword_values("a?a", '?', |s| v.push(s.clone()));
assert_eq!(v, ["aba", "aca"]);

v.clear();

//Iterate over all values whose keys are close to "abc" (Hamming distance of 1)
{
    let mut it = map.iter_neighbor("abc", 1);

    while let Some(value) = it.next() {

        v.push(*value);
    }
    assert_eq!(v, ["ab", "aba", "abb", "abc", "cbc"]);

    v.clear();
}

//Mutate all values whose keys begin with "c"
map.visit_complete_values_mut("c", |s| *s = "xxx");

assert_eq!(map.get("caa"), Some(&"xxx"));
assert_eq!(map.get("cbc"), Some(&"xxx"));
assert_eq!(map.get("cca"), Some(&"xxx"));
```
