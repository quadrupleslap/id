#![no_std]

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Id(::core::any::TypeId);

#[macro_export]
macro_rules! id {
    () => {{
        struct Nonsense;
        $crate::Id(::core::any::TypeId::of::<Nonsense>())
    }}
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        let a = id!();
        let b = id!();
        assert!(a == a);
        assert!(b == b);
        assert!(a != b);
    }

    #[test]
    fn duplicate() {
        struct Nonsense;
        let _ = Nonsense;
        let a = id!();
        let b = id!();
        assert!(a != b);
    }
}
