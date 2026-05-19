pub trait WorkflowTrait {
    fn get(&self) -> String;
}

pub trait HasConstructor {
    fn new() -> Self;
}
