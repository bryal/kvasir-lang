pub use self::collections::ScopeStack;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::once;
use std::{cmp, fmt, io, time};
use std::path::{Path, PathBuf};

#[macro_use]
pub mod front;
pub mod back;
pub mod collections;

/// Returns the unit set of the single element `x`
fn set_of<T: cmp::Ord>(x: T) -> BTreeSet<T> {
    once(x).collect()
}

/// Returns the map of `{k} -> {v}`
fn map_of<K: cmp::Ord, V>(k: K, v: V) -> BTreeMap<K, V> {
    once((k, v)).collect()
}

/// A path-buffer that is guaranteed to be canonical
#[derive(PartialEq, Clone)]
pub struct CanonPathBuf(PathBuf);

impl CanonPathBuf {
    pub fn new(path: &str) -> io::Result<Self> {
        PathBuf::from(path)
            .canonicalize()
            .map(|pb| CanonPathBuf(pb))
    }

    pub fn path(&self) -> &Path {
        self.0.as_ref()
    }

    pub fn with_extension(&self, ext: &str) -> Self {
        CanonPathBuf(self.0.with_extension(ext))
    }
}

impl AsRef<Path> for CanonPathBuf {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

pub struct ErrCode {
    pub module: &'static str,
    pub number: usize,
}

impl ErrCode {
    pub fn undefined() -> Self {
        ErrCode {
            module: "UNDEFINED",
            number: 0,
        }
    }
}

impl fmt::Display for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.module, self.number)
    }
}

/// Time an action and print out the result. Use for profiling/benchmarking purposes
pub fn time_action<A, P, R>(action: A, print_f: P) -> R
where
    A: FnOnce() -> R,
    P: FnOnce(String),
{
    let t0 = time::Instant::now();
    let r = action();
    let t = t0.elapsed();
    print_f(format!("{}.{:04}", t.as_secs(), t.subsec_millis()));
    r
}
