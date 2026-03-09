use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const TAGS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("tags");

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

/// Initialize the database: create config dir and database file with the tags table.
pub fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    let dir = config_dir();
    fs::create_dir_all(&dir)?;
    let path = db_path();
    let db = Database::create(&path)?;
    // Create the table by opening a write transaction
    let txn = db.begin_write()?;
    {
        let _table = txn.open_table(TAGS_TABLE)?;
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

/// Add a tag to an image hash. Does nothing if the tag already exists.
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

/// Remove a tag from an image hash. Does nothing if the tag doesn't exist.
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

/// List all entries in the database: returns (hash, tags) pairs.
pub fn list_all(db: &Database) -> Result<Vec<(String, Vec<String>)>, Box<dyn std::error::Error>> {
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

/// Dump the entire database as a pretty-printed JSON string.
pub fn dump_json() -> Result<String, Box<dyn std::error::Error>> {
    let db = open_db()?;
    let entries = list_all(&db)?;
    let map: std::collections::BTreeMap<String, Vec<String>> = entries.into_iter().collect();
    Ok(serde_json::to_string_pretty(&map)?)
}

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
            let _table = txn.open_table(TAGS_TABLE).unwrap();
        }
        txn.commit().unwrap();
        db
    }

    #[test]
    fn test_get_tags_empty() {
        let db = create_test_db("get_empty");
        let tags = get_tags(&db, "abc123").unwrap();
        assert!(tags.is_empty());
    }

    #[test]
    fn test_set_and_get_tags() {
        let db = create_test_db("set_get");
        let tags = vec!["Alice".to_owned(), "beach".to_owned()];
        set_tags(&db, "hash1", &tags).unwrap();
        let result = get_tags(&db, "hash1").unwrap();
        assert_eq!(result, tags);
    }

    #[test]
    fn test_add_tag() {
        let db = create_test_db("add_tag");
        add_tag(&db, "hash2", "Alice").unwrap();
        add_tag(&db, "hash2", "Bob").unwrap();
        add_tag(&db, "hash2", "Alice").unwrap(); // duplicate, should be ignored
        let tags = get_tags(&db, "hash2").unwrap();
        assert_eq!(tags, vec!["Alice", "Bob"]);
    }

    #[test]
    fn test_remove_tag() {
        let db = create_test_db("remove_tag");
        set_tags(&db, "hash3", &["Alice".into(), "Bob".into(), "beach".into()]).unwrap();
        remove_tag(&db, "hash3", "Bob").unwrap();
        let tags = get_tags(&db, "hash3").unwrap();
        assert_eq!(tags, vec!["Alice", "beach"]);
    }

    #[test]
    fn test_remove_tag_nonexistent() {
        let db = create_test_db("remove_nonexist");
        set_tags(&db, "hash4", &["Alice".into()]).unwrap();
        remove_tag(&db, "hash4", "Bob").unwrap(); // should not error
        let tags = get_tags(&db, "hash4").unwrap();
        assert_eq!(tags, vec!["Alice"]);
    }

    #[test]
    fn test_list_all() {
        let db = create_test_db("list_all");
        set_tags(&db, "aaa", &["tag1".into()]).unwrap();
        set_tags(&db, "bbb", &["tag2".into(), "tag3".into()]).unwrap();
        let entries = list_all(&db).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_hash_file() {
        let path = std::env::temp_dir().join("rsimagetag_test_hashfile.txt");
        fs::write(&path, b"hello world").unwrap();
        let h = hash_file(&path).unwrap();
        // SHA-256 of "hello world"
        assert_eq!(h, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
        fs::remove_file(&path).ok();
    }
}
