use conduit::resources::resource::{Resource, ResourceId, ResourceType};

#[test]
fn resource_id_can_be_created() {
    let id = ResourceId::new();

    assert!(!id.to_string().is_empty());
}

#[test]
fn resources_have_expected_type() {
    let resource = Resource::new(
        ResourceId::new(),
        "test",
        ResourceType::Theme,
    );

    assert_eq!(resource.resource_type(), ResourceType::Theme);
}

#[test]
fn resource_name_is_preserved() {
    let resource = Resource::new(
        ResourceId::new(),
        "cyberpunk",
        ResourceType::Theme,
    );

    assert_eq!(resource.name(), "cyberpunk");
}
