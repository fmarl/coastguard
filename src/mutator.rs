mod levenshtein;
mod homoglyph;
mod pre_in_suffix;

pub trait Mutator {
    fn mutate(input: String) -> String;
}