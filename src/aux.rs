//! Auxiliary macros and the works.

derive_alias! {
    #[derive(Owned!)] = #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)];
    #[derive(Common!)] = #[derive($crate::aux::Owned!, Copy)];
}
