mod into_js_number;
use into_js_number::IntoJsNumber;

mod num;
pub use num::eq;
pub use num::gt;
pub use num::gte;
pub use num::lt;
pub use num::lte;

mod num_predicates;
use num_predicates::PredicateGT;
use num_predicates::PredicateGTE;
use num_predicates::PredicateLT;
use num_predicates::PredicateLTE;
