mod support;

use hassliebe::{
    Availability, AvailabilityMode, Device, EntityCategory, Origin, Qos,
    text::{Text, TextMode},
};
use pretty_assertions::assert_eq;

fn full() -> Text {
    Text {
        availability: vec![Availability {
            topic: "lcd/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some("{\"text\": \"{{ value }}\"}".into()),
        command_topic: "lcd/text/set".into(),
        default_entity_id: Some("text.remote_lcd_screen".into()),
        device: Some(Device {
            identifiers: vec!["lcd01".into()],
            name: Some("LCD".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/lcd.png".into()),
        group: vec!["lcd02_text".into()],
        icon: Some("mdi:ab-testing".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("lcd/attributes".into()),
        max: Some(20),
        message_expiry_interval: Some(60.into()),
        min: Some(2),
        mode: Some(TextMode::Text),
        name: Some("Screen".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "lcd2mqtt".into(),
            ..Default::default()
        }),
        pattern: Some("^[A-Z ]*$".into()),
        qos: Some(Qos::AtMostOnce),
        retain: Some(true),
        state_topic: Some("lcd/text".into()),
        unique_id: Some("lcd01_text".into()),
        value_template: Some("{{ value_json.text }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The first configuration example from
/// <https://www.home-assistant.io/integrations/text.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Text {
        command_topic: "command-topic".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Example with full configuration" from
/// <https://www.home-assistant.io/integrations/text.mqtt/>, as JSON.
#[test]
fn docs_full_example() {
    let text: Text = support::from_fixture("text_lcd.json");
    assert_eq!(
        text,
        Text {
            command_topic: "txt/cmd".into(),
            icon: Some("mdi:ab-testing".into()),
            max: Some(20),
            min: Some(2),
            mode: Some(TextMode::Text),
            name: Some("Remote LCD screen".into()),
            state_topic: Some("txt/state".into()),
            ..Default::default()
        }
    );
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_command_topic() {
    assert!(Text::builder().build().is_err());
    assert!(
        Text::builder()
            .command_topic("t")
            .mode(TextMode::Password)
            .build()
            .is_ok()
    );
}
