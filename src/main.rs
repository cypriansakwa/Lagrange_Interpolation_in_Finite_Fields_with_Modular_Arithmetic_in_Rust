// Helper function to perform modular arithmetic
fn mod_inv(a: i64, m: i64) -> i64 {
    let mut t = 0;
    let mut newt = 1;
    let mut r = m;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        let temp_t = t;
        t = newt;
        newt = temp_t - quotient * newt;
        let temp_r = r;
        r = newr;
        newr = temp_r - quotient * newr;
    }

    if r > 1 {
        panic!("a is not invertible");
    }

    if t < 0 {
        t += m;
    }

    t
}

// Function to compute the Lagrange basis polynomial for given index `i`
fn lagrange_basis(x_vals: &[i64], i: usize, x: i64, modulus: i64) -> i64 {
    let mut numerator = 1;
    let mut denominator = 1;

    for (j, &xj) in x_vals.iter().enumerate() {
        if i != j {
            numerator = (numerator * (x - xj)).rem_euclid(modulus);
            denominator = (denominator * (x_vals[i] - xj)).rem_euclid(modulus);
        }
    }

    // Multiply the numerator by the modular inverse of the denominator
    numerator * mod_inv(denominator, modulus) % modulus
}

// Function to compute the Lagrange interpolation polynomial
fn lagrange_interpolation(points: &[(i64, i64)], modulus: i64) -> impl Fn(i64) -> i64 + '_ {
    let x_vals: Vec<i64> = points.iter().map(|(x, _)| *x).collect();
    let y_vals: Vec<i64> = points.iter().map(|(_, y)| *y).collect();

    move |x: i64| {
        let mut result = 0;
        for i in 0..points.len() {
            let li_x = lagrange_basis(&x_vals, i, x, modulus);
            result = (result + y_vals[i] * li_x).rem_euclid(modulus);
        }
        result
    }
}

fn main() {
    // Given points in the form (x, y)
    let points = vec![(0, 4), (-2, 1), (2, 3)];
    let modulus = 5;

    // Get the interpolation polynomial
    let poly = lagrange_interpolation(&points, modulus);

    // Display the polynomial at different x values
    for x in 0..modulus {
        println!("P({}) = {}", x, poly(x));
    }
}
