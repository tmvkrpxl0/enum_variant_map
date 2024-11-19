use std::marker::PhantomData;

pub trait VariantMap {
    const COUNT: usize;
    fn ordinal(&self) -> usize;
}

pub struct IndexContainer<T: VariantMap> {
    phantom_data: PhantomData<T>
}
