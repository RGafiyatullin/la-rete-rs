mod len_predicates;
use len_predicates::LenEq;
use len_predicates::LenGT;
use len_predicates::LenGTE;
use len_predicates::LenLT;
use len_predicates::LenLTE;

mod list;
pub use list::len_eq;
pub use list::len_gt;
pub use list::len_gte;
pub use list::len_lt;
pub use list::len_lte;
