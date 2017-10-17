// impl CertificationCode {
//     /// Creates a certification code for an email parameter.
//     pub fn create_by_email(conn: &PgConnection, email: &str) -> Result<Self, Error> {
//         let new_cert_stamp = NewCertificationCode::new_by_email(conn, email)?;

//         new_cert_stamp.save()
//     }

//     /// Creates a certification code for a user id.
//     pub fn create_by_id(conn: &PgConnection, user_uuid: Uuid) -> Result<Self, Error> {
//         let new_cert_stamp = NewCertificationCode::new_by_id(conn, user_uuid)?;

//         new_cert_stamp.save()
//     }

//     /// Deletes certification code for a user id.
//     pub fn delete_by_user_id(conn: &PgConnection, user_uuid: Uuid) -> Result<(), Error> {
//         use diesel::FilterDsl;
//         use diesel::ExpressionMethods;
//         use schema::certification_codes::dsl::*;

//         let _ = diesel::delete(certification_codes.filter(user_id.eq(user_uuid))).execute(conn)?;

//         Ok(())
//     }

//     /// search user by Uuid
//     pub fn find_by_user_id(conn: &PgConnection, user_uuid: Uuid) -> Result<Self, Error> {
//         use diesel::FilterDsl;
//         use diesel::ExpressionMethods;
//         use schema::certification_codes::dsl::*;

//         debug!("Querying user id: [ {} ] from database", user_uuid);
//         let certification = certification_codes.filter(user_id.eq(user_uuid))
//             .first::<CertificationCode>(conn)?;

//         Ok(certification)
//     }

//     pub fn id(&self) -> i32 {
//         self.id
//     }

//     pub fn verification_code(&self) -> &str {
//         &self.code
//     }

//     pub fn user_id(&self) -> &Uuid {
//         &self.user_id
//     }
// }




// impl NewCertificationCode {
//     pub fn new_by_email(conn: &PgConnection, user_email: &str) -> Result<Self, Error> {
//         use diesel::FilterDsl;
//         use diesel::ExpressionMethods;
//         use users::User;
//         use schema::users::dsl;

//         let user: User = dsl::users.filter(dsl::email.eq(user_email))
//             .first::<User>(conn)?;

//         Self::new(id(&user))
//     }

//     pub fn new(user_id: Uuid) -> Result<Self, Error> {
//         use rand::{self, Rng};

//         Ok(NewCertificationCode {
//                code: rand::thread_rng()
//                    .gen_ascii_chars()
//                    .take(24)
//                    .collect::<String>(),
//                user_id: user_id,
//            })
//     }

//     pub fn save(&self, conn: &PgConnection) -> Result<CertificationCode, Error> {
//         use schema::certification_codes;

//         let new_code = NewCertificationCode {
//             code: self.code,
//             user_id: self.user_id,
//         };

//         let certification_code = diesel::insert(&new_code).into(certification_codes::table)
//             .get_result(conn)?;

//         Ok(certification_code)
//     }
// }
