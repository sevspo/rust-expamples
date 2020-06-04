pub fn run() {
  /* A prmitive Array:
    this works as we know it
  */
  let arr1 = [1, 2, 3];
  let arr2 = arr1;

  println!("{:?}", (arr1, arr2));

  /*
    With non primite types, if we do reassignment, the frist var does not point to that value anymore!
    We need to use a reference <&> to point to the resource.
    Does this also mean, that in rust you can actually point to a variable? and is there a use for that?
    Or does it mean, that a value would want to be copied without if we do not ad a ref, but it cannot becaue it is a reference type?\
    Or is this the place where ownership comes into paly.

    But if we then add a ref <&> it works again.
  */

  let vec1 = vec![1, 2, 3];
  let vec2 = &vec1;

  println!("{:?}", (&vec1, vec2));
}
