use std::sync::Arc;

use maa_framework::{
    controller::{self, adb::MaaAdbControllerType},
    toolkit::{AdbDeviceInfo, MaaToolkit},
    MaaStatus,
};
use tauri::State;

use crate::{ControllerInstance, Instance, MaaZError, MaaZResult};

#[cfg(feature = "mock")]
use super::mock;

#[tauri::command]
pub async fn find_devices(toolkit: State<'_, MaaToolkit>) -> MaaZResult<Vec<AdbDeviceInfo>> {
    #[cfg(feature = "mock")]
    {
        let device = mock::mock_adb_device();
        return Ok(vec![device]);
    }

    let devices = toolkit.find_adb_device()?;
    Ok(devices)
}

#[tauri::command]
pub async fn connect_to_device(
    inst: State<'_, Arc<Instance>>,
    device: AdbDeviceInfo,
    controller: State<'_, Arc<ControllerInstance>>,
) -> MaaZResult<()> {
    #[cfg(feature = "mock")]
    {
        // return OK after 5s
        std::thread::sleep(std::time::Duration::from_secs(5));
        return Ok(());
    }

    let agent_path = "MaaAgentBinary";

    let controller_type = MaaAdbControllerType {
        touch_type: controller::adb::MaaAdbControllerTouchType::AutoDetect,
        key_type: controller::adb::MaaAdbControllerKeyType::Invalid,
        screencap_type: controller::adb::MaaAdbControlScreencapType::MinicapDirect,
    };

    let controller_instance = controller::MaaControllerInstance::new_adb(
        &device.adb_path,
        &device.adb_serial,
        controller_type,
        &device.adb_config,
        agent_path,
        None,
    );

    let mut controller = controller.lock().await;

    let connection = controller_instance.post_connect();

    let ret = controller_instance.wait(connection)?;

    if let MaaStatus::Success = ret {
        inst.bind_controller(&controller_instance)?;
        *controller = Some(controller_instance);
        Ok(())
    } else {
        Err(MaaZError::ConnectionError)
    }
}
