use enum_variant_map::{EnumVariantMap, VariantMap};
use enum_variant_map_macro::get;

#[test]
fn store_test() {
    #[derive(Copy, Clone, VariantMap)]
    enum Test {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
    }

    let mut map = EnumVariantMap::<Test>::new();
    let mut values = vec![];
    values.push(Test::A);
    values.push(Test::B);
    values.push(Test::C);
    values.push(Test::D);
    values.push(Test::E);
    values.push(Test::F);
    values.push(Test::G);


    values.into_iter().enumerate().for_each(|(size, v)| {
        assert_eq!(map.len(), size);
        map.insert(v);
    });
    assert_eq!(map.capacity(), Test::COUNT);
    assert_eq!(map.len(), Test::COUNT);
}

#[test]
#[allow(dead_code)]
fn get_test() {
    #[derive(Clone, Eq, PartialEq, VariantMap, Debug)]
    enum Test {
        A(i32),
        B {
            _named1: String,
            _named2: &'static i32,
        },
        C(i64, i32),
        D,
    }

    let mut map = EnumVariantMap::<Test>::new();
    let a = Test::A(1);
    let b = Test::B {
        _named1: "B".to_string(),
        _named2: &2,
    };
    let c = Test::C(3, 4);
    let d = Test::D;

    map.insert(a.clone());
    map.insert(b.clone());
    map.insert(c.clone());
    map.insert(d.clone());

    assert_eq!(&a, get!(map, Test::A));
    assert_ne!(&b, get!(map, Test::A));
    assert_ne!(&c, get!(map, Test::A));
    assert_ne!(&d, get!(map, Test::A));

    assert_ne!(&a, get!(map, Test::B));
    assert_eq!(&b, get!(map, Test::B));
    assert_ne!(&c, get!(map, Test::B));
    assert_ne!(&d, get!(map, Test::B));

    assert_ne!(&a, get!(map, Test::C));
    assert_ne!(&b, get!(map, Test::C));
    assert_eq!(&c, get!(map, Test::C));
    assert_ne!(&d, get!(map, Test::C));

    assert_ne!(&a, get!(map, Test::D));
    assert_ne!(&b, get!(map, Test::D));
    assert_ne!(&c, get!(map, Test::D));
    assert_eq!(&d, get!(map, Test::D));
}

#[test]
fn test_generic() {
    #[derive(Clone, Eq, PartialEq, VariantMap, Debug)]
    enum Values<T> {
        A(T),
        B,
        C
    }

    let mut map = EnumVariantMap::<Values<i32>>::new();
    assert_eq!(map.capacity(), Values::<i32>::COUNT);

    let a = Values::A(7);
    let b = Values::B;
    let c = Values::C;

    let data = [a, b, c];
    data.clone().into_iter().enumerate().for_each(|(size, v)| {
        assert_eq!(map.len(), size);
        map.insert(v);
    });

    assert_eq!(map.len(), Values::<i32>::COUNT);

    assert_eq!(&data[0], get!(map, Values::<i32>::A));
    assert_eq!(&data[1], get!(map, Values::<i32>::B));
    assert_eq!(&data[2], get!(map, Values::<i32>::C));
}

#[test]
#[allow(dead_code)]
fn lifetime() {
    #[derive(VariantMap, Eq, PartialEq, Debug)]
    enum Reference<'a, T> {
        A(&'a T),
        B,
        C
    }

    let two = 2;
    let mut map = EnumVariantMap::<Reference<'_, i32>>::new();
    map.insert(Reference::A(&two));
    map.insert(Reference::B);
    map.insert(Reference::C);

    assert_eq!(&Reference::<'_, i32>::A(&two), get!(map, Reference::<'_, i32>::A));
    assert_eq!(&Reference::<'_, i32>::B, get!(map, Reference::<'_, i32>::B));
    assert_eq!(&Reference::<'_, i32>::C, get!(map, Reference::<'_, i32>::C));
}
