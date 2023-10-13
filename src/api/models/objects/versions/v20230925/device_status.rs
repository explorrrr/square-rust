//! DeviceStatus

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::device_status_category::DeviceStatusCategoryV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatusV20230925 {
    pub category: Option<DeviceStatusCategoryV20230925>,
}
