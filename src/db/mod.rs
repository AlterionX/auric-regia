mod navy;
mod legion;
mod industry;
mod event;

use diesel::{Connection, PgConnection};
pub use navy::*;
pub use legion::*;
pub use industry::*;
pub use event::*;

pub trait Connector {
    fn connect(&self) -> PgConnection;
}

impl Connector for &super::DatabaseConfiguration {
    fn connect(&self) -> PgConnection {
        diesel::pg::PgConnection::establish(self.url.as_str()).expect("DB to connect")
    }
}
