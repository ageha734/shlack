use shlack::{HasId, HasName, Msg};

#[test]
fn test_msg_new() {
    let msg = Msg::new(
        "Hello, World!".to_string(),
        "general".to_string(),
        "test-token".to_string(),
    );

    assert_eq!(msg.text(), "Hello, World!");
    assert_eq!(msg.channel(), "general");
    assert_eq!(msg.token(), "test-token");
}

#[test]
fn test_find_addr_with_matching_channel() {
    let msg = Msg::new(
        "test".to_string(),
        "general".to_string(),
        "token".to_string(),
    );

    // Mock channel data
    struct MockChannel {
        id: Option<String>,
        name: Option<String>,
    }

    impl HasId for MockChannel {
        fn id(&self) -> String {
            self.id.clone().unwrap_or_default()
        }
    }

    impl HasName for MockChannel {
        fn name(&self) -> String {
            self.name.clone().unwrap_or_default()
        }
    }

    let channels = vec![
        MockChannel {
            id: Some("C123456".to_string()),
            name: Some("general".to_string()),
        },
        MockChannel {
            id: Some("C789012".to_string()),
            name: Some("random".to_string()),
        },
    ];

    let result = msg.find_addr(&channels);
    assert_eq!(result, Some("C123456".to_string()));
}

#[test]
fn test_find_addr_with_no_matching_channel() {
    let msg = Msg::new(
        "test".to_string(),
        "nonexistent".to_string(),
        "token".to_string(),
    );

    // Mock channel data
    struct MockChannel {
        id: Option<String>,
        name: Option<String>,
    }

    impl HasId for MockChannel {
        fn id(&self) -> String {
            self.id.clone().unwrap_or_default()
        }
    }

    impl HasName for MockChannel {
        fn name(&self) -> String {
            self.name.clone().unwrap_or_default()
        }
    }

    let channels = vec![MockChannel {
        id: Some("C123456".to_string()),
        name: Some("general".to_string()),
    }];

    let result = msg.find_addr(&channels);
    assert_eq!(result, None);
}

#[test]
fn test_find_addr_with_empty_channels() {
    let msg = Msg::new(
        "test".to_string(),
        "general".to_string(),
        "token".to_string(),
    );

    struct MockChannel {
        id: Option<String>,
        name: Option<String>,
    }

    impl HasId for MockChannel {
        fn id(&self) -> String {
            self.id.clone().unwrap_or_default()
        }
    }

    impl HasName for MockChannel {
        fn name(&self) -> String {
            self.name.clone().unwrap_or_default()
        }
    }

    let channels: Vec<MockChannel> = vec![];
    let result = msg.find_addr(&channels);
    assert_eq!(result, None);
}
