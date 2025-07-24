use std::{env::current_exe, fs, sync::{Arc, Mutex, OnceLock}};
use ffi_convert::{CReprOf, CStringArray};
use rusqlite::{Connection, OpenFlags};
use rusqlite_migration::{Migrations, M};

const DRUGS_MIGRATION_SLICE: &[M] = &[
    M::up(include_str!("./migrations/drugs_V01.sql")),
];

const DRUGS_MIGRATIONS: Migrations = Migrations::from_slice(DRUGS_MIGRATION_SLICE);

static DRUGS_DB_CONN: OnceLock<Arc<Mutex<Connection>>> = OnceLock::new();

#[allow(dead_code)]
fn get_drugs_db_connection() -> &'static Arc<Mutex<Connection>> {
    let exe_path = current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let db_dir = exe_dir.join("database");
    let db_path = db_dir.join("drugs.db");
    
    if !db_path.exists() {
        println!("Initializing drugs database... Please wait...");
        fs::create_dir_all(&db_dir).expect("Error creating directory");
        fs::File::create(&db_path).expect("Error creating file");
    }

    // apply initial migrations
    let mut conn = Connection::open(&db_path).unwrap();
    conn.pragma_update(None, "foreign_keys", "on").unwrap();
    conn.pragma_update(None, "journal_mode", "wal").unwrap();
    DRUGS_MIGRATIONS.to_latest(&mut conn).unwrap();
    conn.close().unwrap();

    // open the database read only
    let conn = Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
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
