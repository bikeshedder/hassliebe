mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    alarm_control_panel::{AlarmControlPanel, AlarmControlPanelFeature},
};
use pretty_assertions::assert_eq;

fn full() -> AlarmControlPanel {
    AlarmControlPanel {
        availability: vec![Availability {
            topic: "alarmdecoder/panel/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        code: Some("REMOTE_CODE_TEXT".into()),
        code_arm_required: Some(false),
        code_disarm_required: Some(true),
        code_trigger_required: Some(true),
        command_template: Some("{ \"action\": \"{{ action }}\", \"code\": \"{{ code }}\" }".into()),
        command_topic: "alarmdecoder/panel/set".into(),
        default_entity_id: Some("alarm_control_panel.alarmdecoder_panel".into()),
        device: Some(Device {
            identifiers: vec!["alarmdecoder_panel_device".into()],
            name: Some("AlarmDecoder".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/alarmdecoder_panel.png".into()),
        group: vec!["alarmdecoder_panel_2".into()],
        icon: Some("mdi:shield-home".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("alarmdecoder/panel/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Alarm Panel".into()),
        origin: Some(Origin {
            name: "alarmdecoder_panel2mqtt".into(),
            ..Default::default()
        }),
        payload_arm_away: Some("ARM_AWAY".into()),
        payload_arm_custom_bypass: Some("ARM_CUSTOM_BYPASS".into()),
        payload_arm_home: Some("ARM_HOME".into()),
        payload_arm_night: Some("ARM_NIGHT".into()),
        payload_arm_vacation: Some("ARM_VACATION".into()),
        payload_disarm: Some("DISARM".into()),
        payload_trigger: Some("TRIGGER".into()),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(false),
        state_topic: "alarmdecoder/panel".into(),
        supported_features: Some(vec![
            AlarmControlPanelFeature::ArmHome,
            AlarmControlPanelFeature::ArmAway,
            AlarmControlPanelFeature::Trigger,
        ]),
        unique_id: Some("alarmdecoder_panel".into()),
        value_template: Some("{{ value_json.state }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/alarm_control_panel.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(AlarmControlPanel {
        command_topic: "home/alarm/set".into(),
        state_topic: "home/alarm".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Configuration with partial feature support" from
/// <https://www.home-assistant.io/integrations/alarm_control_panel.mqtt/>, as JSON.
#[test]
fn docs_partial_features_example() {
    let value: AlarmControlPanel = support::from_fixture("alarm_control_panel_partial.json");
    assert_eq!(
        value,
        AlarmControlPanel {
            command_topic: "alarmdecoder/panel/set".into(),
            name: Some("Alarm Panel".into()),
            state_topic: "alarmdecoder/panel".into(),
            supported_features: Some(vec![
                AlarmControlPanelFeature::ArmHome,
                AlarmControlPanelFeature::ArmAway
            ]),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "alarm_control_panel");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "alarm_control_panel");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(AlarmControlPanel::builder().build().is_err());
    assert!(
        AlarmControlPanel::builder()
            .command_topic("t")
            .state_topic("s")
            .build()
            .is_ok()
    );
}

/// An empty list means no features, unlike an absent key (all features).
#[test]
fn empty_supported_features_is_kept() {
    let panel: AlarmControlPanel = serde_json::from_str(
        r#"{"command_topic": "t", "state_topic": "s", "supported_features": []}"#,
    )
    .unwrap();
    assert_eq!(panel.supported_features, Some(vec![]));
}
