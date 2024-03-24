use crate::pb::ActionTrace;

pub trait Action: Sized {
    const NAME: &'static str;

    fn decode(trace: &ActionTrace) -> Result<Self, crate::errors::Error>;

    // if action name and parameters match - return decoded action
    fn match_and_decode(trace: impl AsRef<ActionTrace>) -> Option<Self> {
        let trace = trace.as_ref();
        if trace.action.as_ref().unwrap().name != Self::NAME {
            return None;
        }
        Self::decode(trace).ok()
    }
}

impl AsRef<ActionTrace> for ActionTrace {
    fn as_ref(&self) -> &Self {
        self
    }
}
