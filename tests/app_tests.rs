use rsimagetag::MyApp;

#[test]
fn test_default_name() {
    let app = MyApp::default();
    assert_eq!(app.name, "World");
}

#[test]
fn test_default_greeting() {
    let app = MyApp::default();
    assert_eq!(app.greeting(), "Hello, World!");
}

#[test]
fn test_custom_greeting() {
    let app = MyApp {
        name: "Rust".to_owned(),
    };
    assert_eq!(app.greeting(), "Hello, Rust!");
}

#[test]
fn test_empty_name_greeting() {
    let app = MyApp {
        name: String::new(),
    };
    assert_eq!(app.greeting(), "Hello, !");
}

#[test]
fn test_name_is_mutable() {
    let mut app = MyApp::default();
    app.name = "egui".to_owned();
    assert_eq!(app.greeting(), "Hello, egui!");
}
