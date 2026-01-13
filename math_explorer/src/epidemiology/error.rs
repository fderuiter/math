use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EpidemiologyError {
    #[error("Matrix V is singular")]
    SingularMatrixV,
}
