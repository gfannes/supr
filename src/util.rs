pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Error: {}", &self.msg)?;
        Ok(())
    }
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            msg: msg.to_string(),
        }
    }
    pub fn create(msg: &str) -> Box<Error> {
        Box::new(Error::new(msg))
    }
}

#[macro_export]
macro_rules! fail {
    ($fmt:expr) => {
        return Err(crate::util::Error::create(&format!($fmt)))
    };
    ($fmt:expr, $($args:expr),*) => {
        return Err(crate::util::Error::create(&format!($fmt, $($args),*)))
    };
    ($fmt:expr, $($args:expr),+ ,) => {
        fail!($fmt, $($args),*)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fail() -> Result<()> {
        let lambda = || -> Result<()> {
            fail!("failure");
        };
        match lambda() {
            Ok(()) => Err(Error::create("Expected Err")),
            Err(_) => Ok(()),
        }
    }
}
