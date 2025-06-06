//
// Copyright 2021 Signal Messenger, LLC
// SPDX-License-Identifier: AGPL-3.0-only
//

include!(concat!(env!("OUT_DIR"), "/group_call.rs"));

mod extensions {
    use crate::{
        protos::sfu_to_device::{peek_info::PeekDeviceInfo, PeekInfo},
        sfu::CallSignalingInfo,
    };

    impl From<CallSignalingInfo> for PeekInfo {
        fn from(value: CallSignalingInfo) -> Self {
            Self {
                era_id: value.era_id.map(|era_id| hex::encode(era_id.as_slice())),
                max_devices: None,
                creator: Some(value.creator_id.into()),
                devices: value
                    .client_ids
                    .into_iter()
                    .map(|(demux_id, user_id)| PeekDeviceInfo {
                        demux_id: Some(demux_id.as_u32()),
                        opaque_user_id: Some(user_id.into()),
                    })
                    .collect(),
                pending_devices: value
                    .pending_client_ids
                    .into_iter()
                    .map(|(demux_id, user_id)| PeekDeviceInfo {
                        demux_id: Some(demux_id.as_u32()),
                        opaque_user_id: user_id.map(|id| id.into()),
                    })
                    .collect(),
                call_link_state: None,
            }
        }
    }

    impl From<&CallSignalingInfo> for PeekInfo {
        fn from(value: &CallSignalingInfo) -> Self {
            Self {
                era_id: value
                    .era_id
                    .as_ref()
                    .map(|era_id| hex::encode(era_id.as_slice())),
                max_devices: None,
                creator: Some(value.creator_id.clone().into()),
                devices: value
                    .client_ids
                    .iter()
                    .map(|(demux_id, user_id)| PeekDeviceInfo {
                        demux_id: Some(demux_id.as_u32()),
                        opaque_user_id: Some(user_id.clone().into()),
                    })
                    .collect(),
                pending_devices: value
                    .pending_client_ids
                    .iter()
                    .map(|(demux_id, user_id)| PeekDeviceInfo {
                        demux_id: Some(demux_id.as_u32()),
                        opaque_user_id: user_id.clone().map(|id| id.into()),
                    })
                    .collect(),
                call_link_state: None,
            }
        }
    }
}
