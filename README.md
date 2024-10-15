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
