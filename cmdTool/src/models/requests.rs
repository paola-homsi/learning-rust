/// What the user asked for: which file, and which actions to run on it.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct CmdRequest {
    pub file: Option<String>,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Size,
    Lines,
    Help,
}
