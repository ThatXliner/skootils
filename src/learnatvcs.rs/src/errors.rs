use reqwest::Error as ReqwestError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum LearnAtVcsError {
    #[error("No lesson plans could be found")]
    NoLessonPlans,
    #[error("No lesson plans could be found")]
    InvalidQuarter,
    #[error("Network error")]
    ReqwestError(ReqwestError),
    #[error("Page structure had changed")]
    StructureChanged,
}
