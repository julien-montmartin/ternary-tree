extern crate ternary_tree;
use ternary_tree::Tst;


#[test]
fn tst_create_empty_map() {

    let map: Tst<String> = Tst::new();
    assert_eq!(map.len(), 0);
}


#[test]
fn tst_insert_some_key_value() {

    let mut map = Tst::new();
    assert_eq!(map.len(), 0);

    let old_value = map.insert("k", "v");
    assert_eq!(old_value, None);
    assert_eq!(map.len(), 1);
}


#[test]
fn tst_get_some_value_by_key() {

    let mut map = Tst::new();
    assert_eq!(map.len(), 0);

    let old_value = map.insert("k", "v");
    assert_eq!(old_value, None);
    assert_eq!(map.len(), 1);

    let k = "k";

    let v1 = map.get(k);
    assert_eq!(v1, Some(&"v"));

    let v2 = map.get(k);
    assert_eq!(v2, v1);
}


#[test]
fn tst_replace_some_value() {

    let mut map = Tst::new();
    assert_eq!(map.len(), 0);

    let old_value = map.insert("k", "v1");
    assert_eq!(old_value, None);
    assert_eq!(map.len(), 1);

    {
        let v1 = map.get("k");
        assert_eq!(v1, Some(&"v1"));
    }

    let old_value = map.insert("k", "v2");
    assert_eq!(old_value, Some("v1"));
    assert_eq!(map.len(), 1);

    let v2 = map.get("k");
    assert_eq!(v2, Some(&"v2"));
}


#[test]
fn tst_get_value_back_on_empty_key() {

    let mut map = Tst::new();
    assert_eq!(map.len(), 0);

    let this_value = map.insert("", "woups");
    assert_eq!(this_value, Some("woups"));
    assert_eq!(map.len(), 0);
}


const RANDOM_VEC_123 : [&str; 16] = ["aba", "ab", "bc", "ac", "abc", "a", "b", "aca", "caa", "cbc", "bac", "c", "cca", "aab", "abb", "aa"];

const RANDOM_VEC_123_BIS : [&str; 16] = [ "cca", "aa", "bac", "aba", "b", "bc", "c", "ac", "aab", "ab", "abc", "abb", "a", "caa", "aca", "cbc" ];

const SORTED_VEC_123 : [&str; 16] = ["a", "aa", "aab", "ab", "aba", "abb", "abc", "ac", "aca", "b", "bac", "bc", "c", "caa", "cbc", "cca"];


fn get_sample_map_abc_count() -> Tst<usize> {

    let mut map = Tst::new();
    let mut count = 0;

    for k in RANDOM_VEC_123.iter() {

        count+=1;
        let old_value = map.insert(k, count);
        assert_eq!(old_value, None);
        assert_eq!(map.len(), count);
    }

    map
}


fn get_sample_map_abc_abc() -> Tst<&'static str> {

    let mut map = Tst::new();
    let mut count = 0;

    for k in RANDOM_VEC_123.iter() {

        count+=1;
        let old_value = map.insert(k, *k);
        assert_eq!(old_value, None);
        assert_eq!(map.len(), count);
    }

    map
}


#[test]
fn tst_insert_and_get_more_key_value() {

    let map = get_sample_map_abc_count();

    for k in SORTED_VEC_123.iter() {

        let value = map.get(k);
        assert_eq!(*value.unwrap() > 0, true);
    }
}


#[test]
fn tst_iterate_over_empty_tree() {

    let empty_map: Tst<bool> = Tst::new();
    assert_eq!(empty_map.len(), 0);

    let mut it = empty_map.iter();

    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);

    assert_eq!(it.next_back(), None);
    assert_eq!(it.next_back(), None);
}


#[test]
fn tst_iterate_over_values() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter();

    assert_eq!(it.next(), Some(&"a"));
    assert_eq!(it.next(), Some(&"aa"));
    assert_eq!(it.next(), Some(&"aab"));

    ////////////////////////////////////////////////////

    it = map.iter();
    let mut v = Vec::new();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, SORTED_VEC_123);

    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);
}


#[test]
fn tst_iterate_over_values_with_for_loop() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut v = Vec::new();

    for value in &map {

        v.push(*value);
    }

    assert_eq!(v, SORTED_VEC_123);
}


#[test]
fn tst_iterate_over_values_with_peek_and_co() {

    let map = get_sample_map_abc_count();
    assert_eq!(map.len(), 16);

    let mut it = map.iter().peekable();

    assert_eq!(it.peek(), Some(&&6));
    assert_eq!(it.peek(), Some(&&6));
    assert_eq!(it.next(), Some(&6));

    assert_eq!(it.peek(), Some(&&16));
    assert_eq!(it.peek(), Some(&&16));
    assert_eq!(it.next(), Some(&16));

    ////////////////////////////////////////////////////

    let sum = map.iter().fold(0, |acc, v| acc + *v);
    let n = map.len();

    assert_eq!(sum, n*(n+1)/2);
}


#[test]
fn tst_iterate_over_values_backward() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter();

    assert_eq!(it.next_back(), Some(&"cca"));
    assert_eq!(it.next_back(), Some(&"cbc"));
    assert_eq!(it.next_back(), Some(&"caa"));

    ////////////////////////////////////////////////////

    it = map.iter();
    let mut v = Vec::new();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, SORTED_VEC_123);

    assert_eq!(it.next_back(), None);
    assert_eq!(it.next_back(), None);
}


#[test]
fn tst_iterate_over_values_backward_with_rev() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter().rev();

    assert_eq!(it.next(), Some(&"cca"));
    assert_eq!(it.next(), Some(&"cbc"));
    assert_eq!(it.next(), Some(&"caa"));

    ////////////////////////////////////////////////////

    it = map.iter().rev();
    let mut v = Vec::new();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, SORTED_VEC_123);

    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);
}


#[test]
fn tst_iterate_over_values_from_both_end() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter();
    let mut v = Vec::new();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, SORTED_VEC_123);

    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    ////////////////////////////////////////////////////

    it = map.iter();
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, SORTED_VEC_123);

    assert_eq!(it.next_back(), None);
    assert_eq!(it.next(), None);

    ////////////////////////////////////////////////////

    it = map.iter();

    let mut vi = Vec::new();
    let mut vj = Vec::new();

    for i in 0..map.len() {

        if i%2 == 0 {

            vi.push(*it.next().unwrap());

        } else {

            vj.push(*it.next_back().unwrap());
        }
    }

    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    vj.reverse();

    assert_eq!(vi, ["a", "aa", "aab", "ab", "aba", "abb", "abc", "ac"]);
    assert_eq!(vj, ["aca", "b", "bac", "bc", "c", "caa", "cbc", "cca"]);
}


#[test]
fn tst_visit_values() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut v = Vec::new();

    map.visit_values(|s| v.push(s.clone()));

    assert_eq!(v, SORTED_VEC_123);
}


#[test]
fn tst_visit_complete_values() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    map.visit_complete_values("cc", |s| assert_eq!(s, &"cca"));
    map.visit_complete_values("ccac", |s| assert_eq!(s, &"woups"));

    map.visit_complete_values("bc", |s| assert_eq!(s, &"woups"));

    let mut v = Vec::new();
    map.visit_complete_values("ab", |s| {v.push(s.clone())});
    assert_eq!(v, ["aba", "abb", "abc"]);

    v.clear();
    map.visit_complete_values("", |s| {v.push(s.clone())});
    assert_eq!(v, SORTED_VEC_123);
}


#[test]
fn tst_visit_neighbor_values() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    map.visit_neighbor_values("cc", 0, |s| assert_eq!(s, &"woups"));
    map.visit_neighbor_values("ccac", 0, |s| assert_eq!(s, &"woups"));

    map.visit_neighbor_values("abc", 0, |s| assert_eq!(s, &"abc"));

    let mut v = Vec::new();
    map.visit_neighbor_values("abc", 1, |s| {v.push(s.clone())});
    assert_eq!(v, ["ab", "aba", "abb", "abc", "cbc"]);

    v.clear();
    map.visit_neighbor_values("abc", 2, |s| {v.push(s.clone())});
    assert_eq!(v, ["a", "aa", "aab", "ab", "aba", "abb", "abc", "ac", "aca", "bac", "cbc"]);

    v.clear();
    map.visit_neighbor_values("abc", 3, |s| {v.push(s.clone())});
    assert_eq!(v, SORTED_VEC_123);

    v.clear();
    map.visit_neighbor_values("xxxx", 4, |s| {v.push(s.clone())});
    assert_eq!(v, SORTED_VEC_123);

    v.clear();
    map.visit_neighbor_values("", 0, |s| {v.push(s.clone())});
    assert_eq!(v.is_empty(), true);

    v.clear();
    map.visit_neighbor_values("", 1, |s| {v.push(s.clone())});
    assert_eq!(v, ["a", "b", "c"]);

    v.clear();
    map.visit_neighbor_values("", 2, |s| {v.push(s.clone())});
    assert_eq!(v, ["a", "aa", "ab", "ac", "b", "bc", "c"]);

    v.clear();
    map.visit_neighbor_values("", 3, |s| {v.push(s.clone())});
    assert_eq!(v, SORTED_VEC_123);

    v.clear();
    map.visit_neighbor_values("", 4, |s| {v.push(s.clone())});
    assert_eq!(v, SORTED_VEC_123);
}


#[test]
fn tst_visit_crossword_values() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut v = Vec::new();

    map.visit_crossword_values("", '?', |s| v.push(s.clone()));
    assert_eq!(v.is_empty(), true);

    v.clear();
    map.visit_crossword_values("?", '?', |s| v.push(s.clone()));
    assert_eq!(v, ["a", "b", "c"]);

    v.clear();
    map.visit_crossword_values("a?", '?', |s| v.push(s.clone()));
    assert_eq!(v, ["aa", "ab", "ac"]);

    v.clear();
    map.visit_crossword_values("a?a", '?', |s| v.push(s.clone()));
    assert_eq!(v, ["aba", "aca"]);

    v.clear();
    map.visit_crossword_values("?a?", '?', |s| v.push(s.clone()));
    assert_eq!(v, ["aab", "bac", "caa"]);

    v.clear();
    map.visit_crossword_values("???", '?', |s| v.push(s.clone()));
    assert_eq!(v, ["aab", "aba", "abb", "abc", "aca", "bac", "caa", "cbc", "cca"]);

    v.clear();
    map.visit_crossword_values("????", '?', |s| v.push(s.clone()));
    assert_eq!(v.is_empty(), true);

    v.clear();
    map.visit_crossword_values("aba", 'b', |s| v.push(s.clone()));
    assert_eq!(v, ["aba", "aca"]);
}


#[test]
fn tst_iterate_with_complete() {

    let empty_map: Tst<bool> = Tst::new();
    assert_eq!(empty_map.len(), 0);

    let mut it = empty_map.iter_complete("");

    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);

    assert_eq!(it.next_back(), None);
    assert_eq!(it.next_back(), None);

    ////////////////////////////////////////////////////

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter_complete("x");

    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);

    assert_eq!(it.next_back(), None);
    assert_eq!(it.next_back(), None);

    ////////////////////////////////////////////////////

    it = map.iter_complete("");
    let mut v = Vec::new();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, SORTED_VEC_123);

    ////////////////////////////////////////////////////

    it = map.iter_complete("ab");
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["aba", "abb", "abc"]);

    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);

    ////////////////////////////////////////////////////

    it = map.iter_complete("b");

    assert_eq!(it.next_back(), Some(&"bc"));
    assert_eq!(it.next(), Some(&"bac"));

    assert_eq!(it.next_back(), None);
    assert_eq!(it.next(), None);
}


#[test]
fn tst_iterate_with_neighbor() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter_neighbor("cc", 0);
    let mut v = Vec::new();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v.is_empty(), true);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("ccac", 0);
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v.is_empty(), true);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("abc", 0);
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["abc"]);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("abc", 1);
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["ab", "aba", "abb", "abc", "cbc"]);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("abc", 2);
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["a", "aa", "aab", "ab", "aba", "abb", "abc", "ac", "aca", "bac", "cbc"]);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("abc", 3);
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, SORTED_VEC_123);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("xxxx", 4);
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, SORTED_VEC_123);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("", 0);
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v.is_empty(), true);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("", 1);
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["a", "b", "c"]);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("", 2);
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["a", "aa", "ab", "ac", "b", "bc", "c"]);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("", 3);
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, SORTED_VEC_123);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("", 4);
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, SORTED_VEC_123);
}


#[test]
fn tst_iterate_with_neighbor_backward() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter_neighbor("abc", 1);
    let mut v = Vec::new();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["ab", "aba", "abb", "abc", "cbc"]);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("cc", 0);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    assert_eq!(v.is_empty(), true);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("ccac", 0);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    assert_eq!(v.is_empty(), true);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("abc", 0);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    assert_eq!(v, ["abc"]);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("abc", 1);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["ab", "aba", "abb", "abc", "cbc"]);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("abc", 2);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["a", "aa", "aab", "ab", "aba", "abb", "abc", "ac", "aca", "bac", "cbc"]);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("abc", 3);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, SORTED_VEC_123);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("xxxx", 4);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, SORTED_VEC_123);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("", 0);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    assert_eq!(v.is_empty(), true);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("", 1);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["a", "b", "c"]);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("", 2);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["a", "aa", "ab", "ac", "b", "bc", "c"]);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("", 3);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, SORTED_VEC_123);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("", 4);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, SORTED_VEC_123);
}


#[test]
fn tst_iterate_with_neighbor_from_both_end() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter_neighbor("abc", 2);
    let mut v = Vec::new();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["a", "aa", "aab", "ab", "aba", "abb", "abc", "ac", "aca", "bac", "cbc"]);

    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("abc", 2);
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["a", "aa", "aab", "ab", "aba", "abb", "abc", "ac", "aca", "bac", "cbc"]);

    assert_eq!(it.next_back(), None);
    assert_eq!(it.next(), None);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("abc", 2);
    let count = it.count();
    assert_eq!(count, 11);

    it = map.iter_neighbor("abc", 2);

    let mut vi = Vec::new();
    let mut vj = Vec::new();

    for i in 0..count {

        if i%2 == 0 {

            vi.push(*it.next().unwrap());

        } else {

            vj.push(*it.next_back().unwrap());
        }
    }

    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    vj.reverse();

    assert_eq!(vi, ["a", "aa", "aab", "ab", "aba", "abb"]);
    assert_eq!(vj, ["abc", "ac", "aca", "bac", "cbc"]);
}


#[test]
fn tst_iterate_with_crossword() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter_crossword("", '?');
    let mut v = Vec::new();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v.is_empty(), true);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("?", '?');
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["a", "b", "c"]);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("a?", '?');
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["aa", "ab", "ac"]);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("a?a", '?');
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["aba", "aca"]);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("?a?", '?');
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["aab", "bac", "caa"]);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("???", '?');
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["aab", "aba", "abb", "abc", "aca", "bac", "caa", "cbc", "cca"]);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("????", '?');
    v.clear();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v.is_empty(), true);
}


#[test]
fn tst_iterate_with_crossword_backward() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter_crossword("", '?');
    let mut v = Vec::new();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v.is_empty(), true);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("?", '?');
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["a", "b", "c"]);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("a?", '?');
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["aa", "ab", "ac"]);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("a?a", '?');
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["aba", "aca"]);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("?a?", '?');
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["aab", "bac", "caa"]);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("???", '?');
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["aab", "aba", "abb", "abc", "aca", "bac", "caa", "cbc", "cca"]);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("????", '?');
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v.is_empty(), true);
}


#[test]
fn tst_iterate_with_crossword_from_both_end() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter_crossword("?a?", '?');
    let mut v = Vec::new();

    while let Some(value) = it.next() {

        v.push(*value);
    }

    assert_eq!(v, ["aab", "bac", "caa"]);

    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("?a?", '?');
    v.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
    }

    v.reverse();

    assert_eq!(v, ["aab", "bac", "caa"]);

    assert_eq!(it.next_back(), None);
    assert_eq!(it.next(), None);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("?a?", '?');
    let count = it.count();
    assert_eq!(count, 3);

    it = map.iter_crossword("?a?", '?');

    let mut vi = Vec::new();
    let mut vj = Vec::new();

    for i in 0..count {

        if i%2 == 0 {

            vi.push(*it.next().unwrap());

        } else {

            vj.push(*it.next_back().unwrap());
        }
    }

    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    vj.reverse();

    assert_eq!(vi, ["aab", "bac"]);
    assert_eq!(vj, ["caa"]);
}


#[test]
fn tst_insert_and_remove_some_key_value() {

    let mut empty_map: Tst<bool> = Tst::new();
    assert_eq!(empty_map.len(), 0);

    let value = empty_map.remove("x");
    assert_eq!(value.is_none(), true);

    let mut map = Tst::new();
    assert_eq!(map.len(), 0);

    let old_value = map.insert("k", "v");
    assert_eq!(old_value, None);
    assert_eq!(map.len(), 1);

    let value = empty_map.remove("x");
    assert_eq!(value, None);
    assert_eq!(map.len(), 1);

    {
        let value = map.get("k");
        assert_eq!(value, Some(&"v"));
        assert_eq!(map.len(), 1);
    }

    let value = map.remove("k");
    assert_eq!(value, Some("v"));
    assert_eq!(map.len(), 0);

    {
        let value = map.get("k");
        assert_eq!(value, None);
        assert_eq!(map.len(), 0);
    }

    let value = map.remove("k");
    assert_eq!(value, None);
    assert_eq!(map.len(), 0);
}


#[test]
fn tst_insert_and_remove_more_key_value() {

    let mut map = get_sample_map_abc_abc();

    for k in RANDOM_VEC_123_BIS.iter() {

        let len = map.len();

        {
            let value = map.get(k);
            assert_eq!(value, Some(k));
            assert_eq!(map.len(), len);
        }

        let value = map.remove(k);
        assert_eq!(value, Some(*k));
        assert_eq!(map.len(), len-1);

        {
            let value = map.get(k);
            assert_eq!(value, None);
            assert_eq!(map.len(), len-1);
        }

        let value = map.remove(k);
        assert_eq!(value, None);
        assert_eq!(map.len(), len-1);
    }

    assert_eq!(map.len(), 0);

    let old_value = map.insert("k", "v");
    assert_eq!(old_value, None);
    assert_eq!(map.len(), 1);

    {
        let value = map.get("k");
        assert_eq!(value, Some(&"v"));
        assert_eq!(map.len(), 1);
    }

    let value = map.remove("k");
    assert_eq!(value, Some("v"));
    assert_eq!(map.len(), 0);

    {
        let value = map.get("k");
        assert_eq!(value, None);
        assert_eq!(map.len(), 0);
    }

    let value = map.remove("k");
    assert_eq!(value, None);
    assert_eq!(map.len(), 0);
}


#[test]
fn tst_stats_on_insert_and_remove() {

    let empty_map: Tst<bool> = Tst::new();

    let s1 = empty_map.stat();

    assert_eq!(s1.dist.is_empty(), true);
    assert_eq!(s1.key_len.min, 0);
    assert_eq!(s1.key_len.max, 0);
    assert_eq!(s1.count.nodes, 0);
    assert_eq!(s1.count.values, 0);
    assert_eq!(s1.count.values, empty_map.len());

    //node struct size should be around 32 bytes on x64
    assert_eq!(s1.bytes.node >= 16, true);
    assert_eq!(s1.bytes.node <= 64, true);

    //main tree struct size should be around 16 bytes on x64
    assert_eq!(s1.bytes.total >= 8, true);
    assert_eq!(s1.bytes.total <= 32, true);

    let map = get_sample_map_abc_abc();

    let s2 = map.stat();

    assert_eq!(s2.dist.is_empty(), false);
    assert_eq!(s2.key_len.min, 1);
    assert_eq!(s2.key_len.max, 3);
    assert_eq!(s2.count.nodes, 20);
    assert_eq!(s2.count.values, 16);
    assert_eq!(s2.count.values, map.len());

    //node struct size should be around 48 bytes on x64
    assert_eq!(s2.bytes.node >= 24, true);
    assert_eq!(s2.bytes.node <= 96, true);

    //total size should be around 976 bytes on x64
    assert_eq!(s2.bytes.total >= 488, true);
    assert_eq!(s2.bytes.total <= 16+20*48, true);

    assert_eq!(s1.bytes.node < s2.bytes.node, true);
    assert_eq!(s1.bytes.total < s2.bytes.total, true);

    use ternary_tree::DistStat;

    assert_eq!(s2.dist[0], DistStat { matches: 0, sides: 3, depth: 0 });
    assert_eq!(s2.dist[1], DistStat { matches: 3, sides: 7, depth: 1 });
    assert_eq!(s2.dist[2], DistStat { matches: 4, sides: 4, depth: 2 });
    assert_eq!(s2.dist[3], DistStat { matches: 9, sides: 1, depth: 5 });
    assert_eq!(s2.dist[4], DistStat { matches: 0, sides: 1, depth: 3 });
    assert_eq!(s2.dist[5], DistStat { matches: 0, sides: 0, depth: 3 });
    assert_eq!(s2.dist[6], DistStat { matches: 0, sides: 0, depth: 1 });
    assert_eq!(s2.dist[7], DistStat { matches: 0, sides: 0, depth: 1 });
    assert_eq!(s2.dist.len(), 8);

    ////////////////////////////////////////////////////

    let mut map = Tst::new();

    let s = map.stat();
    assert_eq!(s.count.nodes, 0);
    assert_eq!(s.count.values, 0);

    for k in RANDOM_VEC_123.iter() {

        let s1 = map.stat();
        map.insert(k, *k);
        let s2 = map.stat();

        assert_eq!(s1.count.nodes <= s2.count.nodes, true);
        assert_eq!(s1.count.values <= s2.count.values, true);
        assert_eq!(s2.count.values, map.len());
    }

    for k in RANDOM_VEC_123_BIS.iter() {

        let s1 = map.stat();
        map.remove(k);
        let s2 = map.stat();

        assert_eq!(s2.count.nodes <= s1.count.nodes, true);
        assert_eq!(s2.count.values <= s1.count.values, true);
        assert_eq!(s2.count.values, map.len());
    }

    let s = map.stat();
    assert_eq!(s.count.nodes, 0);
    assert_eq!(s.count.values, 0);
}


#[test]
fn tst_clear_some_map() {

    let mut empty_map: Tst<bool> = Tst::new();

    empty_map.clear();

    let s = empty_map.stat();

    assert_eq!(s.count.nodes, 0);
    assert_eq!(s.count.values, 0);
    assert_eq!(s.count.values, empty_map.len());

    ////////////////////////////////////////////////////

    let mut map = get_sample_map_abc_abc();

    let s = map.stat();

    assert_eq!(s.count.nodes, 20);
    assert_eq!(s.count.values, 16);
    assert_eq!(s.count.values, map.len());

    map.clear();

    let s = map.stat();

    assert_eq!(s.count.nodes, 0);
    assert_eq!(s.count.values, 0);
    assert_eq!(s.count.values, map.len());
}


#[test]
fn tst_iterate_and_read_current_key() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter();
    let mut v = Vec::new();
    let mut w = Vec::new();

    while let Some(value) = it.next() {

        v.push(*value);
        w.push(it.current_key());
    }

    assert_eq!(v, SORTED_VEC_123);
    assert_eq!(w, SORTED_VEC_123);
}


#[test]
fn tst_iterate_and_read_current_key_back() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter();
    let mut v = Vec::new();
    let mut w = Vec::new();

    while let Some(value) = it.next_back() {

        v.push(*value);
        w.push(it.current_key_back());
    }

    v.reverse();
    w.reverse();

    assert_eq!(v, SORTED_VEC_123);
    assert_eq!(w, SORTED_VEC_123);
}


#[test]
fn tst_current_key_iterators() {

    let map = get_sample_map_abc_abc();
    assert_eq!(map.len(), 16);

    let mut it = map.iter_complete("ab");
    let mut v = Vec::new();
    let mut w = Vec::new();

    while let Some(value) = it.next() {

        v.push(*value);
        w.push(it.current_key());
    }

    assert_eq!(v, w);

    ////////////////////////////////////////////////////

    it = map.iter_complete("ab");
    v.clear();
    w.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
        w.push(it.current_key_back());
    }

    assert_eq!(v, w);

    ////////////////////////////////////////////////////

    let mut it = map.iter_neighbor("abc", 1);
    v.clear();
    w.clear();

    while let Some(value) = it.next() {

        v.push(*value);
        w.push(it.current_key());
    }

    assert_eq!(v, w);

    ////////////////////////////////////////////////////

    it = map.iter_neighbor("abc", 1);
    v.clear();
    w.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
        w.push(it.current_key_back());
    }

    assert_eq!(v, w);

    ////////////////////////////////////////////////////

    let mut it = map.iter_crossword("a?a", '?');
    v.clear();
    w.clear();

    while let Some(value) = it.next() {

        v.push(*value);
        w.push(it.current_key());
    }

    assert_eq!(v, w);

    ////////////////////////////////////////////////////

    it = map.iter_crossword("a?a", '?');
    v.clear();
    w.clear();

    while let Some(value) = it.next_back() {

        v.push(*value);
        w.push(it.current_key_back());
    }

    assert_eq!(v, w);
}


#[test]
fn tst_update_some_values() {

    let mut map = get_sample_map_abc_count();
    assert_eq!(map.len(), 16);

    for k in SORTED_VEC_123.iter() {

        let value =  map.get_mut(k);

        if let Some(c) = value {

            assert_eq!(*c > 0, true);
            *c = 0;
        }
    }

    let mut v = Vec::new();

    map.visit_values(|c| v.push(c.clone()));
    assert_eq!(v, [0 ; 16]);

    ////////////////////////////////////////////////////

    map.visit_values_mut(|c| *c = 1);

    v.clear();

    map.visit_values(|c| v.push(c.clone()));
    assert_eq!(v, [1 ; 16]);

    ////////////////////////////////////////////////////

    map.visit_complete_values_mut("a", |c| *c = 2);

    v.clear();

    map.visit_values(|c| v.push(c.clone()));
    assert_eq!(v, [1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1]);

    ////////////////////////////////////////////////////

    map.visit_neighbor_values_mut("abc", 1, |c| *c = 3);

    v.clear();

    map.visit_values(|c| v.push(c.clone()));
    assert_eq!(v, [1, 2, 2, 3, 3, 3, 3, 2, 2, 1, 1, 1, 1, 1, 3, 1]);

    ////////////////////////////////////////////////////

    map.visit_crossword_values_mut("a?a", '?', |c| *c = 4);

    v.clear();

    map.visit_values(|c| v.push(c.clone()));
    assert_eq!(v, [1, 2, 2, 3, 4, 3, 3, 2, 4, 1, 1, 1, 1, 1, 3, 1]);
}


#[test]
fn tst_create_with_macro() {

    use ternary_tree::tst;

    let map = tst!["aba" => "aba", "ab" => "ab", "bc" => "bc", "ac" => "ac",
                   "abc" => "abc", "a" => "a", "b" => "b", "aca" => "aca",
                   "caa" => "caa", "cbc" => "cbc", "bac" => "bac", "c" => "c",
                   "cca" => "cca", "aab" => "aab", "abb" => "abb", "aa" => "aa"];

    let stat = map.stat();

    assert_eq!(stat.dist.is_empty(), false);
    assert_eq!(stat.key_len.min, 1);
    assert_eq!(stat.key_len.max, 3);
    assert_eq!(stat.count.nodes, 20);
    assert_eq!(stat.count.values, 16);
    assert_eq!(stat.count.values, map.len());

    //node struct size should be around 48 bytes on x64
    assert_eq!(stat.bytes.node >= 24, true);
    assert_eq!(stat.bytes.node <= 96, true);

    //total size should be around 976 bytes on x64
    assert_eq!(stat.bytes.total >= 488, true);
    assert_eq!(stat.bytes.total <= 16+20*48, true);

    use ternary_tree::DistStat;

    assert_eq!(stat.dist[0], DistStat { matches: 0, sides: 3, depth: 0 });
    assert_eq!(stat.dist[1], DistStat { matches: 3, sides: 7, depth: 1 });
    assert_eq!(stat.dist[2], DistStat { matches: 4, sides: 4, depth: 2 });
    assert_eq!(stat.dist[3], DistStat { matches: 9, sides: 1, depth: 5 });
    assert_eq!(stat.dist[4], DistStat { matches: 0, sides: 1, depth: 3 });
    assert_eq!(stat.dist[5], DistStat { matches: 0, sides: 0, depth: 3 });
    assert_eq!(stat.dist[6], DistStat { matches: 0, sides: 0, depth: 1 });
    assert_eq!(stat.dist[7], DistStat { matches: 0, sides: 0, depth: 1 });
    assert_eq!(stat.dist.len(), 8);

    let mut v = Vec::new();

    map.visit_values(|s| v.push(s.clone()));

    assert_eq!(v, SORTED_VEC_123);
}


#[test]
fn tst_pretty_print() {

    use ternary_tree::tst;

    let map = tst!["ab" => "ab", "aa" => "aa", "ac" => "ac"];

    let mut w = Vec::new();

    map.pretty_print(&mut w);

    let s = String::from_utf8(w).unwrap();
    //print!("{:?}", s);

    let r = "digraph {\nnode [shape=plaintext]\nN0 [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD COLSPAN=\"3\">☐ a</TD></TR><TR><TD PORT=\"l\"></TD><TD PORT=\"m\"></TD><TD PORT=\"r\"></TD></TR></TABLE>>]\nN0:m -> N1 [style=bold]\nN1 [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD COLSPAN=\"3\">☑ b</TD></TR><TR><TD PORT=\"l\"></TD><TD PORT=\"m\"></TD><TD PORT=\"r\"></TD></TR></TABLE>>]\nN1:l -> N2 [style=solid]\nN1:r -> N3 [style=solid]\nN2 [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD COLSPAN=\"3\">☑ a</TD></TR><TR><TD PORT=\"l\"></TD><TD PORT=\"m\"></TD><TD PORT=\"r\"></TD></TR></TABLE>>]\nN3 [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD COLSPAN=\"3\">☑ c</TD></TR><TR><TD PORT=\"l\"></TD><TD PORT=\"m\"></TD><TD PORT=\"r\"></TD></TR></TABLE>>]\n}\n";

    assert_eq!(s, r);
}
