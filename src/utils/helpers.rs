use serde_json::{json, Value};
use std::borrow::Cow;
use validator::ValidationErrors;

pub fn format_validation_errors(errors: ValidationErrors) -> Value {
    let formatted_errors = errors
        .field_errors()
        .iter()
        .map(|(field, errs)| {
            let message = errs
                .iter()
                .next() // Take the first error message
                .map(|e| e.message.clone().unwrap_or(Cow::Borrowed("Invalid value")))
                .unwrap_or(Cow::Borrowed("Invalid value"))
                .to_string();
            (field.to_string(), json!(message))
        })
        .collect::<serde_json::Map<String, Value>>();

    json!({ "errors": formatted_errors })
}