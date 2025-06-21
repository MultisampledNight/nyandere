//! Auxiliary macros and the works.

derive_alias! {
    #[derive(NotOrd!)] = #[derive(Clone, Debug, PartialEq, Eq)];
    #[derive(Owned!)] = #[derive($crate::aux::NotOrd!, PartialOrd, Ord, Hash)];
    #[derive(Stack!)] = #[derive($crate::aux::Owned!, Copy)];
}
