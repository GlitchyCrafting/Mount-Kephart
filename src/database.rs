pub mod structs {
    // To store lesson data
    pub struct Lesson {
        pub name: String,
        pub content: String,
        pub code: String,
        pub answer: String,
    }
}

pub mod functions {
    use rusqlite::{Connection, Result};

    use crate::database::structs::Lesson;

    pub fn get_lesson(id: i64) -> Result<Lesson, rusqlite::Error> {
        // Open a database connection
        let database = Connection::open("db.sqlite")?;

        // Query the database
        database.query_row(
                "SELECT name,content,code,answer FROM lesson WHERE _id = ?",
                [id], |row| {
                    Ok(Lesson {
                        name: row.get(0)?,
                        content: row.get(1)?,
                        code: row.get(2)?,
                        answer: row.get(3)?
                    })})
    }
}
