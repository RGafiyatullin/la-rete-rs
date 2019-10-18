#[derive(Debug, Fail)]
pub enum TrieBuildFailure {
    #[fail(display = "TrieBuildFailure::Unimplemented: {}", _0)]
    Unimplemented(&'static str),

    #[fail(display = "TrieBuildFailure::UnreachableRule")]
    UnreachableRule,
}
