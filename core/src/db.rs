use std::{cell::RefCell, env::current_exe, ffi::{c_char, CStr, CString}, fs, sync::{Arc, Mutex, OnceLock}};
use ffi_convert::{CReprOf, CStringArray};
use rusqlite::{Connection, OpenFlags};
use rusqlite_migration::{Migrations, M};

const DRUGS_MIGRATION_SLICE: &[M] = &[
    M::up(include_str!("./migrations/drugs_V01.sql")),
];

const DRUGS_MIGRATIONS: Migrations = Migrations::from_slice(DRUGS_MIGRATION_SLICE);

static DRUGS_DB_CONN: OnceLock<Arc<Mutex<Connection>>> = OnceLock::new();

thread_local! {
    static LAST_GENERIC_NAME: RefCell<Option<CString>> = RefCell::new(None);
    static LAST_BRAND_NAME: RefCell<Option<CString>> = RefCell::new(None);
    static LAST_MANUFACTURER_NAME: RefCell<Option<CString>> = RefCell::new(None);
}

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
pub extern "C" fn get_generic_names() -> CStringArray {
    let conn_arc_mutex = get_drugs_db_connection();
    let conn_guard = conn_arc_mutex.lock().unwrap();
    let mut stmt = conn_guard.prepare("SELECT DISTINCT name FROM Generics ORDER BY name").unwrap();
    let rows = stmt.query_map([], |row| row.get::<usize, Option<String>>(0)).unwrap(); 
    let generic_names: Vec<String> = rows.map(|row| row.unwrap().unwrap_or_default()).collect();
    CStringArray::c_repr_of(generic_names).unwrap()
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn get_generic_name_by_brand_name(brand_name: *const c_char) -> *const c_char {
    let brand_name_str = unsafe { CStr::from_ptr(brand_name).to_str().unwrap() };
    let conn_arc_mutex = get_drugs_db_connection();
    let conn_guard = conn_arc_mutex.lock().unwrap();
    let mut stmt = conn_guard.prepare("SELECT Generics.name FROM Drugs JOIN Generics ON Generics.id == Drugs.generic_id WHERE Drugs.brand_name = ?1").unwrap();
    let mut rows = stmt.query_map([brand_name_str], |row| row.get::<usize, String>(0)).unwrap();
    let generic_name = rows.next().unwrap().unwrap_or_default();
    let cstring = CString::c_repr_of(generic_name).unwrap();
    let ptr = cstring.as_ptr();

    LAST_GENERIC_NAME.with(|last| {
        *last.borrow_mut() = Some(cstring);
    });
    ptr
}


#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn get_manufacturer_by_brand_name(brand_name: *const c_char) -> *const c_char {
    let brand_name_str = unsafe { CStr::from_ptr(brand_name).to_str().unwrap() };
    let conn_arc_mutex = get_drugs_db_connection();
    let conn_guard = conn_arc_mutex.lock().unwrap();
    let mut stmt = conn_guard.prepare("
        SELECT m.name FROM Drugs d
        JOIN Manufacturers m ON m.id == d.manufacturer_id
        WHERE d.brand_name = ?1
    ").unwrap();
    let mut rows = stmt.query_map([brand_name_str], |row| row.get::<usize, String>(0)).unwrap();
    let manufacturer_name = rows.next().unwrap().unwrap_or_default();
    
    let cstring = CString::c_repr_of(manufacturer_name).unwrap();
    let ptr = cstring.as_ptr();

    LAST_MANUFACTURER_NAME.with(|last| {
        *last.borrow_mut() = Some(cstring);
    });
    ptr
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn get_brand_name_by_generic_name_manufacturer_and_strength(
    generic_name: *const c_char, manufacturer: *const c_char, strength: *const c_char
) -> *const c_char {
    let generic_name_str = unsafe { CStr::from_ptr(generic_name).to_str().unwrap() };
    let manufacturer_str = unsafe { CStr::from_ptr(manufacturer).to_str().unwrap() };
    let strength_str = unsafe { CStr::from_ptr(strength).to_str().unwrap() };
    
    let conn_arc_mutex = get_drugs_db_connection();
    let conn_guard = conn_arc_mutex.lock().unwrap();
    let mut stmt = conn_guard.prepare("
        SELECT d.brand_name FROM Drugs d 
        JOIN Generics g ON g.id == d.generic_id 
        JOIN Manufacturers m ON m.id == d.manufacturer_id
        JOIN Strengths s ON s.id == d.strength_id
        WHERE g.name = ?1 AND m.name = ?2 AND s.value = ?3
    ").unwrap();
    let mut rows = stmt.query_map([generic_name_str, manufacturer_str, strength_str], |row| row.get::<usize, String>(0)).unwrap();
    if let Some(row) = rows.next() {
        let brand_name = row.unwrap_or_default();
        let cstring = CString::c_repr_of(brand_name).unwrap();
        let ptr = cstring.as_ptr();

        LAST_BRAND_NAME.with(|last| {
            *last.borrow_mut() = Some(cstring);
        });
        ptr
    } else {
        std::ptr::null()
    }
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn get_brand_name_by_generic_name_and_manufacturer(
    generic_name: *const c_char, manufacturer: *const c_char
) -> *const c_char {
    let generic_name_str = unsafe { CStr::from_ptr(generic_name).to_str().unwrap() };
    let manufacturer_str = unsafe { CStr::from_ptr(manufacturer).to_str().unwrap() };
    
    let conn_arc_mutex = get_drugs_db_connection();
    let conn_guard = conn_arc_mutex.lock().unwrap();
    let mut stmt = conn_guard.prepare("
        SELECT d.brand_name FROM Drugs d 
        JOIN Generics g ON g.id == d.generic_id 
        JOIN Manufacturers m ON m.id == d.manufacturer_id
        WHERE g.name = ?1 AND m.name = ?2
    ").unwrap();
    let mut rows = stmt.query_map([generic_name_str, manufacturer_str], |row| row.get::<usize, String>(0)).unwrap();
    let brand_name = rows.next().unwrap().unwrap_or_default();
    
    let cstring = CString::c_repr_of(brand_name).unwrap();
    let ptr = cstring.as_ptr();

    LAST_BRAND_NAME.with(|last| {
        *last.borrow_mut() = Some(cstring);
    });
    ptr
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn get_strengths_by_generic_name(generic_name: *const c_char) -> CStringArray {
    let generic_name_str = unsafe { CStr::from_ptr(generic_name).to_str().unwrap() };
    let conn_arc_mutex = get_drugs_db_connection();
    let conn_guard = conn_arc_mutex.lock().unwrap();
    let mut stmt = conn_guard.prepare(
        "SELECT DISTINCT s.value FROM Drugs d JOIN Generics g ON g.id == d.generic_id JOIN Strengths s ON s.id == d.strength_id WHERE g.name = ?1"
    ).unwrap();
    let rows = stmt.query_map([generic_name_str], |row| row.get::<usize, Option<String>>(0)).unwrap(); 
    let strengths: Vec<String> = rows.map(|row| row.unwrap().unwrap_or_default()).collect();
    CStringArray::c_repr_of(strengths).unwrap()
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn get_formulations_by_brand_name_and_strength(brand_name: *const c_char, strength: *const c_char) -> CStringArray {
    let brand_name_str = unsafe { CStr::from_ptr(brand_name).to_str().unwrap() };
    let strength_str = unsafe { CStr::from_ptr(strength).to_str().unwrap() };
    let conn_arc_mutex = get_drugs_db_connection();
    let conn_guard = conn_arc_mutex.lock().unwrap();
    let mut stmt = conn_guard.prepare("
        SELECT DISTINCT do.value FROM Drugs d
        JOIN Strengths s ON s.id == d.strength_id
        JOIN Dosages do ON do.id == d.dosage_id 
        WHERE d.brand_name = ?1 AND s.value = ?2
    ").unwrap();
    let rows = stmt.query_map([brand_name_str, strength_str], |row| row.get::<usize, Option<String>>(0)).unwrap(); 
    let formulations: Vec<String> = rows.map(|row| row.unwrap().unwrap_or_default()).collect();
    CStringArray::c_repr_of(formulations).unwrap()
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn get_brand_names() -> CStringArray {
    let conn_arc_mutex = get_drugs_db_connection();
    let conn_guard = conn_arc_mutex.lock().unwrap();
    let mut stmt = conn_guard.prepare("SELECT DISTINCT brand_name FROM Drugs").unwrap();
    let rows = stmt.query_map([], |row| row.get::<usize, Option<String>>(0)).unwrap(); 
    let brand_names: Vec<String> = rows.map(|row| row.unwrap().unwrap_or_default()).collect();
    CStringArray::c_repr_of(brand_names).unwrap()
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn get_manufacturers() -> CStringArray {
    let conn_arc_mutex = get_drugs_db_connection();
    let conn_guard = conn_arc_mutex.lock().unwrap();
    let mut stmt = conn_guard.prepare("SELECT DISTINCT name FROM Manufacturers ORDER BY name").unwrap();
    let rows = stmt.query_map([], |row| row.get::<usize, Option<String>>(0)).unwrap(); 
    let brand_names: Vec<String> = rows.map(|row| row.unwrap().unwrap_or_default()).collect();
    CStringArray::c_repr_of(brand_names).unwrap()
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn get_manufacturers_by_generic_name(generic_name: *const c_char) -> CStringArray {
    let generic_name_str = unsafe { CStr::from_ptr(generic_name).to_str().unwrap() };
    let conn_arc_mutex = get_drugs_db_connection();
    let conn_guard = conn_arc_mutex.lock().unwrap();
    let mut stmt = conn_guard.prepare("
        SELECT DISTINCT m.name FROM Drugs d
        JOIN Manufacturers m ON m.id == d.manufacturer_id
        JOIN Generics g ON g.id == d.generic_id
        WHERE g.name = ?1
        ORDER BY m.name
    ").unwrap();
    let rows = stmt.query_map([generic_name_str], |row| row.get::<usize, Option<String>>(0)).unwrap(); 
    let brand_names: Vec<String> = rows.map(|row| row.unwrap().unwrap_or_default()).collect();
    CStringArray::c_repr_of(brand_names).unwrap()
}
