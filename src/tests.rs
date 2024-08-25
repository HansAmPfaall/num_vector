#[cfg(test)]
use crate::NumVector;

#[test]
fn test_new_and_empty() {
    let vec1 = NumVector::new(vec![1, 2, 3]);
    assert_eq!(vec1.get(), &vec![1, 2, 3]);

    let empty_vec: NumVector<i32> = NumVector::empty();
    assert_eq!(empty_vec.get(), &Vec::<i32>::new());
}

#[test]
fn test_addition() {
    let vec1 = NumVector::new(vec![1, 2, 3]);
    let vec2 = NumVector::new(vec![4, 5, 6]);
    let result = &vec1 + &vec2;
    assert_eq!(result.get(), &vec![5, 7, 9]);
}

#[test]
fn test_subtraction() {
    let vec1 = NumVector::new(vec![4, 5, 6]);
    let vec2 = NumVector::new(vec![1, 2, 3]);
    let result = &vec1 - &vec2;
    assert_eq!(result.get(), &vec![3, 3, 3]);
}

#[test]
fn test_scalar_multiplication() {
    let vec = NumVector::new(vec![1, 2, 3]);
    let result = &vec * 2;
    assert_eq!(result.get(), &vec![2, 4, 6]);
}

#[test]
fn test_scalar_division() {
    let vec = NumVector::new(vec![2, 4, 6]);
    let result = &vec / 2;
    assert_eq!(result.get(), &vec![1, 2, 3]);
}

#[test]
#[should_panic(expected = "assertion failed: rhs != 0")]
fn test_scalar_division_by_zero() {
    let mut vec = NumVector::new(vec![1, 2, 3]);
    vec /= 0; // This should panic
}

#[test]
fn test_add_assign() {
    let mut vec1 = NumVector::new(vec![1, 2, 3]);
    let vec2 = NumVector::new(vec![4, 5, 6]);
    vec1 += &vec2;
    assert_eq!(vec1.get(), &vec![5, 7, 9]);
}

#[test]
fn test_sub_assign() {
    let mut vec1 = NumVector::new(vec![4, 5, 6]);
    let vec2 = NumVector::new(vec![1, 2, 3]);
    vec1 -= &vec2;
    assert_eq!(vec1.get(), &vec![3, 3, 3]);
}

#[test]
fn test_mul_assign() {
    let mut vec = NumVector::new(vec![1, 2, 3]);
    vec *= 2;
    assert_eq!(vec.get(), &vec![2, 4, 6]);
}

#[test]
fn test_div_assign() {
    let mut vec = NumVector::new(vec![2, 4, 6]);
    vec /= 2;
    assert_eq!(vec.get(), &vec![1, 2, 3]);
}

#[test]
fn test_empty_vector_operations() {
    let vec1: NumVector<i32> = NumVector::empty();
    let vec2: NumVector<i32> = NumVector::empty();
    assert_eq!((&vec1 + &vec2).get(), &vec![]);
    assert_eq!((&vec1 - &vec2).get(), &vec![]);
}

#[test]
fn test_mixed_operations() {
    let mut vec1 = NumVector::new(vec![10, 20, 30]);
    let vec2 = NumVector::new(vec![1, 2, 3]);
    vec1 += &vec2;
    assert_eq!(vec1.get(), &vec![11, 22, 33]);

    vec1 -= &vec2;
    assert_eq!(vec1.get(), &vec![10, 20, 30]);

    vec1 *= 2;
    assert_eq!(vec1.get(), &vec![20, 40, 60]);

    vec1 /= 2;
    assert_eq!(vec1.get(), &vec![10, 20, 30]);
}

#[test]
fn test_vector_inequality() {
    let vec1 = NumVector::new(vec![1, 2, 3]);
    let vec2 = NumVector::new(vec![4, 5, 6]);
    assert_ne!(vec1, vec2);
}

#[test]
fn test_large_vector_operations() {
    let vec1 = NumVector::new(vec![1; 1000]);
    let vec2 = NumVector::new(vec![2; 1000]);
    let result = &vec1 + &vec2;
    assert_eq!(result.get(), &vec![3; 1000]);

    let result = &vec2 - &vec1;
    assert_eq!(result.get(), &vec![1; 1000]);
}
