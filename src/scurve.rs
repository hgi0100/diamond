// scurve
// Neil Crago
// 02/05/24

/*
Code to build a simple function in Rust to represent:-

A.) a basic S-curve and variations for
B.) front and
C.) back-loaded versions:

Explanation:

1. `s_curve` function:
   - Takes three arguments:
     - `t`: Represents the normalized time (0.0 to `i`).
     - `a`: Controls the starting curvature (higher `a` for more emphasis at the beginning).
     - `b`: Controls the ending curvature (higher `b` for more emphasis at the end).
2. The function uses a cubic equation with `t` cubed for a smooth S-shape.
3. `(3.0 - 2.0 * t)` acts as a weighting factor that changes over time.
4. `(a + b * t)` combines the starting and ending curvatures.
5. `do_s_curve` function:
   - Loops through time values from 0.0 to `i` with increments.
   - Calculates the S-curve value for each time step using different combinations of `a` and `b`.
   - Prints the corresponding time (`t`) and S-curve value (`y`).

Adjust the function and parameters to achieve different S-curve shapes.
*/

pub mod scurves {

    pub fn s_curve(t: f64, a: f64, b: f64) -> f64 {
        t * t * (3.0 - 2.0 * t) * (a + b * t)
    }

    pub fn do_s_curve(i: &i32) -> Vec<(f64, f64, f64)> {
        let mut scvec: Vec<(f64, f64, f64)> = Vec::new();

        for t in 0..=*i {
            let s = *i as f64;
            let q = t as f64 / s;

            // Example S-curve
            let a = s_curve(q, 1.0, 0.0);

            // Back-loaded S-curve (more emphasis at the end)
            let b = s_curve(q, 0.0, 1.0); // 0.0, 1.0

            // Front-loaded S-curve (more emphasis at the beginning)
            let c = s_curve(q, 2.0, -1.0);

            scvec.push((a, b, c));
        }
        scvec
    }
}
