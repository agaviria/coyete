use std::collections::HashMap;

use validator::Validate;
use rocket_contrib::{Json, SerdeError};

use response::{APIResponse, bad_request};

pub fn validate_json<T: Validate>(req_data: Result<Json<T>, SerdeError>)
-> Result<Json<T>, APIResponse> {
    // Check for JSON decoding error
    match req_data {
        // Json successfully decoded, now check for validation errors.
        Ok(data) => {
            // Check for validation Error
            match data.validate() {
                Err(validation_errors) => {
                    let mut formatted_errors = HashMap::new();
                    let default_message = "No explicit error message";
                    for (name, errors) in validation_errors.inner() {
                        let mut error_string = String::new();
                        for error in errors.iter() {
                            match error.message {
                                Some(ref message) => {
                                    error_string.push_str(message.as_ref());
                                }
                                None => error_string.push_str(message.as_ref());
                            }
                            error_string.push('\n');
                        }
                        formatted_errors.insert(name, error_string);
                    }
                    return Err(bad_request().errors(formatted_errors));
                }
                // No validation error found, return data.
                Ok(_) => return Ok(data),
            }
        }
        // Json decoding error, return error message.
        Err(json_error) => return Err(bad_request().error("json", json_error.to_string().as_str())),
    }
}
