macro_rules! field_get_ref {
    ($field: ident, $func: ident, $type: ty) => {
        pub fn $func(&self) -> &$type {
            &self.$field
        }
    };
}

macro_rules! field_get_ref_mut {
    ($field: ident, $func: ident, $type: ty) => {
        pub fn $func(&mut self) -> &mut $type {
            &mut self.$field
        }
    }
}

// macro_rules! field_get_opt_ref {
//     ($field: ident, $func: ident, $type: ty) => {
//         pub fn $func(&self) -> Option<&$type> {
//             self.$field.as_ref()
//         }
//     };
// }

// macro_rules! field_update_opt_none {
//    ($field: ident, $func: ident) => {
//        pub fn $func(self) -> Self {
//            let $field = None;
//            Self {
//                $field,
//                .. self
//            }
//        }
//    }
// }

// macro_rules! field_update_opt_some {
//    ($field: ident, $func: ident, $type: ty) => {
//        pub fn $func(self, $field: Option<$type>) -> Self {
//         Self {
//             $field,
//             .. self
//         }
//     }
//    }
// }

// macro_rules! field_get_copy {
//     ($field: ident, $func: ident, $type: ty) => {
//         pub fn $func(&self) -> $type {
//             self.$field
//         }
//     };
// }

macro_rules! field_update {
    ($field: ident, $func: ident, $type: ty) => {
        pub fn $func(self, $field: $type) -> Self {
            Self {
                $field,
                .. self
            }
        }
    };
}

// macro_rules! enum_variant_from {
//     ($target: tt, $variant: ident, $source: ty) => {
//         impl From<$source> for $target {
//             fn from(inner: $source) -> Self {
//                 $target::$variant(inner)
//             }
//         }
//     };
// }
