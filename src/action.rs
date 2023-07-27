use crate::pb::Action as ActionCall;
// use serde::{Deserialize, Deserializer, Serialize};

pub trait Action: Sized {
    const NAME: &'static str;

    fn match_call(action: &ActionCall) -> bool{
        action.name == Self::NAME
    }
    fn decode(action: &ActionCall) -> Result<Self, String>;


    /// Attempts to match and decode the action call.
    /// If `Self::match_call(call)` is `false`, returns `None`.
    /// If it matches, but decoding fails, logs the decoding error and returns `None`.
    fn match_and_decode(call: impl AsRef<ActionCall>) -> Option<Self> {
        let call = call.as_ref();
        if !Self::match_call(call) {
            return None;
        }
        substreams::log::info!("match_and_decode about to decode: {:?}", call);

        match Self::decode(&call) {
            Ok(action) => Some(action),
            Err(err) => {
                substreams::log::info!(
                    "Call for action `{}` matched but failed to decode with error: {}",
                    Self::NAME,
                    err
                );
                None
            }
        }
    }
}

impl AsRef<ActionCall> for ActionCall {
    fn as_ref(&self) -> &Self {
        self
    }
}