mod heq;
pub use heq::ArcDynHEq;
pub use heq::HEq;

mod table;
pub use table::Table as HEqTable;

mod trivial;
pub use trivial::HEqTrivial;
