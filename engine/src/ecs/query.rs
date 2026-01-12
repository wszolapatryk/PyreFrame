use std::any::TypeId;

/// A set of component types for querying entities.
/// Used to specify which components an entity must have.
pub trait ComponentSet {
    fn type_ids() -> Vec<TypeId>;
}

impl<A: 'static> ComponentSet for (A,) {
    fn type_ids() -> Vec<TypeId> {
        vec![TypeId::of::<A>()]
    }
}

impl<A: 'static, B: 'static> ComponentSet for (A, B) {
    fn type_ids() -> Vec<TypeId> {
        vec![TypeId::of::<A>(), TypeId::of::<B>()]
    }
}

/* If more than 2 components are needed in a set, here's an example of how to implement it:
impl<A: 'static, B: 'static, C: 'static> ComponentSet for (A, B, C) {
    fn type_ids() -> Vec<TypeId> {
        vec![TypeId::of::<A>(), TypeId::of::<B>(), TypeId::of::<C>()]
    }
}
*/
