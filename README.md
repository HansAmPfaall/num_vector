# Vector Math Library

This library provides a `NumVector` struct in Rust, enabling basic vector arithmetic operations. The library is designed to work with generic types that implement the required arithmetic traits.

## Features

- **Vector Initialization**: Easily create vectors with `Vector::new(vec)` or initialize an empty vector with `Vector::empty()`.
- **Element-wise Arithmetic**: Perform addition or subtraction on vectors element-wise.
- **Scalar Operations**: Multiply or divide all elements of a vector by a scalar value.
- **In-Place Modifications**: Supports modifying vectors directly using operators like `+=`, `-=`, `*=`, and `/=`.
- **Dereferencing**: Allows direct access to the underlying vector using dereferencing.


## Usage

To use this library in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
num_vector = "0.1.0"
```

Then, in your Rust code, import the library and start using the vector math operations:

```rust
use num_vector::NumVector;

fn main() {
    let vec = NumVector::new(vec![1, 2, 3]);
    let mut vec1 = NumVector::new(vec![1, 2, 3]);
    let vec2 = NumVector::new(vec![4, 5, 6]);

    let sum: NumVector<i32> = &vec1 + &vec2;    // Element-wise addition
    let diff: NumVector<i32> = &vec1 - &vec2;   // Element-wise subtraction
    let scaled: NumVector<i32> = &vec * 2;      // Scalar multiplication
    let divided: NumVector<i32> = &vec / 2;     // Scalar division
    vec1 += &vec2;                              // In-place addition
    vec1 -= &vec2;                              // In-place subtraction
    vec1 *= 2;                                  // In-place scalar multiplication
    vec1 /= 2;                                  // In-place scalar division
    let data: &Vec<i32> = vec.get();            // Access the underlying Vec<T>
    let first_element: i32 = vec[0];            // Using deref coercion
}
```

## Future Improvements
- Allocate memory on the stack instead of the heap for vectors of small sizes.
- Write a Display implementation for the Vector struct to enable printing vectors.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/HansAmPfaall/num_vector).

## License

This library is licensed under the [MIT License](LICENSE).
