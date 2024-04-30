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

macro_rules! fail {
    ($fmt:expr) => {
        return Err(Error::create(&format!($fmt)))
    };
    ($fmt:expr, $($args:expr),*) => {
        return Err(Error::create(&format!($fmt, $($args),*)))
    };
    ($fmt:expr, $($args:expr),+ ,) => {
        fail!($fmt, $($args),*)
    }
}
