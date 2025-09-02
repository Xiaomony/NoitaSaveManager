use serde_json;

#[derive(Debug)]
enum ErrorType {
    General,
    Io(std::io::Error),
    Serialize(serde_json::Error),
}

#[derive(Debug)]
pub struct NAchError {
    m_explanation: String,
    m_err_type: ErrorType,
    m_isfatal: bool,
}

impl std::error::Error for NAchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ErrorType::*;
        match self.m_err_type {
            General => None,
            Io(ref err) => Some(err),
            Serialize(ref err) => Some(err),
        }
    }
}

impl NAchError {
    pub fn new(explanation: &str) -> Self {
        Self {
            m_explanation: explanation.to_string(),
            m_err_type: ErrorType::General,
            m_isfatal: false,
        }
    }
    pub fn newfatal(explanation: &str) -> Self {
        Self {
            m_explanation: explanation.to_string(),
            m_err_type: ErrorType::General,
            m_isfatal: true,
        }
    }
    #[inline]
    pub fn fatal(&mut self) {
        self.m_isfatal = true;
    }
}

// -------------------------- `Result` expansion -----------------------------
#[inline]
pub fn throw<T>(explanation: &str) -> Result<T, NAchError> {
    Err(NAchError::new(explanation))
}

#[inline]
pub fn throwfatal<T>(explanation: &str) -> Result<T, NAchError> {
    Err(NAchError::newfatal(explanation))
}

pub trait ResultExt<T> {
    fn explain_isfatal(self, explanation: &str, is_fatal: bool) -> Result<T, NAchError>;
    fn explain(self, explanation: &str) -> Result<T, NAchError>;
    fn explain_fatal(self, explanation: &str) -> Result<T, NAchError>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<NAchError>,
{
    fn explain_isfatal(self, explanation: &str, is_fatal: bool) -> Result<T, NAchError> {
        self.map_err(|err: E| {
            let mut new_err: NAchError = err.into();
            new_err.m_explanation.push_str(explanation);
            if is_fatal {
                new_err.fatal();
            }
            new_err
        })
    }

    #[inline]
    fn explain(self, explanation: &str) -> Result<T, NAchError> {
        self.explain_isfatal(explanation, false)
    }

    #[inline]
    fn explain_fatal(self, explanation: &str) -> Result<T, NAchError> {
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
        };
        write!(f, "{}", msg)
    }
}

impl std::fmt::Display for NAchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} :  {}", self.m_err_type, self.m_explanation)
    }
}

// ------------------- Type Convertion from other Error Types ----------------
impl From<std::io::Error> for NAchError {
    fn from(value: std::io::Error) -> Self {
        NAchError {
            m_explanation: String::new(),
            m_err_type: ErrorType::Io(value),
            m_isfatal: false,
        }
    }
}

impl From<serde_json::Error> for NAchError {
    fn from(value: serde_json::Error) -> Self {
        NAchError {
            m_explanation: String::new(),
            m_err_type: ErrorType::Serialize(value),
            m_isfatal: false,
        }
    }
}
