use enum_variant_map::VariantMap;

#[test]
fn basic() {
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

    assert_eq!(Test::A.ordinal(), 0);
    assert_eq!(Test::B.ordinal(), 1);
    assert_eq!(Test::C.ordinal(), 2);
    assert_eq!(Test::D.ordinal(), 3);
    assert_eq!(Test::E.ordinal(), 4);
    assert_eq!(Test::F.ordinal(), 5);
    assert_eq!(Test::G.ordinal(), 6);
}

#[test]
#[allow(dead_code)]
fn fields() {
    #[derive(VariantMap)]
    enum Test {
        A(i32),
        B {
            _named1: String,
            _named2: &'static i32,
        },
        C(i64, i32),
        D,
    }

    assert_eq!(Test::A(43223).ordinal(), 0);
    assert_eq!(
        (Test::B {
            _named1: String::from("test"),
            _named2: &10
        })
        .ordinal(),
        1
    );
    assert_eq!(Test::C(10, 0).ordinal(), 2);
    assert_eq!(Test::D.ordinal(), 3);
}

#[test]
fn generic() {
    #[derive(VariantMap)]
    enum Values<T> {
        A(T),
        B,
        C
    }

    assert_eq!(Values::<i32>::A(2).ordinal(), 0);
    assert_eq!(Values::<u32>::A(3).ordinal(), 0);
    assert_eq!(Values::<i32>::B.ordinal(), 1);
    assert_eq!(Values::<u32>::B.ordinal(), 1);
    assert_eq!(Values::<i32>::C.ordinal(), 2);
    assert_eq!(Values::<u32>::C.ordinal(), 2);
}

#[test]
#[allow(dead_code)]
fn lifetime() {
    #[derive(VariantMap)]
    enum Reference<'a, T> {
        A(&'a T),
        B,
        C
    }

    let two = 2;
    let three = 3;
    assert_eq!(Reference::<i32>::A(&two).ordinal(), 0);
    assert_eq!(Reference::<u32>::A(&three).ordinal(), 0);
    assert_eq!(Reference::<i32>::B.ordinal(), 1);
    assert_eq!(Reference::<u32>::B.ordinal(), 1);
    assert_eq!(Reference::<i32>::C.ordinal(), 2);
    assert_eq!(Reference::<u32>::C.ordinal(), 2);
}
