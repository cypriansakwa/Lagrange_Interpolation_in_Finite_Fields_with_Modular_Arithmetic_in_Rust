## Overview
- This Rust program implements Lagrange interpolation in finite fields using modular arithmetic. Given a set of points in the form $(x,y)$, the program computes the polynomial $P(x)$ that passes through these points, evaluated modulo a specified integer. The program is particularly useful for applications in cryptography, coding theory, and numeric computations where modular arithmetic is required.
## Features
 - **Modular Arithmetic:** Handles calculations using modulo $m$ to ensure results stay within the finite field.
 - **Lagrange Interpolation:** Computes the polynomial using the Lagrange basis polynomial method.
 - **Custom Input:** Users can define their own set of points for interpolation and the modulus.
## Dependencies
- This project uses the following dependencies:
  - `num`: Provides integer algorithms for computing greatest common divisors and other number theoretic functions.
  - `num-bigint`: Supports arbitrary-precision arithmetic for big integers.
  - `num-traits`: Provides numerical traits for Rust's primitive types.
- Add the following lines to your Cargo.toml:
>```
>[dependencies]
>num = "0.4"
>num-bigint = "0.4"
>num-traits = "0.2"
## Example Usage
- The program computes the polynomial for the following points: $(0,4),(-2,1),(2,3)$.
- The results will show the value of $P(x)$ for $x=0,1,2,3,4 modulo $5$.
## Example Output
- The expected output will show the evaluations of the polynomial at the specified $x$ values:
>```
>P(0) = <value>
>P(1) = <value>
>P(2) = <value>
>P(3) = <value>
>P(4) = <value>
  
## Requirements
- Rust installed on your machine. (If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it).

## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

## Usage
- To use this code, you can clone or download this repository.
- Compile and run the Rust program using the following command:
>```
>cargo build
>cargo run
## Acknowledgments
- Rust

## Clone the repository:

   ```bash
   git clone https://github.com/cypriansakwa/Lagrange_Interpolation_in_Finite_Fields_with_Modular_Arithmetic_in_Rust.git
   cd Lagrange_Interpolation_in_Finite_Fields_with_Modular_Arithmetic_in_Rust
