use unpat::{try_unpat, unpat};

#[derive(PartialEq, Debug)]
struct TestTuple(i32, i32);

#[derive(PartialEq, Debug)]
struct TestStruct {
    int: i32,
    tuple: TestTuple,
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum TestEnum {
    Int(i32),
    Tuple(i32, f64),
    TupleStruct(TestTuple),
    Named { x: i32, y: f64 },
    Struct(TestStruct),
}

#[test]
fn match_enum() {
    let int = TestEnum::Int(1);
    unpat!(TestEnum::Int(v) <- int); // `v = 3` binding is created here.
    assert_eq!(v, 1);
}

#[test]
fn match_tuple_enum() {
    let tup = TestEnum::Tuple(1, 3.5);
    unpat!(TestEnum::Tuple(a, b) <- tup);
    assert_eq!((a, b), (1, 3.5));
}

#[test]
fn match_tuple_struct_enum() {
    let tup_struct = TestEnum::TupleStruct(TestTuple(1, 2));
    unpat!(TestEnum::TupleStruct(TestTuple(a, _)) <- tup_struct);
    assert_eq!(a, 1);
}

#[test]
fn match_named_enum() {
    let named = TestEnum::Named { x: 3, y: 5.1 };
    {
        unpat!(TestEnum::Named{x, y} <- named);
        assert_eq!((x, y), (3, 5.1));
    }
    {
        unpat!(TestEnum::Named{x: a, y} <- named);
        assert_eq!((a, y), (3, 5.1));
    }
    {
        unpat!(TestEnum::Named{y, ..} <- named);
        assert_eq!(y, 5.1);
    }
}

#[test]
fn match_struct_enum() {
    let test_struct = TestEnum::Struct(TestStruct {
        int: 1,
        tuple: TestTuple(2, 3),
    });
    {
        unpat!(
            TestEnum::Struct(
                TestStruct { int, tuple: TestTuple(x, y) }
            ) <- test_struct
        );
        assert_eq!((int, x, y), (1, 2, 3));
    }
    {
        unpat!(
            TestEnum::Struct(
                TestStruct { int, tuple: v @ TestTuple(x, y) }
            ) <- test_struct
        );
        assert_eq!((int, x, y), (1, 2, 3));
        assert_eq!(v, TestTuple(2, 3));
    }
}

#[test]
fn match_mut_enum() {
    let mut test_struct = TestEnum::Struct(TestStruct {
        int: 1,
        tuple: TestTuple(2, 3),
    });

    unpat!(
        TestEnum::Struct(
            TestStruct { ref mut int, tuple: TestTuple(ref mut x, ref mut y) }
        ) <- test_struct
    );
    assert_eq!((*int, *x, *y), (1, 2, 3));
}

#[test]
fn match_struct() {
    let test_struct = TestStruct {
        int: 1,
        tuple: TestTuple(2, 3),
    };

    unpat!(
        TestStruct { int, tuple: TestTuple(x, y) } <- test_struct
    );
    assert_eq!((int, x, y), (1, 2, 3));
}

fn try_match_tuple(tuple: TestEnum) -> Result<(i32, f64), String> {
    try_unpat!(TestEnum::Tuple(a, b) <- tuple, String::from(""));
    Ok((a, b))
}

#[test]
fn try_match_tuple_enum() {
    assert_eq!((1, 3.5), try_match_tuple(TestEnum::Tuple(1, 3.5)).unwrap());
    assert!(try_match_tuple(TestEnum::Int(1)).is_err());
}
