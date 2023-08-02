use crate::pb::ActionTrace;

pub trait Action: Sized {
    const NAME: &'static str;

    fn match_call(trace: &ActionTrace) -> bool {
        trace.action.as_ref().unwrap().name == Self::NAME
    }
    fn decode(trace: &ActionTrace) -> Result<Self, crate::errors::Error>;

    fn match_and_decode(trace: impl AsRef<ActionTrace>) -> Option<Self> {
        let trace = trace.as_ref();
        if !Self::match_call(trace) {
            return None;
        }
        match Self::decode(trace) {
            Ok(action) => Some(action),
            Err(err) => {
                substreams::log::info!("Call for action `{}` matched but failed to decode with error: {}", Self::NAME, err);
                None
            }
        }
    }
}

impl AsRef<ActionTrace> for ActionTrace {
    fn as_ref(&self) -> &Self {
        self
    }
}
