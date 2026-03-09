use std::path::Path;
use rsimagetag::{scan_images, MyApp};

#[test]
fn test_scan_images_empty_dir() {
    let dir = std::env::temp_dir().join("rsimagetag_test_empty");
    std::fs::create_dir_all(&dir).unwrap();
    let images = scan_images(&dir);
    assert!(images.is_empty());
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_scan_images_finds_images() {
    let dir = std::env::temp_dir().join("rsimagetag_test_images");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("photo.jpg"), b"fake").unwrap();
    std::fs::write(dir.join("photo.png"), b"fake").unwrap();
    std::fs::write(dir.join("notes.txt"), b"not an image").unwrap();
    let images = scan_images(&dir);
    assert_eq!(images.len(), 2);
    assert!(images.iter().all(|p| p.extension().is_some_and(|e| e == "jpg" || e == "png")));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_scan_images_recursive() {
    let dir = std::env::temp_dir().join("rsimagetag_test_recursive");
    let sub = dir.join("subdir");
    std::fs::create_dir_all(&sub).unwrap();
    std::fs::write(dir.join("a.jpg"), b"fake").unwrap();
    std::fs::write(sub.join("b.png"), b"fake").unwrap();
    let images = scan_images(&dir);
    assert_eq!(images.len(), 2);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_scan_images_case_insensitive_extension() {
    let dir = std::env::temp_dir().join("rsimagetag_test_case");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("photo.JPG"), b"fake").unwrap();
    std::fs::write(dir.join("photo.Png"), b"fake").unwrap();
    let images = scan_images(&dir);
    assert_eq!(images.len(), 2);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_scan_images_sorted() {
    let dir = std::env::temp_dir().join("rsimagetag_test_sorted");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("c.jpg"), b"fake").unwrap();
    std::fs::write(dir.join("a.jpg"), b"fake").unwrap();
    std::fs::write(dir.join("b.jpg"), b"fake").unwrap();
    let images = scan_images(&dir);
    let names: Vec<&str> = images.iter().map(|p| p.file_name().unwrap().to_str().unwrap()).collect();
    assert_eq!(names, vec!["a.jpg", "b.jpg", "c.jpg"]);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_nonexistent_dir() {
    let images = scan_images(Path::new("/tmp/rsimagetag_nonexistent_dir_12345"));
    assert!(images.is_empty());
}

#[test]
fn test_default_app_current_index() {
    let app = MyApp::default();
    assert_eq!(app.current_index, 0);
}

#[test]
fn test_image_count() {
    let app = MyApp::default();
    assert_eq!(app.image_count(), app.images.len());
}

#[test]
fn test_navigation_empty() {
    let mut app = MyApp {
        images: vec![],
        current_index: 0,
        texture: None,
    };
    app.go_next();
    assert_eq!(app.current_index, 0);
    app.go_prev();
    assert_eq!(app.current_index, 0);
}

#[test]
fn test_navigation_wraps() {
    let mut app = MyApp {
        images: vec!["a.jpg".into(), "b.jpg".into(), "c.jpg".into()],
        current_index: 0,
        texture: None,
    };
    // next wraps around
    app.go_next();
    assert_eq!(app.current_index, 1);
    app.go_next();
    assert_eq!(app.current_index, 2);
    app.go_next();
    assert_eq!(app.current_index, 0);
    // prev wraps around
    app.go_prev();
    assert_eq!(app.current_index, 2);
}

#[test]
fn test_current_path() {
    let app = MyApp {
        images: vec!["a.jpg".into(), "b.jpg".into()],
        current_index: 1,
        texture: None,
    };
    assert_eq!(app.current_path().unwrap(), Path::new("b.jpg"));
}

#[test]
fn test_current_path_empty() {
    let app = MyApp {
        images: vec![],
        current_index: 0,
        texture: None,
    };
    assert!(app.current_path().is_none());
}
