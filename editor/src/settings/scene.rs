use fyrox::core::algebra::Vector3;
use fyrox::core::pool::ErasedHandle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct SceneCameraSettings {
    pub position: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for SceneCameraSettings {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 1.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct NodeInfo {
    pub is_expanded: bool,
}

impl Default for NodeInfo {
    fn default() -> Self {
        Self { is_expanded: true }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Default)]
pub struct SceneSettings {
    pub camera_settings: SceneCameraSettings,
    pub node_infos: HashMap<ErasedHandle, NodeInfo>,
}
