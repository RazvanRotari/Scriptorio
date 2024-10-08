use serde::{Deserialize, Serialize};
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signal {
    name: String,
    r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeciderConditions {
    comparator: String,
    constant: Option<i32>,
    copy_count_from_input: bool,
    first_signal: Option<Signal>,
    second_signal: Option<Signal>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ControlBehavior {
    decider_conditions: Option<DeciderConditions>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entity {
    entity_number: i32,
    name: String,
    position: Position,
    control_behavior: Option<ControlBehavior>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Blueprint {
    entities: Vec<Entity>,
    item: String,
    version: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlueprintWrapper {
    blueprint: Blueprint,
}
