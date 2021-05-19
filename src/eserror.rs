use hirofa_utils::js_utils::JsError;
use std::fmt::{Error, Formatter};

/// The EsError struct is used throughout this crate to represent errors

pub struct EsError {
    name: String,
    message: String,
    stack: String,
}

impl EsError {
    pub fn new(name: String, message: String, stack: String) -> Self {
        Self {
            name,
            message,
            stack,
        }
    }
    pub fn new_str(err: &str) -> Self {
        Self::new_string(err.to_string())
    }
    pub fn new_string(err: String) -> Self {
        EsError {
            name: "".to_string(),
            message: err,
            stack: "".to_string(),
        }
    }
    pub fn get_message(&self) -> &str {
        self.message.as_str()
    }
    pub fn get_stack(&self) -> &str {
        self.stack.as_str()
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl From<JsError> for EsError {
    fn from(js_error: JsError) -> Self {
        EsError {
            name: js_error.get_name().to_string(),
            message: js_error.get_message().to_string(),
            stack: js_error.get_stack().to_string(),
        }
    }
}

impl From<EsError> for JsError {
    fn from(es_error: EsError) -> Self {
        JsError::new(
            es_error.get_name().to_string(),
            es_error.get_message().to_string(),
            es_error.get_stack().to_string(),
        )
    }
}

impl std::fmt::Display for EsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let e = format!("{}: {} at{}", self.name, self.message, self.stack);
        f.write_str(e.as_str())
    }
}
