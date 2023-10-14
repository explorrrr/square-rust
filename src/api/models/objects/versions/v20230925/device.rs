//! Device

use serde::{Deserialize, Serialize};

use super::{
    component::ComponentV20230925, device_attributes::DeviceAttributesV20230925, device_status::DeviceStatusV20230925,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceV20230925 {
    /// Read only A synthetic identifier for the device. The identifier includes a standardized prefix and is otherwise an opaque id generated from key device fields.
    pub id: Option<String>,
    /// A collection of DeviceAttributes representing the device.
    pub attributes: DeviceAttributesV20230925,
    /// A list of components applicable to the device.
    pub components: Option<Vec<ComponentV20230925>>,
    /// Read only The current status of the device.
    pub status: Option<DeviceStatusV20230925>,
}
