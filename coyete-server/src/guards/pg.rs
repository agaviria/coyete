use coyete_data::persistance::{PgConn, PgPool, Error};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{State, Request, Outcome};

// PgConn rocket request guard attempts to retrieve a single connection from the
// managed pool.  If no pool is currently managed, fails with an
// `InternalServerError` status.  If no connections are available, fails with a
// `ServiceUnavailable` status and GetTimeout or InitializationError.
impl<'a, 'r> FromRequest<'a, 'r> for PgConn {
    type Error = Error;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<PgConn, Self::Error> {
        let pool = match <State<PgPool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => Outcome::Failure(e),
            Outcome::Forward(_) => Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(PgConn(conn)),
            Err(e) => Outcome::Failure((Status::ServiceUnavailable, e)),
        }
    }
}
