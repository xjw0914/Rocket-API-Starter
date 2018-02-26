use diesel;
use diesel::mysql::MysqlConnection;
use diesel::RunQueryDsl;

use super::schema::todos;

pub struct Todo {}

#[table_name = "todos"]
#[derive(Debug, Insertable, Serialize, Deserialize)]
pub struct NewTodo {
    pub description: String,
}

impl Todo {
    pub fn insert(description: String, conn: &MysqlConnection) -> bool {
        let t = NewTodo {
            description: description,
        };
        diesel::insert_into(todos::table)
            .values(&t)
            .execute(conn)
            .is_ok()
    }
}
