// FIXME: Make me pass! Diff budget: 10 lines.
// Do not `use` any items.

// Do not change the following two lines.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
struct IntWrapper(isize);

// Implement a generic function here
// fn max...
fn max<T: PartialOrd, P: From<T> + PartialOrd>(a: T, b: P) -> P {
   let A = P::from(a);
   if A >= b {
      return A;
   }
   else {
      return b;
   }
}

#[test]
fn expressions() {
    assert_eq!(max(1usize, 3), 3);
    assert_eq!(max(1u8, 3), 3);
    assert_eq!(max(1u8, 3), 3);
    assert_eq!(max(IntWrapper(120), IntWrapper(248)), IntWrapper(248));
}
