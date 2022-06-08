use unpat::unpat;

#[derive(PartialEq, Debug)]
struct TupleStruct(i32, i32);

#[derive(PartialEq, Debug)]
struct TestStruct {
    int: i32,
    tuple_struct: TupleStruct,
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum Test {
    Int(i32),
    Tuple(i32, f64),
    TupleStruct(TupleStruct),
    Named { x: i32, y: f64 },
    TestStruct(TestStruct),
}

#[test]
fn match_enum() {
    let int = Test::Int(1);
    unpat!(Test::Int(v) <- int);
    assert_eq!(v, 1);
}

#[test]
fn match_tuple_enum() {
    let tup = Test::Tuple(1, 3.5);
    unpat!(Test::Tuple(a, b) <- tup);
    assert_eq!((a, b), (1, 3.5));
}

#[test]
fn match_tuple_struct_enum() {
    let tup_struct = Test::TupleStruct(TupleStruct(1, 2));
    unpat!(Test::TupleStruct(TupleStruct(a, _)) <- tup_struct);
    assert_eq!(a, 1);
}

#[test]
fn match_named_enum() {
    let named = Test::Named { x: 3, y: 5.1 };
    {
        unpat!(Test::Named{x, y} <- named);
        assert_eq!((x, y), (3, 5.1));
    }
    {
        unpat!(Test::Named{x: a, y} <- named);
        assert_eq!((a, y), (3, 5.1));
    }
    {
        unpat!(Test::Named{y, ..} <- named);
        assert_eq!(y, 5.1);
    }
}

#[test]
fn match_struct_enum() {
    let test_struct = Test::TestStruct(TestStruct {
        int: 1,
        tuple_struct: TupleStruct(2, 3),
    });
    {
        unpat!(
            Test::TestStruct(
                TestStruct { int, tuple_struct: TupleStruct(x, y) }
            ) <- test_struct
        );
        assert_eq!((int, x, y), (1, 2, 3));
    }
    {
        unpat!(
            Test::TestStruct(
                TestStruct { int, tuple_struct: v @ TupleStruct(x, y) }
            ) <- test_struct
        );
        assert_eq!((int, x, y), (1, 2, 3));
        assert_eq!(v, TupleStruct(2, 3));
    }
}

#[test]
fn match_mut_enum() {
    let mut test_struct = Test::TestStruct(TestStruct {
        int: 1,
        tuple_struct: TupleStruct(2, 3),
    });

    unpat!(
        Test::TestStruct(
            TestStruct { ref mut int, tuple_struct: TupleStruct(ref mut x, ref mut y) }
        ) <- test_struct
    );
    assert_eq!((*int, *x, *y), (1, 2, 3));
}
