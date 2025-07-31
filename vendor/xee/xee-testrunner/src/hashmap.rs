use fxhash::FxBuildHasher;
use indexmap::{IndexMap, IndexSet};

// we use indexmap so we get stable serialization in insta tests,
// The run-time impact should be small as we're not really
// intense users
pub(crate) type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
pub(crate) type FxIndexSet<T> = IndexSet<T, FxBuildHasher>;
