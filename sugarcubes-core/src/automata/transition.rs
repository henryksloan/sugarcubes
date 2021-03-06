/// Defines a generic transition, effectively a directed edge in a graph
pub trait Transition: Default + PartialEq + Clone + Copy {
    fn from(&self) -> u32;
    fn to(&self) -> u32;
}
