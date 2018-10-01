use std::env;
use std::ffi::{OsStr, OsString};
use std::iter::{self, Iterator};
use std::path::PathBuf;

pub fn var<K: AsRef<OsStr>>(key: K) -> Option<Var> {
    env::var_os(key).map(Var::new)
}

#[cfg(windows)]
const PATH_VAR_NAME: &'static str = "Path";

#[cfg(unix)]
const PATH_VAR_NAME: &'static str = "PATH";

pub fn path() -> Option<Var> {
    var(PATH_VAR_NAME)
}

pub fn set_path<V: AsRef<OsStr>>(v: V) {
    env::set_var(PATH_VAR_NAME, v);
}

pub struct PathVar<'a> {
    entries: Option<Box<dyn Iterator<Item = PathBuf> + 'a>>
}

impl<'a> PathVar<'a> {
    fn new(original: &'a Var) -> Self {
        let mut path = PathVar { entries: None };
        path.entries = Some(Box::new(env::split_paths(original)));
        path
    }

    pub fn remove(mut self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        self.entries = Some(Box::new(self.entries.unwrap().filter(move |p| p != &path)));
        self
    }

    pub fn prefix(mut self, path: impl Into<PathBuf>) -> PathVar<'a> {
        self.entries = Some(Box::new(iter::once(path.into()).chain(self.entries.unwrap())));
        self
    }

    pub fn suffix(mut self, path: impl Into<PathBuf>) -> PathVar<'a> {
        self.entries = Some(Box::new(self.entries.unwrap().chain(iter::once(path.into()))));
        self
    }

    pub fn join(self) -> Result<OsString, env::JoinPathsError> {
        env::join_paths(self.entries.unwrap())
    }
}

impl<'a> Iterator for PathVar<'a> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.entries.as_mut().unwrap().next()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Var {
    payload: OsString
}

impl AsRef<OsStr> for Var {
    fn as_ref(&self) -> &OsStr {
        &self.payload
    }
}

impl Var {
    fn new(payload: OsString) -> Var {
        Var { payload }
    }

    pub fn split(&self) -> PathVar {
        PathVar::new(self)
    }

    pub fn into_string(self) -> Result<String, OsString> {
        self.payload.into_string()
    }
}

impl From<String> for Var {
    fn from(s: String) -> Self {
        Var { payload: OsString::from(s) }
    }
}

impl From<Var> for OsString {
    fn from(v: Var) -> Self {
        v.payload
    }
}

impl<'a> From<&'a str> for Var {
    fn from(s: &'a str) -> Self {
        Var { payload: OsString::from(s) }
    }
}

#[cfg(all(test, unix))]
mod tests {
    use std::ffi::OsString;
    use super::{path, Var};

    #[test]
    fn string_contents() {
        assert_eq!(OsString::from(Var::from("/usr/bin:/usr/local/bin")),
            OsString::from("/usr/bin:/usr/local/bin"));
    }

    #[test]
    fn path_exists() {
        assert_eq!(path().is_some(), true);
    }

    #[test]
    fn count_split_path() {
        let var = Var::from("/bin:/usr/bin:/usr/local/bin");
        assert_eq!(var.split().count(), 3);
    }

    #[test]
    fn split_join_round_trip() {
        let var = Var::from("/bin:/usr/bin:/usr/local/bin");
        assert_eq!(OsString::from(var.split().join().unwrap()), OsString::from(var.clone()));
    }

    #[test]
    fn remove() {
        let var = Var::from("/bin:/usr/bin:/usr/local/bin");
        assert_eq!(OsString::from(var.split().remove("/usr/bin").join().unwrap()),
            OsString::from("/bin:/usr/local/bin"));
    }

    #[test]
    fn prefix() {
        let var = Var::from("/usr/bin:/usr/local/bin");
        assert_eq!(OsString::from(var.split().prefix("/bin").join().unwrap()),
            OsString::from("/bin:/usr/bin:/usr/local/bin"));
    }

    #[test]    
    fn suffix() {
        let var = Var::from("/bin:/usr/bin");
        assert_eq!(OsString::from(var.split().suffix("/usr/local/bin").join().unwrap()),
            OsString::from("/bin:/usr/bin:/usr/local/bin"));
    }
}
