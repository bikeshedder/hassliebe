mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    light::{ColorMode, JsonLight, JsonLightSchema},
};
use pretty_assertions::assert_eq;

fn full() -> JsonLight {
    JsonLight {
        availability: vec![Availability {
            topic: "home/rgb1/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        brightness: Some(true),
        brightness_scale: Some(255),
        color_temp_kelvin: Some(true),
        command_topic: "home/rgb1/set".into(),
        default_entity_id: Some("light.home_rgb1".into()),
        device: Some(Device {
            identifiers: vec!["home_rgb1_device".into()],
            name: Some("LED strip".into()),
            ..Default::default()
        }),
        effect: Some(true),
        effect_list: vec!["rainbow".into(), "colorloop".into()],
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/home_rgb1.png".into()),
        flash: Some(true),
        flash_time_long: Some(10),
        flash_time_short: Some(2),
        group: vec!["home_rgb1_2".into()],
        icon: Some("mdi:led-strip-variant".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("home/rgb1/attributes".into()),
        max_kelvin: Some(6500),
        max_mireds: None,
        message_expiry_interval: Some(60.into()),
        min_kelvin: Some(2700),
        min_mireds: None,
        name: Some("Strip".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "home_rgb12mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(false),
        schema: JsonLightSchema::Json,
        state_topic: Some("home/rgb1".into()),
        supported_color_modes: Some(vec![ColorMode::ColorTemp, ColorMode::Rgb]),
        transition: Some(true),
        unique_id: Some("home_rgb1".into()),
        visible_by_default: Some(true),
        white_scale: Some(255),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example of the JSON schema from
/// <https://www.home-assistant.io/integrations/light.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(JsonLight {
        command_topic: "home/rgb1/set".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Brightness and RGB support" example of the JSON schema from
/// <https://www.home-assistant.io/integrations/light.mqtt/>, as JSON.
#[test]
fn docs_brightness_rgb_example() {
    let value: JsonLight = support::from_fixture("light_json_rgb.json");
    assert_eq!(
        value,
        JsonLight {
            brightness: Some(true),
            command_topic: "home/rgb1/set".into(),
            name: Some("mqtt_json_light_1".into()),
            state_topic: Some("home/rgb1".into()),
            supported_color_modes: Some(vec![ColorMode::Rgb]),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "light");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "light");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(JsonLight::builder().build().is_err());
    assert!(JsonLight::builder().command_topic("t").build().is_ok());
}

#[test]
fn schema_is_always_serialized_and_checked() {
    assert_eq!(
        serde_json::to_value(JsonLight::default()).unwrap()["schema"],
        "json"
    );
    assert!(serde_json::from_str::<JsonLight>(r#"{"command_topic": "t"}"#).is_err());
    assert!(
        serde_json::from_str::<JsonLight>(r#"{"schema": "template", "command_topic": "t"}"#)
            .is_err()
    );
}

#[cfg(feature = "json")]
#[test]
fn component_dispatches_on_schema() {
    let component: Component =
        serde_json::from_str(r#"{"platform": "light", "schema": "json", "command_topic": "t"}"#)
            .unwrap();
    assert!(matches!(component, Component::JsonLight(_)));
}
