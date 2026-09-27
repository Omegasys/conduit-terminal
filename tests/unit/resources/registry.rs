use conduit::resources::registry::ResourceRegistry;
use conduit::resources::resource::{Resource, ResourceId, ResourceType};

#[test]
fn registry_starts_empty() {
    let registry = ResourceRegistry::new();

    assert_eq!(registry.len(), 0);
}

#[test]
fn registry_can_register_resource() {
    let mut registry = ResourceRegistry::new();

    let resource = Resource::new(
        ResourceId::new(),
        "default",
        ResourceType::Theme,
    );

    let id = resource.id();

    registry
        .register(resource)
        .expect("resource should register");

    assert!(registry.get(id).is_some());
}

#[test]
fn registry_can_remove_resource() {
    let mut registry = ResourceRegistry::new();

    let resource = Resource::new(
        ResourceId::new(),
        "temporary",
        ResourceType::Theme,
    );

    let id = resource.id();

    registry
        .register(resource)
        .expect("resource should register");

    registry
        .remove(id)
        .expect("resource should be removable");

    assert!(registry.get(id).is_none());
}

#[test]
fn registry_can_find_resource_by_name() {
    let mut registry = ResourceRegistry::new();

    registry
        .register(Resource::new(
            ResourceId::new(),
            "nord",
            ResourceType::Theme,
        ))
        .expect("resource should register");

    assert!(registry.find_by_name("nord").is_some());
}
