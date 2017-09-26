use std::io::Cursor;
use std::convert::From;
use rocket_contrib::Value;
use rocket::request::Request;
use rocket::response::{Response, Responder};
use rocket::http::{ContentType, RawStr, Status};
use diesel::result::Error as DieselError;
use rwt::RwtError;
use service;

#[derive(Debug)]
pub struct APIResponse {
    data: Value,
    status: Status,
}

impl APIResponse {
    /// Convenience method to set `self.data` to `{"message": message}`
    pub fn message(mut self, message: &str) -> APIResponse {
        self.message = json!({
            "message": message
        });
        self
    }

    pub fn data(mut self, data: Value) -> APIResponse {
        self.data = data;
        self
    }
}

impl<'r> Responder<'r> for APIResponse {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        let body = match (self.data);

        Response::build()
            .status(self.status)
            .sized_body(Cursor::new(body.to_string()))
            .header(ContentType::JSON)
            .ok()
    }
}

impl From<DieselError> for APIResponse {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => not_found(),
            _ => internal_server_error(),
        }
    }
}

impl From<RwtError> for APIResponse {
    fn from(_: RwtError) -> Self {
        bad_token()
    }
}

pub fn ok() -> APIResponse {
    APIResponse {
        message: json!(null),
        status: Status::Ok,
    }
}

pub fn created() -> APIResponse {
    APIResponse {
        message: json!(null),
        status: Status::Created,
    }
}

pub fn accepted() -> APIResponse {
    APIResponse {
        message: json!(null),
        status: Status::Accepted,
    }
}

pub fn no_content() -> APIResponse {
    APIResponse {
        message: json!(null),
        status: Status::NoContent,
    }
}

pub fn bad_request() -> APIResponse {
    APIResponse {
        message: json!({"message": "Bad Request"}),
        status: Status::BadRequest,
    }
}

pub fn bad_token() -> APIResponse {
    APIResponse {
        message: json!({"message": "Bad Token"}),
        status: Status::BadRequest,
    }
}

pub fn unauthorized() -> APIResponse {
    APIResponse {
        message: json!({"message": "Unauthorized"}),
        status: Status::Unauthorized,
    }
}

pub fn forbidden() -> APIResponse {
    APIResponse {
        message: json!({"message": "Forbidden"}),
        status: Status::Forbidden,
    }
}

pub fn not_found() -> APIResponse {
    APIResponse {
        message: json!({"message": "Not Found"}),
        status: Status::NotFound,
    }
}

pub fn method_not_allowed() -> APIResponse {
    APIResponse {
        message: json!({"message": "Method Not Allowed"}),
        status: Status::MethodNotAllowed,
    }
}

pub fn conflict() -> APIResponse {
    APIResponse {
        message: json!({"message": "Conflict"}),
        status: Status::Conflict,
    }
}

pub fn unprocessable_entity(errors: Value) -> APIResponse {
    APIResponse {
        message: json!({"message": errors}),
        status: Status::UnprocessableEntity,
    }
}

pub fn internal_server_error() -> APIResponse {
    APIResponse {
        message: json!({"message": "Internal Server Error"}),
        status: Status::InternalServerError,
    }
}

pub fn service_unavailable() -> APIResponse {
    APIResponse {
        message: json!({"message": "Service Unavailable"}),
        status: Status::ServiceUnavailable,
    }
}

pub fn expired_token() -> APIResponse {
    APIResponse {
        message: json!({"message": "Expired Auth Token"}),
        status: Status::Unauthorized,
    }
}

pub struct Identifier(i64);

impl Deref for Identifier {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FromParam<'a> for Identifier {
    type Error = APIResponse;

    fn from_param(param: &'a RawStr) -> Result<Self> {
        match service::harsh().decode(param) {
            Some(ref mut x) if !x.is_empty() => {
                let id = x.pop().unwrap();
                Ok(Identifier(id as i64))
            },

            _ => Err(APIResponse {
                data: json!({"message": "`{}` does not map to a valid identifier", param}),
                status: Status::UnprocessableEntity,
            })
        }
    }
}
