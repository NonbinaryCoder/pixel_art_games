#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ordering {
    #[default]
    Default,
    SideToSide,
    Spiral,
}
