use uuid::Uuid;
use schema::certification_codes;
use users::User;

#[derive(Insertable)]
#[table_name = "certification_codes"]
pub struct NewCertificationCode<'a> {
    code: &'a str,
    user_id: Uuid,
}

#[derive(Debug, Queryable, Associations, Identifiable)]
#[belongs_to(User)]
pub struct CertificationCode {
    id: i32,
    code: String,
    user_id: Uuid,
}
