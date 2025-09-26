use proptest::prelude::*;

proptest! {
    #[test]
    fn normalize_id_is_total(id in "\\PC{1,64}") {
        let result = my_rust_pi_app::normalize_id(&id);
        // Should never panic
        assert!(result.len() <= id.len());
    }

    #[test]
    fn normalize_id_preserves_alphanumeric(id in "[a-zA-Z0-9_-]{1,32}") {
        let result = my_rust_pi_app::normalize_id(&id);
        // Should preserve valid characters and convert to lowercase
        assert_eq!(result, id.to_lowercase());
    }

    #[test]
    fn normalize_id_filters_invalid_chars(
        prefix in "[a-zA-Z0-9_-]{0,10}",
        invalid_chars in "[^a-zA-Z0-9_-]{1,10}",
        suffix in "[a-zA-Z0-9_-]{0,10}"
    ) {
        let input = format!("{}{}{}", prefix, invalid_chars, suffix);
        let result = my_rust_pi_app::normalize_id(&input);
        let expected = format!("{}{}", prefix, suffix).to_lowercase();
        assert_eq!(result, expected);
    }

    #[test]
    fn normalize_id_handles_unicode(input in "\\PC{0,20}") {
        let result = my_rust_pi_app::normalize_id(&input);
        // Should only contain valid ASCII characters
        assert!(result.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
    }
}

#[test]
fn normalize_id_empty_input() {
    let result = my_rust_pi_app::normalize_id("");
    assert_eq!(result, "");
}