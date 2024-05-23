use supr::fail;
use supr::util;
use supr::util::{Error, Result};

#[test]
fn test_util_error_create() -> Result<()> {
    let lambda = || -> Result<()> { Err(Error::create("failure")) };

    match lambda() {
        Ok(()) => Err(Box::new(Error::new("Expected Err"))),
        Err(_) => Ok(()),
    }
}

#[test]
fn test_util_fail() -> Result<()> {
    let lambda = || -> Result<()> {
        fail!("failure");
    };

    match lambda() {
        Ok(()) => Err(Error::create("Expected Err")),
        Err(err) => {
            let msg = format!("{}", err);
            if msg != "Error: failure" {
                Err(Error::create("Expected 'Error: failure' message"))
            } else {
                Ok(())
            }
        }
    }
}
