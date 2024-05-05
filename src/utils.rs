use iced::{Program, program::Definition};

pub trait Patchable: Sized {
    fn patch(self, patcher: impl FnOnce(Self) -> Self) -> Self;
}

impl<P: Definition> Patchable for Program<P> {
    fn patch(self, patcher: impl FnOnce(Self) -> Self) -> Self {
        patcher(self)
    }
}
