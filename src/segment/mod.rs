//! UAX #29 Unicode Text Segmentation.
//!
//! Grapheme cluster, word, and sentence boundaries for cursor movement,
//! backspace, truncation, and character counting.

mod grapheme;
mod sentence;
mod word;

pub use grapheme::{grapheme_boundaries, grapheme_cluster_boundaries, GraphemeClusterBoundaries};
pub use sentence::sentence_boundaries;
pub use word::word_boundaries;
