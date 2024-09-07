use crate::path_item::PathItem;
use std::{
    convert::Infallible,
    env::{self, VarError},
    str::FromStr,
};

#[derive(Debug)]
pub struct Path(Vec<PathItem>);

impl Path {
    #[cfg(windows)]
    pub const SEP: char = ';';
    #[cfg(unix)]
    pub const SEP: char = ':';

    /// Get the path as available to the current process at the time of invocation
    fn for_process() -> Result<Self, Infallible> {
        match env::var("PATH") {
            Ok(path) => Self::from_str(&path),
            Err(err) => match err {
                VarError::NotPresent => Self::from_str(""),
                VarError::NotUnicode(val) => Self::from_str(&val.to_string_lossy()),
            },
        }
    }

    /// Get the path visible to the current user at the start of any process.t
    ///
    /// ## Platform-Specific Details
    ///
    /// ### Windows
    ///
    /// On Windows, this information is looked up directly from the system registry.
    ///
    /// ### Unix-like
    ///
    /// This function isn't available on Unix-like systems since there is no concept of
    /// user-specific environment variables. The closest analogue is to use [Self::for_shell]
    /// and pass a specific shell name. That shell will then be invoked and the PATH for that
    /// shell on initialization will be returned.
    fn for_user() -> Self {
        todo!()
    }

    fn for_shell() -> Self {
        todo!()
    }

    fn for_system() -> Self {
        todo!()
    }
}

impl FromStr for Path {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self(Vec::with_capacity(0)));
        }

        let result: Vec<_> = s
            .split(Self::SEP)
            .map(PathItem::from_str)
            .map(Result::unwrap)
            .collect();

        Ok(Self(result))
    }
}

#[cfg(test)]
mod test {
    use super::Path;
    use std::str::FromStr;

    const EMPTY_PATH: &str = "";
    #[cfg(windows)]
    const PATH_ONE_ITEM_PATH: &str = "C:\\Users\\bnhei\\source\\path-update";
    #[cfg(unix)]
    const PATH_ONE_ITEM_PATH: &str = "/usr/bin";

    #[test]
    fn empty_path() {
        let path = Path::from_str(EMPTY_PATH);

        let path = match path {
            Ok(path) => path,
            Err(err) => panic!("Should have succeeded but got: {err}"),
        };

        assert_eq!(path.0.len(), 0)
    }

    #[test]
    fn path_with_one_path_item_returns_path_with_one_path_item() {
        let path = Path::from_str(PATH_ONE_ITEM_PATH);

        let path = match path {
            Ok(path) => path,
            Err(err) => panic!("Should have succeeded but got: {err}"),
        };

        assert_eq!(path.0.len(), 1);

        let item = path.0.first().expect("Expected there to be one path item");

        let path = match item {
            crate::PathItem::Path(path) => path,
            crate::PathItem::Variable(var) => {
                panic!("Should have gotten a path, but got a var: {var}")
            }
        };

        assert_eq!(path.to_string_lossy(), PATH_ONE_ITEM_PATH);
    }
}
