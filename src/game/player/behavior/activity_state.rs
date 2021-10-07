// TODO: Cooldown layer for movement

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityState {
    Exempt,
    Ready,
    Queuing,
    Busy,
    Cooldown,
}