use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Image tags table: image_sha256_hex -> JSON-encoded Vec<String>
/// Tags are plain strings. If a tag starts with "people/c" it references
/// a Google Contact resourceName; otherwise it is a free-form tag (scene, event, etc.).
const TAGS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("tags");

/// People lookup table: resourceName (e.g. "people/c1234567890") -> display_name
/// Used to resolve people tags to human-readable names for the UI.
const PEOPLE_TABLE: TableDefinition<&str, &str> = TableDefinition::new("people");

/// Return the config directory: ~/.config/rsimagetag/
pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("could not determine config directory")
        .join("rsimagetag")
}

/// Return the database file path: ~/.config/rsimagetag/tags.redb
pub fn db_path() -> PathBuf {
    config_dir().join("tags.redb")
}

/// Check whether a tag string is a people reference.
pub fn is_person_tag(tag: &str) -> bool {
    tag.starts_with("people/c")
}

/// Initialize the database: create config dir and database file with all tables.
pub fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    let dir = config_dir();
    fs::create_dir_all(&dir)?;
    let path = db_path();
    let db = Database::create(&path)?;
    let txn = db.begin_write()?;
    {
        let _t1 = txn.open_table(TAGS_TABLE)?;
        let _t2 = txn.open_table(PEOPLE_TABLE)?;
    }
    txn.commit()?;
    println!("Database initialized at {}", path.display());
    Ok(())
}

/// Open the existing database. Returns an error if it doesn't exist.
pub fn open_db() -> Result<Database, Box<dyn std::error::Error>> {
    let path = db_path();
    if !path.exists() {
        return Err(format!(
            "Database not found at {}. Run 'rsimagetag db-init' first.",
            path.display()
        )
        .into());
    }
    Ok(Database::open(&path)?)
}

/// Compute SHA-256 hash of a file's contents, returned as a hex string.
pub fn hash_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

// ---------------------------------------------------------------------------
// People lookup table
// ---------------------------------------------------------------------------

/// Add a person to the people lookup table.
pub fn add_person(
    db: &Database,
    resource_name: &str,
    display_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let txn = db.begin_write()?;
    {
        let mut table = txn.open_table(PEOPLE_TABLE)?;
        table.insert(resource_name, display_name)?;
    }
    txn.commit()?;
    Ok(())
}

/// Remove a person from the people lookup table.
pub fn remove_person(
    db: &Database,
    resource_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let txn = db.begin_write()?;
    {
        let mut table = txn.open_table(PEOPLE_TABLE)?;
        table.remove(resource_name)?;
    }
    txn.commit()?;
    Ok(())
}

/// Get the display name for a person by resourceName.
pub fn get_person(
    db: &Database,
    resource_name: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let txn = db.begin_read()?;
    let table = txn.open_table(PEOPLE_TABLE)?;
    Ok(table.get(resource_name)?.map(|v| v.value().to_owned()))
}

/// List all people: returns (resourceName, display_name) pairs, sorted by resourceName.
pub fn list_people(
    db: &Database,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let txn = db.begin_read()?;
    let table = txn.open_table(PEOPLE_TABLE)?;
    let mut entries = Vec::new();
    for item in table.iter()? {
        let item = item?;
        let rn: String = item.0.value().to_owned();
        let name: String = item.1.value().to_owned();
        entries.push((rn, name));
    }
    Ok(entries)
}

/// Import multiple people at once (batch insert in a single transaction).
pub fn import_people(
    db: &Database,
    people: &[(String, String)],
) -> Result<usize, Box<dyn std::error::Error>> {
    let txn = db.begin_write()?;
    let count;
    {
        let mut table = txn.open_table(PEOPLE_TABLE)?;
        count = people.len();
        for (resource_name, display_name) in people {
            table.insert(resource_name.as_str(), display_name.as_str())?;
        }
    }
    txn.commit()?;
    Ok(count)
}

// ---------------------------------------------------------------------------
// Image tags operations (flat string list)
// ---------------------------------------------------------------------------

/// Get tags for an image hash. Returns empty vec if not found.
pub fn get_tags(db: &Database, hash: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let txn = db.begin_read()?;
    let table = txn.open_table(TAGS_TABLE)?;
    match table.get(hash)? {
        Some(value) => {
            let json_str: &str = value.value();
            let tags: Vec<String> = serde_json::from_str(json_str)?;
            Ok(tags)
        }
        None => Ok(vec![]),
    }
}

/// Set tags for an image hash.
pub fn set_tags(
    db: &Database,
    hash: &str,
    tags: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(tags)?;
    let txn = db.begin_write()?;
    {
        let mut table = txn.open_table(TAGS_TABLE)?;
        table.insert(hash, json.as_str())?;
    }
    txn.commit()?;
    Ok(())
}

/// Add a tag to an image. Does nothing if the tag already exists.
pub fn add_tag(
    db: &Database,
    hash: &str,
    tag: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tags = get_tags(db, hash)?;
    if !tags.iter().any(|t| t == tag) {
        tags.push(tag.to_owned());
        set_tags(db, hash, &tags)?;
    }
    Ok(())
}

/// Remove a tag from an image. Does nothing if the tag doesn't exist.
pub fn remove_tag(
    db: &Database,
    hash: &str,
    tag: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tags = get_tags(db, hash)?;
    let len_before = tags.len();
    tags.retain(|t| t != tag);
    if tags.len() != len_before {
        set_tags(db, hash, &tags)?;
    }
    Ok(())
}

/// List all image tag entries: returns (hash, tags) pairs.
pub fn list_all_tags(
    db: &Database,
) -> Result<Vec<(String, Vec<String>)>, Box<dyn std::error::Error>> {
    let txn = db.begin_read()?;
    let table = txn.open_table(TAGS_TABLE)?;
    let mut entries = Vec::new();
    for item in table.iter()? {
        let item = item?;
        let hash: String = item.0.value().to_owned();
        let json_str: &str = item.1.value();
        let tags: Vec<String> = serde_json::from_str(json_str)?;
        entries.push((hash, tags));
    }
    Ok(entries)
}

// ---------------------------------------------------------------------------
// Import from JSON file
// ---------------------------------------------------------------------------

/// A single entry in the rscontacts export-json --short format.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactEntry {
    pub resource_name: String,
    pub display_name: String,
}

/// Import people from a JSON file (rscontacts export-json --short format).
/// Returns the number of people imported.
pub fn import_people_from_file(path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let entries: Vec<ContactEntry> = serde_json::from_str(&data)?;
    let db = open_db()?;
    let people: Vec<(String, String)> = entries
        .into_iter()
        .map(|e| (e.resource_name, e.display_name))
        .collect();
    import_people(&db, &people)
}

// ---------------------------------------------------------------------------
// Dump
// ---------------------------------------------------------------------------

/// Full database dump structure for JSON export.
#[derive(Debug, Serialize)]
pub struct DbDump {
    pub people: BTreeMap<String, String>,
    pub image_tags: BTreeMap<String, Vec<String>>,
}

/// Dump the entire database as a pretty-printed JSON string.
pub fn dump_json() -> Result<String, Box<dyn std::error::Error>> {
    let db = open_db()?;
    let people: BTreeMap<String, String> = list_people(&db)?.into_iter().collect();
    let image_tags: BTreeMap<String, Vec<String>> = list_all_tags(&db)?.into_iter().collect();
    let dump = DbDump {
        people,
        image_tags,
    };
    Ok(serde_json::to_string_pretty(&dump)?)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn test_db_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("rsimagetag_test_{name}.redb"))
    }

    fn create_test_db(name: &str) -> Database {
        let path = test_db_path(name);
        let _ = fs::remove_file(&path);
        let db = Database::create(&path).unwrap();
        let txn = db.begin_write().unwrap();
        {
            let _t1 = txn.open_table(TAGS_TABLE).unwrap();
            let _t2 = txn.open_table(PEOPLE_TABLE).unwrap();
        }
        txn.commit().unwrap();
        db
    }

    // --- People lookup tests ---

    #[test]
    fn test_add_and_get_person() {
        let db = create_test_db("add_get_person");
        add_person(&db, "people/c123", "Alice Smith").unwrap();
        let name = get_person(&db, "people/c123").unwrap();
        assert_eq!(name.unwrap(), "Alice Smith");
    }

    #[test]
    fn test_get_person_not_found() {
        let db = create_test_db("get_person_notfound");
        let name = get_person(&db, "people/c999").unwrap();
        assert!(name.is_none());
    }

    #[test]
    fn test_remove_person() {
        let db = create_test_db("remove_person");
        add_person(&db, "people/c123", "Alice").unwrap();
        remove_person(&db, "people/c123").unwrap();
        assert!(get_person(&db, "people/c123").unwrap().is_none());
    }

    #[test]
    fn test_list_people() {
        let db = create_test_db("list_people");
        add_person(&db, "people/c2", "Bob").unwrap();
        add_person(&db, "people/c1", "Alice").unwrap();
        let people = list_people(&db).unwrap();
        assert_eq!(people.len(), 2);
        assert_eq!(people[0].0, "people/c1");
        assert_eq!(people[1].0, "people/c2");
    }

    #[test]
    fn test_import_people() {
        let db = create_test_db("import_people");
        let batch = vec![
            ("people/c1".to_owned(), "Alice".to_owned()),
            ("people/c2".to_owned(), "Bob".to_owned()),
            ("people/c3".to_owned(), "Charlie".to_owned()),
        ];
        let count = import_people(&db, &batch).unwrap();
        assert_eq!(count, 3);
        assert_eq!(list_people(&db).unwrap().len(), 3);
    }

    #[test]
    fn test_import_people_updates_existing() {
        let db = create_test_db("import_update");
        add_person(&db, "people/c1", "Old Name").unwrap();
        let batch = vec![("people/c1".to_owned(), "New Name".to_owned())];
        import_people(&db, &batch).unwrap();
        assert_eq!(get_person(&db, "people/c1").unwrap().unwrap(), "New Name");
    }

    // --- Tag type detection ---

    #[test]
    fn test_is_person_tag() {
        assert!(is_person_tag("people/c1234567890"));
        assert!(is_person_tag("people/c1"));
        assert!(!is_person_tag("beach"));
        assert!(!is_person_tag("wedding"));
        assert!(!is_person_tag("people")); // no /c
        assert!(!is_person_tag("people/")); // no c
    }

    // --- Image tags tests (flat list) ---

    #[test]
    fn test_get_tags_empty() {
        let db = create_test_db("get_tags_empty");
        let tags = get_tags(&db, "abc123").unwrap();
        assert!(tags.is_empty());
    }

    #[test]
    fn test_set_and_get_tags() {
        let db = create_test_db("set_get_tags");
        let tags = vec!["people/c1".to_owned(), "beach".to_owned()];
        set_tags(&db, "hash1", &tags).unwrap();
        let result = get_tags(&db, "hash1").unwrap();
        assert_eq!(result, tags);
    }

    #[test]
    fn test_add_tag() {
        let db = create_test_db("add_tag");
        add_tag(&db, "hash1", "people/c1").unwrap();
        add_tag(&db, "hash1", "beach").unwrap();
        add_tag(&db, "hash1", "people/c1").unwrap(); // duplicate
        let tags = get_tags(&db, "hash1").unwrap();
        assert_eq!(tags, vec!["people/c1", "beach"]);
    }

    #[test]
    fn test_remove_tag() {
        let db = create_test_db("remove_tag");
        set_tags(&db, "hash1", &["people/c1".into(), "beach".into(), "sunset".into()]).unwrap();
        remove_tag(&db, "hash1", "beach").unwrap();
        let tags = get_tags(&db, "hash1").unwrap();
        assert_eq!(tags, vec!["people/c1", "sunset"]);
    }

    #[test]
    fn test_remove_tag_nonexistent() {
        let db = create_test_db("remove_tag_nonexist");
        set_tags(&db, "hash1", &["beach".into()]).unwrap();
        remove_tag(&db, "hash1", "wedding").unwrap();
        let tags = get_tags(&db, "hash1").unwrap();
        assert_eq!(tags, vec!["beach"]);
    }

    #[test]
    fn test_list_all_tags() {
        let db = create_test_db("list_all_tags");
        set_tags(&db, "hash1", &["people/c1".into()]).unwrap();
        set_tags(&db, "hash2", &["beach".into()]).unwrap();
        let entries = list_all_tags(&db).unwrap();
        assert_eq!(entries.len(), 2);
    }

    // --- Hash tests ---

    #[test]
    fn test_hash_file() {
        let path = std::env::temp_dir().join("rsimagetag_test_hashfile.txt");
        fs::write(&path, b"hello world").unwrap();
        let h = hash_file(&path).unwrap();
        assert_eq!(h, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
        fs::remove_file(&path).ok();
    }
}
