use std::sync::PoisonError;

use serde_json;

pub type NSComResult = Result<(), NSError>;
pub type NSResult<T> = Result<T, NSError>;

#[derive(Debug)]
enum ErrorType {
    General,
    Io(std::io::Error),
    Serialize(serde_json::Error),
    Mutex(String),
    Regex(regex::Error),
}

#[derive(Debug)]
pub struct NSError {
    m_explanation: Vec<String>,
    m_err_type: ErrorType,
    m_isfatal: bool,
}

impl std::error::Error for NSError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ErrorType::*;
        match self.m_err_type {
            General => None,
            Io(ref err) => Some(err),
            Serialize(ref err) => Some(err),
            Mutex(..) => None,
            Regex(ref err) => Some(err),
        }
    }
}

impl NSError {
    pub fn new(explanation: &str) -> Self {
        Self {
            m_explanation: vec![explanation.to_string()],
            m_err_type: ErrorType::General,
            m_isfatal: false,
        }
    }

    pub fn newfatal(explanation: &str) -> Self {
        Self {
            m_explanation: vec![explanation.to_string()],
            m_err_type: ErrorType::General,
            m_isfatal: true,
        }
    }

    #[inline]
    pub fn fatal(&mut self) {
        self.m_isfatal = true;
    }

    #[inline]
    pub fn is_fatal(&self) -> bool {
        self.m_isfatal
    }

    #[inline]
    pub fn get_explanation(&self) -> &[String] {
        &self.m_explanation
    }
}

// -------------------------- `Result` expansion -----------------------------
#[inline]
pub fn throw<T>(explanation: &str) -> NSResult<T> {
    Err(NSError::new(explanation))
}

#[inline]
pub fn throwfatal<T>(explanation: &str) -> NSResult<T> {
    Err(NSError::newfatal(explanation))
}

pub trait ResultExt<T> {
    fn explain_isfatal(self, explanation: &str, is_fatal: bool) -> NSResult<T>;
    fn explain(self, explanation: &str) -> NSResult<T>;
    fn explain_fatal(self, explanation: &str) -> NSResult<T>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<NSError>,
{
    fn explain_isfatal(self, explanation: &str, is_fatal: bool) -> NSResult<T> {
        self.map_err(|err: E| {
            let mut new_err: NSError = err.into();
            new_err.m_explanation.push(explanation.to_string());
            if is_fatal {
                new_err.fatal();
            }
            new_err
        })
    }

    #[inline]
    fn explain(self, explanation: &str) -> NSResult<T> {
        self.explain_isfatal(explanation, false)
    }

    #[inline]
    fn explain_fatal(self, explanation: &str) -> NSResult<T> {
        self.explain_isfatal(explanation, true)
    }
}

// ------------------------------ Display ------------------------------------
impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorType::*;
        let msg = match self {
            General => "GeneralError",
            Io(..) => "IoError",
            Serialize(..) => "SerializeError",
            Mutex(msg) => &format!("MutexError({})", msg),
            Regex(..) => "RegexError",
        };
        write!(f, "{}", msg)
    }
}

impl std::fmt::Display for NSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.m_err_type)
    }
}

// ------------------- Type Convertion from other Error Types ----------------
impl From<std::io::Error> for NSError {
    fn from(value: std::io::Error) -> Self {
        NSError {
            m_explanation: Vec::new(),
            m_err_type: ErrorType::Io(value),
            m_isfatal: false,
        }
    }
}

impl From<serde_json::Error> for NSError {
    fn from(value: serde_json::Error) -> Self {
        NSError {
            m_explanation: Vec::new(),
            m_err_type: ErrorType::Serialize(value),
            m_isfatal: false,
        }
    }
}

impl<T> From<PoisonError<T>> for NSError {
    fn from(value: PoisonError<T>) -> Self {
        NSError {
            m_explanation: Vec::new(),
            m_err_type: ErrorType::Mutex(value.to_string()),
            m_isfatal: true,
        }
    }
}

impl From<regex::Error> for NSError {
    fn from(value: regex::Error) -> Self {
        NSError {
            m_explanation: Vec::new(),
            m_err_type: ErrorType::Regex(value),
            m_isfatal: true,
        }
    }
}
