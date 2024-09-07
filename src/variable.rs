use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Debug)]
pub struct Variable(String);

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(windows)]
        {
            write!(f, "{}{}{}", Self::TAG, self.0, Self::TAG)
        }

        #[cfg(unix)]
        {
            write!(f, "{}{}", Self::TAG, self.0)
        }
    }
}

impl Variable {
    #[cfg(windows)]
    const TAG: char = '%';
    #[cfg(unix)]
    const TAG: char = '$';
}

impl FromStr for Variable {
    type Err = ParseVariableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let first = match chars.next() {
            Some(first) => first,
            None => Err(ParseVariableError::Length)?,
        };

        if first != Self::TAG {
            Err(ParseVariableError::StartChar)?
        }

        #[cfg(windows)]
        {
            let last = match chars.last() {
                Some(last) => last,
                None => Err(ParseVariableError::Length)?,
            };

            if last != Self::TAG {
                Err(ParseVariableError::EndChar)?
            }
        }

        Ok(Self(s[1..s.len() - 1].to_owned()))
    }
}

#[derive(Debug, Error)]
pub enum ParseVariableError {
    #[cfg(unix)]
    #[error("An environment variable name must be at least 2 characters long")]
    Length,
    #[cfg(windows)]
    #[error("An environment variable name must be at least 3 characters long")]
    Length,
    #[error("An environment variable name must start with {}", Variable::TAG)]
    StartChar,
    #[cfg(windows)]
    #[error("An environment variable name must end with {}", Variable::TAG)]
    EndChar,
}

#[cfg(test)]
mod test {
    use super::Variable;
    use std::str::FromStr;

    #[cfg(windows)]
    const VALID_VARIABLE: &str = "%VARIABLE%";
    #[cfg(unix)]
    const VALID_VARIABLE: &str = "$VARIABLE";
    const INVALID_VARIABLE_NO_TAG: &str = "VARIABLE";

    const EMPTY_STRING: &str = "";
    #[cfg(windows)]
    const MISSING_FINAL_TAG: &str = "%VARIABLE";

    #[cfg(unix)]
    const WIDE_CHARS: &str = "$変数";
    #[cfg(windows)]
    const WIDE_CHARS: &str = "%変数%";

    #[test]
    fn parses_valid_variable() {
        let result = Variable::from_str(VALID_VARIABLE);
        let result = result.expect("Should have succeeded");
        assert_eq!(result.0, "VARIABLE");
    }

    #[test]
    fn parses_wide_chars() {
        let result = Variable::from_str(WIDE_CHARS);
        let result = result.expect("Should have succeeded");
        assert_eq!(result.0, "変数");
    }

    #[test]
    fn fails_parse_invalid_var_no_tag() {
        let result = Variable::from_str(INVALID_VARIABLE_NO_TAG);

        let err = match result {
            Ok(_) => panic!("Should not have succeeded"),
            Err(err) => err,
        };

        match err {
            super::ParseVariableError::StartChar => (),
            other => panic!("Should have got InvalidStartChar but got: {other}"),
        }
    }

    #[test]
    fn empty_string_fails() {
        let result = Variable::from_str(EMPTY_STRING);

        let err = match result {
            Ok(ok) => panic!("Should have failed but got {ok}"),
            Err(err) => err,
        };

        match err {
            super::ParseVariableError::Length => (),
            other => panic!("Should have got Length error but got; {other}"),
        }
    }

    #[cfg(windows)]
    #[test]
    fn missing_ending_tag_fails() {
        let result = Variable::from_str(MISSING_FINAL_TAG);

        let err = match result {
            Ok(ok) => panic!("Should have failed but got {ok}"),
            Err(err) => err,
        };

        match err {
            super::ParseVariableError::EndChar => (),
            other => panic!("Should have got Length error but got; {other}"),
        }
    }
}
