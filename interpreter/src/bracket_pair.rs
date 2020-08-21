/// struct representing a matching pair of brackets
pub struct BracketPair {
    first_bracket: usize,
    second_bracket: usize,
}

impl BracketPair {
    /// create a bracket pair from a pair of indices
    pub fn from_indeces(pair: (usize, usize)) -> Self {
        BracketPair {
            first_bracket: pair.0,
            second_bracket: pair.1,
        }
    }

    /// returns the first bracktet's index
    pub fn get_first_bracket_index(&self) -> usize {
        self.first_bracket
    }

    /// returns the second bracktet's index
    pub fn get_second_bracket_index(&self) -> usize {
        self.second_bracket
    }
}
