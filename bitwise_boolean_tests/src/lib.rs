use bitwise_boolean::BitwiseBoolean;

#[derive(Default, BitwiseBoolean)]
struct ExampleSettingsData {
    // up to 8 flags total
    #[bitwise_bool(
        admin,
        agreed_to_terms,
        banned,
        random_other_flag
    )]
    pub booleans: u8,
}

#[test]
fn test_bitwise_boolean() {
    let mut data = ExampleSettingsData::default();
    data.set_admin(true);
    assert!(data.admin());
    assert!(!data.agreed_to_terms());
    assert!(!data.banned());
    assert!(!data.random_other_flag());

    data.set_agreed_to_terms(true);
    assert!(data.admin());
    assert!(data.agreed_to_terms());
    assert!(!data.banned());
    assert!(!data.random_other_flag());

    data.set_banned(true);
    assert!(data.admin());
    assert!(data.agreed_to_terms());
    assert!(data.banned());
    assert!(!data.random_other_flag());

    data.set_random_other_flag(true);
    assert!(data.admin());
    assert!(data.agreed_to_terms());
    assert!(data.banned());
    assert!(data.random_other_flag());
    
}