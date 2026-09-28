mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    light::{TemplateLight, TemplateLightSchema},
};
use pretty_assertions::assert_eq;

fn full() -> TemplateLight {
    TemplateLight {
        availability: vec![Availability {
            topic: "home/rgb1/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        blue_template: Some("{{ value.split(',')[2].split('-')[2] }}".into()),
        brightness_template: Some("{{ value.split(',')[1] }}".into()),
        color_temp_kelvin: Some(false),
        color_temp_template: Some("{{ value.split(',')[4] }}".into()),
        command_off_template: "off".into(),
        command_on_template:
            "on,{{ brightness|d }},{{ red|d }}-{{ green|d }}-{{ blue|d }},{{ hue|d }}-{{ sat|d }}"
                .into(),
        command_topic: "home/rgb1/set".into(),
        default_entity_id: Some("light.home_rgb1".into()),
        device: Some(Device {
            identifiers: vec!["home_rgb1_device".into()],
            name: Some("RGB bulb".into()),
            ..Default::default()
        }),
        effect_list: vec!["rainbow".into(), "colorloop".into()],
        effect_template: Some("{{ value.split(',')[5] }}".into()),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/home_rgb1.png".into()),
        green_template: Some("{{ value.split(',')[2].split('-')[1] }}".into()),
        group: vec!["home_rgb1_2".into()],
        icon: Some("mdi:lightbulb".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("home/rgb1/attributes".into()),
        max_kelvin: None,
        max_mireds: Some(370),
        message_expiry_interval: Some(60.into()),
        min_kelvin: None,
        min_mireds: Some(153),
        name: Some("Bulb".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "home_rgb12mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        red_template: Some("{{ value.split(',')[2].split('-')[0] }}".into()),
        retain: Some(false),
        schema: TemplateLightSchema::Template,
        state_template: Some("{{ value.split(',')[0] }}".into()),
        state_topic: Some("home/rgb1/status".into()),
        unique_id: Some("home_rgb1".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example of the template schema from
/// <https://www.home-assistant.io/integrations/light.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(TemplateLight {
        command_off_template: "off".into(),
        command_on_template: "on".into(),
        command_topic: "home/rgb1/set".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Simple string payload" example of the template schema from
/// <https://www.home-assistant.io/integrations/light.mqtt/>, as JSON.
#[test]
fn docs_simple_string_example() {
    let value: TemplateLight = support::from_fixture("light_template_string.json");
    assert_eq!(
        value,
        TemplateLight {
            blue_template: Some("{{ value.split(',')[2].split('-')[2] }}".into()),
            brightness_template: Some("{{ value.split(',')[1] }}".into()),
            command_off_template: "off".into(),
            command_on_template: "on,{{ brightness|d }},{{ red|d }}-{{ green|d }}-{{ blue|d }},{{ hue|d }}-{{ sat|d }}".into(),
            command_topic: "home/rgb1/set".into(),
            green_template: Some("{{ value.split(',')[2].split('-')[1] }}".into()),
            red_template: Some("{{ value.split(',')[2].split('-')[0] }}".into()),
            state_template: Some("{{ value.split(',')[0] }}".into()),
            state_topic: Some("home/rgb1/status".into()),
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
    assert!(TemplateLight::builder().build().is_err());
    assert!(
        TemplateLight::builder()
            .command_topic("t")
            .command_on_template("on")
            .command_off_template("off")
            .build()
            .is_ok()
    );
}

#[cfg(feature = "json")]
#[test]
fn component_dispatches_on_schema() {
    let component: Component = serde_json::from_str(
        r#"{"platform": "light", "schema": "template", "command_topic": "t",
            "command_on_template": "on", "command_off_template": "off"}"#,
    )
    .unwrap();
    assert!(matches!(component, Component::TemplateLight(_)));
}
