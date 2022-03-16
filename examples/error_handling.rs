#[tokio::main]
async fn main() {}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use anyhow::{Context, Result};
    use thiserror::Error;

    #[test]
    pub fn test_anyhow_context() -> Result<()> {
        let path = "invalid";
        let content =
            std::fs::read(path).with_context(|| format!("failed to read instrs from {}", path));

        assert_eq!(content.is_err(), true);
        if let Err(e) = content {
            println!(
                "error source is {:?} \nerror message is {:?}",
                e.source(),
                e.to_string()
            );

            println!("the error type is {}", e);
            Ok(())
        } else {
            Ok(())
        }
    }

    #[test]
    pub fn test_without_anyhow_context() -> Result<()> {
        let path = "invalid";
        let content = std::fs::read(path);

        assert_eq!(content.is_err(), true);
        match content {
            Ok(_) => Ok(()),
            Err(msg) => {
                println!("error source is {:?}", msg);
                Ok(())
            }
        }
    }

    #[derive(Error, Debug)]
    pub enum FormatError {
        #[error("Invalid header (expected {expected:?}, got {found:?})")]
        InvalidHeader { expected: String, found: String },

        #[error("Missing attribute: {0}")]
        MissingAttribute(String),
    }

    fn get_missing_attr_error() -> Result<(), FormatError> {
        Err(FormatError::MissingAttribute("test".to_string()))
    }

    fn get_invalid_header_error() -> Result<(), FormatError> {
        Err(FormatError::InvalidHeader {
            expected: "expect true".to_string(),
            found: "found false".to_string(),
        })
    }

    #[test]
    pub fn test_this_error() -> Result<(), FormatError> {
        match get_missing_attr_error() {
            Ok(_) => {}
            Err(e) => {
                println!("error source is {:?} ", e);
            }
        }

        match get_invalid_header_error() {
            Ok(_) => Ok(()),
            Err(e) => {
                // This is a formatError;
                println!("error source is {:?} ", e);
                Ok(())
            }
        }
    }

    #[test]
    pub fn test_anyhow_with_thiserror() -> Result<(), FormatError> {
        let err =
            get_missing_attr_error().with_context(|| format!("a custom missing attribute error"));

        if let Err(e) = err {
            println!(
                "error source is {:?} \nerror message is {:?}",
                e.source(),
                e.to_string()
            );
            Ok(())
        } else {
            Ok(())
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum CustomError {
        #[error("io error: {0}")]
        IOError(#[from] std::io::Error),

        #[error("db error on table {table:?}, row {row:?})")]
        DBError {
            table: String,
            row: String,
            #[source]
            source: anyhow::Error,
        },
    }

    pub fn make_io_error() -> Result<(), CustomError> {
        Ok(std::fs::remove_file("/this/file/does/not/exist")?)
    }

    pub fn make_db_error() -> Result<(), CustomError> {
        Err(CustomError::DBError {
            table: "account".to_string(),
            row: "name".to_string(),
            source: anyhow::anyhow!("test"),
        })
    }

    #[test]
    pub fn test_io_error() -> Result<(), CustomError> {
        match make_io_error() {
            Ok(_) => Ok(()),
            Err(e) => {
                // The error type is an anyhow::Error
                println!("{:?}", e.to_string());
                Ok(())
            }
        }
    }

    #[test]
    pub fn test_db_error() -> Result<(), CustomError> {
        let dbErr = make_db_error();
        if let Err(CustomError::DBError { table, row, source }) = dbErr {
            println!("{:?}, {:?}, {:?}", table, row, source);
            Ok(())
        } else {
            Ok(())
        }
    }
}
