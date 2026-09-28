mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos, lock::Lock,
};
use pretty_assertions::assert_eq;

fn full() -> Lock {
    Lock {
        availability: vec![Availability {
            topic: "home/frontdoor/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        code_format: Some("^\\d{4}$".into()),
        command_template: Some("{ \"action\": \"{{ value }}\", \"code\":\"{{ code }}\" }".into()),
        command_topic: "home/frontdoor/set".into(),
        default_entity_id: Some("lock.home_frontdoor".into()),
        device: Some(Device {
            identifiers: vec!["home_frontdoor_device".into()],
            name: Some("Front door lock".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/home_frontdoor.png".into()),
        group: vec!["home_frontdoor_2".into()],
        icon: Some("mdi:lock".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("home/frontdoor/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Front door".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "home_frontdoor2mqtt".into(),
            ..Default::default()
        }),
        payload_lock: Some("LOCK".into()),
        payload_open: Some("OPEN".into()),
        payload_reset: Some("None".into()),
        payload_unlock: Some("UNLOCK".into()),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(true),
        state_jammed: Some("MOTOR_JAMMED".into()),
        state_locked: Some("LOCKED".into()),
        state_locking: Some("LOCKING".into()),
        state_open: Some("OPEN".into()),
        state_opening: Some("OPENING".into()),
        state_topic: Some("home/frontdoor/state".into()),
        state_unlocked: Some("UNLOCKED".into()),
        state_unlocking: Some("UNLOCKING".into()),
        unique_id: Some("home_frontdoor".into()),
        value_template: Some("{{ value_json.state }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/lock.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Lock {
        command_topic: "home/frontdoor/set".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Full configuration" example from <https://www.home-assistant.io/integrations/lock.mqtt/>, as JSON.
#[test]
fn docs_full_example() {
    let value: Lock = support::from_fixture("lock_frontdoor.json");
    assert_eq!(
        value,
        Lock {
            code_format: Some("^\\d{4}$".into()),
            command_template: Some(
                "{ \"action\": \"{{ value }}\", \"code\":\"{{ code }}\" }".into()
            ),
            command_topic: "home-assistant/frontdoor/set".into(),
            name: Some("Frontdoor".into()),
            optimistic: Some(false),
            payload_lock: Some("LOCK".into()),
            payload_unlock: Some("UNLOCK".into()),
            qos: Some(Qos::AtLeastOnce),
            retain: Some(true),
            state_jammed: Some("MOTOR_JAMMED".into()),
            state_locked: Some("LOCK".into()),
            state_locking: Some("LOCKING".into()),
            state_topic: Some("home-assistant/frontdoor/state".into()),
            state_unlocked: Some("UNLOCK".into()),
            state_unlocking: Some("UNLOCKING".into()),
            value_template: Some("{{ value.x }}".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "lock");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "lock");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Lock::builder().build().is_err());
    assert!(Lock::builder().command_topic("t").build().is_ok());
}
