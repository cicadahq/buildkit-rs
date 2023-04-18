use crate::utils::OperationOutput;

pub trait MultiBorrowedOutput<'a> {
    fn output(&'a self, number: u32) -> OperationOutput<'a>;
}

pub trait MultiBorrowedLastOutput<'a> {
    fn last_output(&'a self) -> Option<OperationOutput<'a>>;
}

pub trait MultiOwnedOutput<'a> {
    fn output(&self, number: u32) -> OperationOutput<'a>;
}

pub trait MultiOwnedLastOutput<'a> {
    fn last_output(&self) -> Option<OperationOutput<'a>>;
}

pub trait SingleBorrowedOutput<'a> {
    fn output(&'a self) -> OperationOutput<'a>;
}

pub trait SingleOwnedOutput<'a> {
    fn output(&self) -> OperationOutput<'a>;
}
