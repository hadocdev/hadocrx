use std::sync::{Arc, Mutex, OnceLock};
use ffi_convert::{CReprOf, CStringArray};
use rusqlite::{Connection, OpenFlags};

const DRUGS_DB_PATH: &str = "drugs.db";
static DRUGS_DB_CONN: OnceLock<Arc<Mutex<Connection>>> = OnceLock::new();

#[allow(dead_code)]
fn get_drugs_db_connection() -> &'static Arc<Mutex<Connection>> {
    let conn = Connection::open_with_flags(DRUGS_DB_PATH, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    DRUGS_DB_CONN.get_or_init(|| Arc::new(Mutex::new(conn)))
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn get_generics() -> CStringArray {
    let conn_arc_mutex = get_drugs_db_connection();
    let conn_guard = conn_arc_mutex.lock().unwrap();
    let mut stmt = conn_guard.prepare("SELECT name FROM Generics ORDER BY name").unwrap();
    let rows = stmt.query_map([], |row| row.get::<usize, String>(0)).unwrap(); 
    let generics: Vec<String> = rows.map(|row| row.unwrap()).collect();
    CStringArray::c_repr_of(generics).unwrap()
}
