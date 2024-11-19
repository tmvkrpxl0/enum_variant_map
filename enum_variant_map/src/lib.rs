#![feature(generic_const_exprs)]

pub use enum_variant_map_types::VariantMap;
pub use enum_variant_map_macro::VariantMap;
pub use enum_variant_map_macro::get;

#[derive(Clone, Debug)]
pub struct EnumVariantMap<E: VariantMap> where [Option<E>; E::COUNT]: {
    storage: [Option<E>; E::COUNT]
}

impl<E: VariantMap> EnumVariantMap<E> where [Option<E>; E::COUNT]:, [Option<E>; E::COUNT]: Default {
    pub fn new() -> EnumVariantMap<E> {
        EnumVariantMap {
            storage: Default::default()
        }
    }

    pub fn insert(&mut self, value: E) -> Option<E> {
        unsafe {
            debug_assert!(value.ordinal() < E::COUNT);
            self.storage.get_unchecked_mut(value.ordinal()).replace(value)
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<&E> {
        match self.storage.get(index) {
            None => None,
            Some(v) => v.as_ref()
        }
    }

    pub unsafe fn get_by_index_unsafe(&self, index: usize) -> &E {
        self.storage.get_unchecked(index).as_ref().unwrap_unchecked()
    }

    pub fn has_variant(&self, index: usize) -> bool {
        self.storage.get(index).map(|v| v.is_some()).unwrap_or(false)
    }

    pub fn capacity(&self) -> usize {
        E::COUNT
    }

    pub fn len(&self) -> usize {
        self.storage.iter().filter(|v| v.is_some()).count()
    }
}
