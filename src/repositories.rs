use super::models::*;
use super::schema::*;
use diesel::prelude::*;
use diesel::result::QueryResult;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn load_all(c: &MysqlConnection) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(100).load::<Rustacean>(c)
    }

    pub fn find(c: &MysqlConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn create(c: &MysqlConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(c)?;

        let last_id = Self::last_id(c)?;

        Self::find(c, last_id)
    }

    pub fn update(c: &MysqlConnection, rustacean: UpdatedRustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(rustacean.id))
            .set((
                rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(c)?;

        Self::find(c, rustacean.id)
    }

    pub fn delete(c: &MysqlConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }

    fn last_id(c: &MysqlConnection) -> QueryResult<i32> {
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(c)
    }
}
