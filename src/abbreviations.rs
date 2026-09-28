//! Expansion of abbreviated keys and the `~` base topic in discovery payloads
//! (feature `json`).
//!
//! Home Assistant accepts abbreviated keys (`stat_t`, `dev`, `cmps`, …) and a
//! base topic `~` that other `*topic` values can start or end with. The types
//! in this crate only know the full keys, so payloads must be expanded before
//! deserializing:
//!
//! ```
//! # use hassliebe::{abbreviations::expand_component_payload, switch::Switch};
//! let mut payload = serde_json::json!({
//!     "~": "homeassistant/switch/irrigation",
//!     "name": "garden",
//!     "cmd_t": "~/set",
//!     "stat_t": "~/state"
//! });
//! expand_component_payload(&mut payload);
//! let switch: Switch = serde_json::from_value(payload)?;
//! assert_eq!(switch.command_topic, "homeassistant/switch/irrigation/set");
//! # Ok::<(), serde_json::Error>(())
//! ```
//!
//! This mirrors `homeassistant/components/mqtt/discovery.py`; the tables are
//! ported from `homeassistant/components/mqtt/abbreviations.py`.
//!
//! <https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic>

use serde_json::{Map, Value};

/// The key of the base topic.
const TOPIC_BASE: &str = "~";

/// Options at the root of a device discovery payload that components inherit
/// (`SHARED_OPTIONS` in `mqtt/schemas.py`).
const SHARED_OPTIONS: &[&str] = &[
    "availability",
    "availability_mode",
    "availability_template",
    "availability_topic",
    "command_topic",
    "encoding",
    "message_expiry_interval",
    "payload_available",
    "payload_not_available",
    "state_topic",
    "qos",
];

/// Expands a single component discovery payload in place: abbreviated keys
/// (also inside `device`, `origin` and `availability`) and the `~` base topic.
pub fn expand_component_payload(payload: &mut Value) {
    replace_all_abbreviations(payload, false);
    if let Value::Object(payload) = payload {
        replace_topic_base(payload);
    }
}

/// Expands a device discovery payload in place, like Home Assistant does
/// before setting up its components: abbreviated keys (also inside `device`,
/// `origin`, `availability` and every component), then the shared options are
/// copied into every component that does not set them, then each component's
/// `~` base topic is applied. Removal entries (only `platform`) are left
/// alone.
pub fn expand_device_payload(payload: &mut Value) {
    replace_all_abbreviations(payload, false);
    let Value::Object(root) = payload else {
        return;
    };
    let shared: Vec<(String, Value)> = SHARED_OPTIONS
        .iter()
        .filter_map(|&key| Some((key.to_owned(), root.get(key)?.clone())))
        .collect();
    let Some(Value::Object(components)) = root.get_mut("components") else {
        return;
    };
    for component in components.values_mut() {
        let Value::Object(component) = component else {
            continue;
        };
        if component.keys().all(|key| key == "platform") {
            continue;
        }
        for (key, value) in &shared {
            if !component.contains_key(key) {
                component.insert(key.clone(), value.clone());
            }
        }
        replace_topic_base(component);
    }
}

/// `_replace_abbreviations` in `mqtt/discovery.py`.
fn replace_abbreviations(payload: &mut Value, table: &[(&str, &'static str)]) {
    let Value::Object(payload) = payload else {
        return;
    };
    let abbreviated: Vec<(String, &str)> = payload
        .keys()
        .filter_map(|key| Some((key.clone(), lookup(table, key)?)))
        .collect();
    for (key, full) in abbreviated {
        if let Some(value) = payload.remove(&key) {
            payload.insert(full.to_owned(), value);
        }
    }
}

/// `_replace_all_abbreviations` in `mqtt/discovery.py`.
fn replace_all_abbreviations(payload: &mut Value, component_only: bool) {
    replace_abbreviations(payload, ABBREVIATIONS);
    let Value::Object(payload) = payload else {
        return;
    };
    for_each_availability(payload, |availability| {
        replace_abbreviations(availability, ABBREVIATIONS);
    });
    if component_only {
        return;
    }
    if let Some(origin) = payload.get_mut("origin") {
        replace_abbreviations(origin, ORIGIN_ABBREVIATIONS);
    }
    if let Some(device) = payload.get_mut("device") {
        replace_abbreviations(device, DEVICE_ABBREVIATIONS);
    }
    if let Some(Value::Object(components)) = payload.get_mut("components") {
        for component in components.values_mut() {
            replace_all_abbreviations(component, true);
        }
    }
}

/// `_replace_topic_base` in `mqtt/discovery.py`.
fn replace_topic_base(payload: &mut Map<String, Value>) {
    let Some(base) = payload.remove(TOPIC_BASE) else {
        return;
    };
    let base = match base {
        Value::String(base) => base,
        other => other.to_string(),
    };
    for (key, value) in payload.iter_mut() {
        if key.ends_with("topic")
            && let Value::String(topic) = value
            && let Some(expanded) = expand_topic(topic, &base)
        {
            *topic = expanded;
        }
    }
    for_each_availability(payload, |availability| {
        if let Some(Value::String(topic)) = availability.get_mut("topic")
            && let Some(expanded) = expand_topic(topic, &base)
        {
            *topic = expanded;
        }
    });
}

/// Replaces a leading or trailing `~` with `base`. Like Home Assistant, a
/// trailing `~` wins if both are present.
fn expand_topic(topic: &str, base: &str) -> Option<String> {
    if let Some(rest) = topic.strip_suffix(TOPIC_BASE) {
        Some(format!("{rest}{base}"))
    } else {
        topic
            .strip_prefix(TOPIC_BASE)
            .map(|rest| format!("{base}{rest}"))
    }
}

/// Calls `f` for the `availability` entry or entries (HA also accepts a single
/// object).
fn for_each_availability(payload: &mut Map<String, Value>, mut f: impl FnMut(&mut Value)) {
    match payload.get_mut("availability") {
        Some(Value::Array(entries)) => entries.iter_mut().for_each(f),
        Some(entry @ Value::Object(_)) => f(entry),
        _ => {}
    }
}

fn lookup(table: &[(&str, &'static str)], key: &str) -> Option<&'static str> {
    table
        .binary_search_by(|(abbreviation, _)| (*abbreviation).cmp(key))
        .ok()
        .map(|index| table[index].1)
}

/// Abbreviations of entity and device discovery keys (`ABBREVIATIONS`). Sorted by abbreviation.
const ABBREVIATIONS: &[(&str, &str)] = &[
    ("act_stat_t", "activity_state_topic"),
    ("act_t", "action_topic"),
    ("act_tpl", "action_template"),
    ("act_val_tpl", "activity_value_template"),
    ("atype", "automation_type"),
    ("av_tones", "available_tones"),
    ("avty", "availability"),
    ("avty_mode", "availability_mode"),
    ("avty_t", "availability_topic"),
    ("avty_tpl", "availability_template"),
    ("b_tpl", "blue_template"),
    ("bri_cmd_t", "brightness_command_topic"),
    ("bri_cmd_tpl", "brightness_command_template"),
    ("bri_scl", "brightness_scale"),
    ("bri_stat_t", "brightness_state_topic"),
    ("bri_tpl", "brightness_template"),
    ("bri_val_tpl", "brightness_value_template"),
    ("cln_segmnts_cmd_t", "clean_segments_command_topic"),
    ("cln_segmnts_cmd_tpl", "clean_segments_command_template"),
    ("clr_temp_cmd_t", "color_temp_command_topic"),
    ("clr_temp_cmd_tpl", "color_temp_command_template"),
    ("clr_temp_k", "color_temp_kelvin"),
    ("clr_temp_stat_t", "color_temp_state_topic"),
    ("clr_temp_tpl", "color_temp_template"),
    ("clr_temp_val_tpl", "color_temp_value_template"),
    ("clrm_stat_t", "color_mode_state_topic"),
    ("clrm_val_tpl", "color_mode_value_template"),
    ("cmd_off_tpl", "command_off_template"),
    ("cmd_on_tpl", "command_on_template"),
    ("cmd_t", "command_topic"),
    ("cmd_tpl", "command_template"),
    ("cmps", "components"),
    ("cod_arm_req", "code_arm_required"),
    ("cod_dis_req", "code_disarm_required"),
    ("cod_form", "code_format"),
    ("cod_trig_req", "code_trigger_required"),
    ("cont_type", "content_type"),
    ("curr_hum_t", "current_humidity_topic"),
    ("curr_hum_tpl", "current_humidity_template"),
    ("curr_temp_t", "current_temperature_topic"),
    ("curr_temp_tpl", "current_temperature_template"),
    ("def_ent_id", "default_entity_id"),
    ("dev", "device"),
    ("dev_cla", "device_class"),
    ("dir_cmd_t", "direction_command_topic"),
    ("dir_cmd_tpl", "direction_command_template"),
    ("dir_stat_t", "direction_state_topic"),
    ("dir_val_tpl", "direction_value_template"),
    ("dock_cmd_t", "dock_command_topic"),
    ("dock_cmd_tpl", "dock_command_template"),
    ("dsp_prc", "display_precision"),
    ("e", "encoding"),
    ("en", "enabled_by_default"),
    ("ent_cat", "entity_category"),
    ("ent_pic", "entity_picture"),
    ("evt_typ", "event_types"),
    ("exp_aft", "expire_after"),
    ("fan_mode_cmd_t", "fan_mode_command_topic"),
    ("fan_mode_cmd_tpl", "fan_mode_command_template"),
    ("fan_mode_stat_t", "fan_mode_state_topic"),
    ("fan_mode_stat_tpl", "fan_mode_state_template"),
    ("fanspd_lst", "fan_speed_list"),
    ("flsh", "flash"),
    ("flsh_tlng", "flash_time_long"),
    ("flsh_tsht", "flash_time_short"),
    ("frc_upd", "force_update"),
    ("fx_cmd_t", "effect_command_topic"),
    ("fx_cmd_tpl", "effect_command_template"),
    ("fx_list", "effect_list"),
    ("fx_stat_t", "effect_state_topic"),
    ("fx_tpl", "effect_template"),
    ("fx_val_tpl", "effect_value_template"),
    ("g_tpl", "green_template"),
    ("grp", "group"),
    ("hs_cmd_t", "hs_command_topic"),
    ("hs_cmd_tpl", "hs_command_template"),
    ("hs_stat_t", "hs_state_topic"),
    ("hs_val_tpl", "hs_value_template"),
    ("hum_cmd_t", "target_humidity_command_topic"),
    ("hum_cmd_tpl", "target_humidity_command_template"),
    ("hum_stat_t", "target_humidity_state_topic"),
    ("hum_state_tpl", "target_humidity_state_template"),
    ("ic", "icon"),
    ("img_e", "image_encoding"),
    ("img_t", "image_topic"),
    ("init", "initial"),
    ("json_attr", "json_attributes"),
    ("json_attr_t", "json_attributes_topic"),
    ("json_attr_tpl", "json_attributes_template"),
    ("l_ver_t", "latest_version_topic"),
    ("l_ver_tpl", "latest_version_template"),
    ("lrst_val_tpl", "last_reset_value_template"),
    ("max", "max"),
    ("max_hum", "max_humidity"),
    ("max_k", "max_kelvin"),
    ("max_mirs", "max_mireds"),
    ("max_temp", "max_temp"),
    ("migr_discvry", "migrate_discovery"),
    ("min", "min"),
    ("min_hum", "min_humidity"),
    ("min_k", "min_kelvin"),
    ("min_mirs", "min_mireds"),
    ("min_temp", "min_temp"),
    ("mode", "mode"),
    ("mode_cmd_t", "mode_command_topic"),
    ("mode_cmd_tpl", "mode_command_template"),
    ("mode_stat_t", "mode_state_topic"),
    ("mode_stat_tpl", "mode_state_template"),
    ("modes", "modes"),
    ("msg_exp_int", "message_expiry_interval"),
    ("name", "name"),
    ("o", "origin"),
    ("off_dly", "off_delay"),
    ("on_cmd_type", "on_command_type"),
    ("ops", "options"),
    ("opt", "optimistic"),
    ("osc_cmd_t", "oscillation_command_topic"),
    ("osc_cmd_tpl", "oscillation_command_template"),
    ("osc_stat_t", "oscillation_state_topic"),
    ("osc_val_tpl", "oscillation_value_template"),
    ("p", "platform"),
    ("pause_cmd_t", "pause_command_topic"),
    ("pause_mw_cmd_tpl", "pause_command_template"),
    ("pct_cmd_t", "percentage_command_topic"),
    ("pct_cmd_tpl", "percentage_command_template"),
    ("pct_stat_t", "percentage_state_topic"),
    ("pct_val_tpl", "percentage_value_template"),
    ("pl", "payload"),
    ("pl_arm_away", "payload_arm_away"),
    ("pl_arm_custom_b", "payload_arm_custom_bypass"),
    ("pl_arm_home", "payload_arm_home"),
    ("pl_arm_nite", "payload_arm_night"),
    ("pl_arm_vacation", "payload_arm_vacation"),
    ("pl_avail", "payload_available"),
    ("pl_cln_sp", "payload_clean_spot"),
    ("pl_cls", "payload_close"),
    ("pl_disarm", "payload_disarm"),
    ("pl_home", "payload_home"),
    ("pl_inst", "payload_install"),
    ("pl_loc", "payload_locate"),
    ("pl_lock", "payload_lock"),
    ("pl_not_avail", "payload_not_available"),
    ("pl_not_home", "payload_not_home"),
    ("pl_off", "payload_off"),
    ("pl_on", "payload_on"),
    ("pl_open", "payload_open"),
    ("pl_osc_off", "payload_oscillation_off"),
    ("pl_osc_on", "payload_oscillation_on"),
    ("pl_paus", "payload_pause"),
    ("pl_prs", "payload_press"),
    ("pl_ret", "payload_return_to_base"),
    ("pl_rst", "payload_reset"),
    ("pl_rst_hum", "payload_reset_humidity"),
    ("pl_rst_mode", "payload_reset_mode"),
    ("pl_rst_pct", "payload_reset_percentage"),
    ("pl_rst_pr_mode", "payload_reset_preset_mode"),
    ("pl_stop", "payload_stop"),
    ("pl_stop_tilt", "payload_stop_tilt"),
    ("pl_strt", "payload_start"),
    ("pl_toff", "payload_turn_off"),
    ("pl_ton", "payload_turn_on"),
    ("pl_trig", "payload_trigger"),
    ("pl_unlk", "payload_unlock"),
    ("pos", "reports_position"),
    ("pos_clsd", "position_closed"),
    ("pos_open", "position_open"),
    ("pos_t", "position_topic"),
    ("pos_tpl", "position_template"),
    ("pow_cmd_t", "power_command_topic"),
    ("pow_cmd_tpl", "power_command_template"),
    ("pr_mode_cmd_t", "preset_mode_command_topic"),
    ("pr_mode_cmd_tpl", "preset_mode_command_template"),
    ("pr_mode_stat_t", "preset_mode_state_topic"),
    ("pr_mode_val_tpl", "preset_mode_value_template"),
    ("pr_modes", "preset_modes"),
    ("ptrn", "pattern"),
    ("r_tpl", "red_template"),
    ("rel_s", "release_summary"),
    ("rel_u", "release_url"),
    ("ret", "retain"),
    ("rgb_cmd_t", "rgb_command_topic"),
    ("rgb_cmd_tpl", "rgb_command_template"),
    ("rgb_stat_t", "rgb_state_topic"),
    ("rgb_val_tpl", "rgb_value_template"),
    ("rgbw_cmd_t", "rgbw_command_topic"),
    ("rgbw_cmd_tpl", "rgbw_command_template"),
    ("rgbw_stat_t", "rgbw_state_topic"),
    ("rgbw_val_tpl", "rgbw_value_template"),
    ("rgbww_cmd_t", "rgbww_command_topic"),
    ("rgbww_cmd_tpl", "rgbww_command_template"),
    ("rgbww_stat_t", "rgbww_state_topic"),
    ("rgbww_val_tpl", "rgbww_value_template"),
    ("send_cmd_t", "send_command_topic"),
    ("send_if_off", "send_if_off"),
    ("set_fan_spd_t", "set_fan_speed_topic"),
    ("set_pos_t", "set_position_topic"),
    ("set_pos_tpl", "set_position_template"),
    ("spd_rng_max", "speed_range_max"),
    ("spd_rng_min", "speed_range_min"),
    ("src_type", "source_type"),
    ("stat_cla", "state_class"),
    ("stat_closing", "state_closing"),
    ("stat_clsd", "state_closed"),
    ("stat_jam", "state_jammed"),
    ("stat_locked", "state_locked"),
    ("stat_locking", "state_locking"),
    ("stat_off", "state_off"),
    ("stat_on", "state_on"),
    ("stat_open", "state_open"),
    ("stat_opening", "state_opening"),
    ("stat_stopped", "state_stopped"),
    ("stat_t", "state_topic"),
    ("stat_tpl", "state_template"),
    ("stat_unlocked", "state_unlocked"),
    ("stat_unlocking", "state_unlocking"),
    ("stat_val_tpl", "state_value_template"),
    ("step", "step"),
    ("stop_cmd_t", "stop_command_topic"),
    ("stop_cmd_tpl", "stop_command_template"),
    ("strt_mw_cmd_t", "start_mowing_command_topic"),
    ("strt_mw_cmd_tpl", "start_mowing_command_template"),
    ("stype", "subtype"),
    ("sug_dsp_prc", "suggested_display_precision"),
    ("sup_clrm", "supported_color_modes"),
    ("sup_dur", "support_duration"),
    ("sup_feat", "supported_features"),
    ("sup_vol", "support_volume_set"),
    ("swing_h_mode_cmd_t", "swing_horizontal_mode_command_topic"),
    (
        "swing_h_mode_cmd_tpl",
        "swing_horizontal_mode_command_template",
    ),
    ("swing_h_mode_stat_t", "swing_horizontal_mode_state_topic"),
    (
        "swing_h_mode_stat_tpl",
        "swing_horizontal_mode_state_template",
    ),
    ("swing_h_modes", "swing_horizontal_modes"),
    ("swing_mode_cmd_t", "swing_mode_command_topic"),
    ("swing_mode_cmd_tpl", "swing_mode_command_template"),
    ("swing_mode_stat_t", "swing_mode_state_topic"),
    ("swing_mode_stat_tpl", "swing_mode_state_template"),
    ("swing_modes", "swing_modes"),
    ("t", "topic"),
    ("temp_cmd_t", "temperature_command_topic"),
    ("temp_cmd_tpl", "temperature_command_template"),
    ("temp_hi_cmd_t", "temperature_high_command_topic"),
    ("temp_hi_cmd_tpl", "temperature_high_command_template"),
    ("temp_hi_stat_t", "temperature_high_state_topic"),
    ("temp_hi_stat_tpl", "temperature_high_state_template"),
    ("temp_lo_cmd_t", "temperature_low_command_topic"),
    ("temp_lo_cmd_tpl", "temperature_low_command_template"),
    ("temp_lo_stat_t", "temperature_low_state_topic"),
    ("temp_lo_stat_tpl", "temperature_low_state_template"),
    ("temp_stat_t", "temperature_state_topic"),
    ("temp_stat_tpl", "temperature_state_template"),
    ("temp_unit", "temperature_unit"),
    ("tilt_clsd_val", "tilt_closed_value"),
    ("tilt_cmd_t", "tilt_command_topic"),
    ("tilt_cmd_tpl", "tilt_command_template"),
    ("tilt_max", "tilt_max"),
    ("tilt_min", "tilt_min"),
    ("tilt_opnd_val", "tilt_opened_value"),
    ("tilt_opt", "tilt_optimistic"),
    ("tilt_status_t", "tilt_status_topic"),
    ("tilt_status_tpl", "tilt_status_template"),
    ("tit", "title"),
    ("trns", "transition"),
    ("tz", "timezone"),
    ("uniq_id", "unique_id"),
    ("unit_of_meas", "unit_of_measurement"),
    ("url_t", "url_topic"),
    ("url_tpl", "url_template"),
    ("val_tpl", "value_template"),
    ("vis", "visible_by_default"),
    ("whit_cmd_t", "white_command_topic"),
    ("whit_scl", "white_scale"),
    ("xy_cmd_t", "xy_command_topic"),
    ("xy_cmd_tpl", "xy_command_template"),
    ("xy_stat_t", "xy_state_topic"),
    ("xy_val_tpl", "xy_value_template"),
];

/// Abbreviations inside the `device` block (`DEVICE_ABBREVIATIONS`). Sorted by abbreviation.
const DEVICE_ABBREVIATIONS: &[(&str, &str)] = &[
    ("cns", "connections"),
    ("cu", "configuration_url"),
    ("hw", "hw_version"),
    ("ids", "identifiers"),
    ("mdl", "model"),
    ("mdl_id", "model_id"),
    ("mf", "manufacturer"),
    ("name", "name"),
    ("sa", "suggested_area"),
    ("sn", "serial_number"),
    ("sw", "sw_version"),
];

/// Abbreviations inside the `origin` block (`ORIGIN_ABBREVIATIONS`). Sorted by abbreviation.
const ORIGIN_ABBREVIATIONS: &[(&str, &str)] = &[
    ("name", "name"),
    ("sw", "sw_version"),
    ("url", "support_url"),
];
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    /// Fails when the tables drift from `abbreviations.py` in size; update
    /// them from the source when it does.
    #[test]
    fn table_sizes() {
        assert_eq!(ABBREVIATIONS.len(), 275);
        assert_eq!(DEVICE_ABBREVIATIONS.len(), 11);
        assert_eq!(ORIGIN_ABBREVIATIONS.len(), 3);
    }

    #[test]
    fn tables_are_sorted_and_expansions_are_not_abbreviations() {
        for table in [ABBREVIATIONS, DEVICE_ABBREVIATIONS, ORIGIN_ABBREVIATIONS] {
            assert!(table.windows(2).all(|pair| pair[0].0 < pair[1].0));
            for (abbreviation, full) in table {
                if abbreviation != full {
                    assert_eq!(lookup(table, full), None, "{full}");
                }
            }
        }
    }

    #[test]
    fn topic_base() {
        let base = "home/x";
        assert_eq!(expand_topic("~/set", base).as_deref(), Some("home/x/set"));
        assert_eq!(expand_topic("set/~", base).as_deref(), Some("set/home/x"));
        assert_eq!(
            expand_topic("~/set/~", base).as_deref(),
            Some("~/set/home/x")
        );
        assert_eq!(expand_topic("set", base), None);
        assert_eq!(expand_topic("", base), None);
    }

    #[test]
    fn availability_entries() {
        let mut payload = json!({
            "~": "dev1",
            "avty": [{"t": "~/online", "pl_avail": "1"}],
            "avty_t": "~/status",
            "json_attr_tpl": "~/not/a/topic"
        });
        expand_component_payload(&mut payload);
        assert_eq!(
            payload,
            json!({
                "availability": [{"topic": "dev1/online", "payload_available": "1"}],
                "availability_topic": "dev1/status",
                "json_attributes_template": "~/not/a/topic"
            })
        );
    }
}
