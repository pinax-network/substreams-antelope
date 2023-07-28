use crate::pb::Action as ActionCall;

pub trait Action: Sized {
    const NAME: &'static str;

    fn match_call(action: &ActionCall) -> bool{
        action.name == Self::NAME
    }
    fn decode(action: &ActionCall) -> Result<Self, crate::errors::Error>;

    fn match_and_decode(call: impl AsRef<ActionCall>) -> Option<Self> {
        let call = call.as_ref();
        if !Self::match_call(call) {
            return None;
        }
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