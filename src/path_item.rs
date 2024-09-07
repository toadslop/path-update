use thiserror::Error;

use crate::variable::Variable;
use std::{convert::Infallible, path::PathBuf, str::FromStr};

#[derive(Debug)]
pub enum PathItem {
    Path(PathBuf),
    Variable(Variable),
}

impl FromStr for PathItem {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match Variable::from_str(s) {
            Ok(var) => PathItem::Variable(var),
            Err(_) => PathItem::Path(PathBuf::from(s)),
        };

        Ok(result)
    }
}

#[derive(Debug, Error)]
pub enum ParsePathItemError {}
