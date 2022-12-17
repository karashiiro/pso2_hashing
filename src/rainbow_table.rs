use std::{ops::Range, str::FromStr};

use itertools::Itertools;

pub fn build_graphemes<F>(charset: &[char], grapheme_size: usize, filter: F) -> Vec<String>
where
    F: Fn(&str) -> bool,
{
    debug_assert!(grapheme_size >= 1);
    (0..grapheme_size + 1)
        .flat_map(|n| {
            charset
                .into_iter()
                // Removing this causes chars to not be duplicated when they should be.
                // However, this also produces every single permutation twice.
                .flat_map(move |m| std::iter::repeat(m).take(n))
                .permutations(n)
                // Hence the filter :)
                // This can be fixed properly once it causes performance issues. Taking
                // the Cartesian product of the charset with itself `n` times is the correct
                // way to do this, but that's gross to do because of itertools types.
                .unique()
        })
        .filter_map(|chars| {
            let s = String::from_iter(chars);
            if filter(&s) {
                Some(s)
            } else {
                None
            }
        })
        .collect_vec()
}

pub fn validate_permutation_bounds(min: usize, max: usize, grapheme_max: usize) -> (usize, usize) {
    debug_assert!(max >= min);
    debug_assert!(min % grapheme_max == 0);
    debug_assert!(max % grapheme_max == 0);
    let min = min / grapheme_max;
    let max = max / grapheme_max;
    (min, max)
}

fn take_str_slice(s: &[&str]) -> Vec<String> {
    s.into_iter()
        .map(|prefix| String::from_str(prefix).expect("input string should have been well-formed"))
        .collect_vec()
}

struct PlaintextGenerator {
    prefix_list: Vec<String>,
    suffix_list: Vec<String>,
    graphemes: Vec<String>,
    graphemes_per_str: usize,
    item_max_len: usize,
}

impl PlaintextGenerator {
    fn new<F>(
        prefix_list: &[&str],
        suffix_list: &[&str],
        charset: &str,
        grapheme_max_len: usize,
        grapheme_filter: F,
        item_max_len: usize,
    ) -> Self
    where
        F: Fn(&str) -> bool,
    {
        let (permuted_min_len, permuted_max_len) =
            validate_permutation_bounds(0, item_max_len, grapheme_max_len);
        let graphemes_per_str = permuted_max_len - permuted_min_len;
        Self {
            prefix_list: take_str_slice(prefix_list),
            suffix_list: take_str_slice(suffix_list),
            // Build valid graphemes to minimize set of generated strings.
            // The larger the grapheme length is, the more time will be spent generating
            // graphemes (duh). However, filtering out more illegal graphemes ahead of time
            // will result in far fewer hashes needing to be generated.
            graphemes: build_graphemes(
                &charset.chars().collect_vec(),
                grapheme_max_len,
                grapheme_filter,
            ),
            graphemes_per_str,
            item_max_len,
        }
    }

    fn len(&self) -> usize {
        self.prefix_list.len()
            * self.suffix_list.len()
            * self.graphemes.len().pow(self.graphemes_per_str as u32)
    }

    fn range(&self, output_len: u32) -> Range<u64> {
        0..(self.len() as u64)
    }

    fn get(&self, i: u64) -> String {
        // The index of a particular permutation P can be represented by a tuple
        // (pi, si, G), where pi and si are the indices of the prefix and
        // suffix, and G is a tuple of the indices of the graphemes used to
        // produce the final string. Given this tuple (pi, si, G), the index can
        // be converted into a single value by computing:
        //
        // a = pi + si * p_max
        // i = a + G * s_max
        //
        // a represents the 1-dimensional index of (pi, si) on the 2-dimensional
        // plane (p, s) of prefixes and suffixes (TODO finish this)
        String::from("")
    }
}

struct PSO2RainbowTable {
    prefix_list: Vec<String>,
    suffix_list: Vec<String>,
    charset: String,
    item_max_len: usize,
    chain_length: usize,
    total_chains: u64,
}
