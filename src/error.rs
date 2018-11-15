use auto_from::auto_from;
use crate::export::ExportError;
use failure_derive::Fail;
use std;

pub type Result<T> = std::result::Result<T, Error>;

/// immense Error type.
#[auto_from]
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Error exporting mesh.")]
    Export(ExportError),
}
