use std::ops::Deref;

use diesel::mysql::MysqlConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

static DATABASE_URL: &'static str = dotenv!("DATABASE_URL");

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(DATABASE_URL);
    r2d2::Pool::new(manager).expect("db pool")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl Deref for DbConn {
    type Target = MysqlConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
