Quick code to generate:- 
 
1.) an S-curve
2.) a front-loaded curve
3.) a back-loaded curve
4.) reveals the `california envelope` when the resulting CSV is plotted

Note: Data is written to `diamond.csv`

Explanation:

1. `s_curve` function:
   - Takes three arguments:
     - `t`: Represents the normalized time (0.0 to 1.0).
     - `a`: Controls the starting curvature (higher `a` for more emphasis at the beginning).
     - `b`: Controls the ending curvature (higher `b` for more emphasis at the end).
2. The function uses a cubic equation with `t` cubed for a smooth S-shape.
3. `(3.0 - 2.0 * t)` acts as a weighting factor that changes over time.
4. `(a + b * t)` combines the starting and ending curvatures.
5. `main` function:
   - Loops through time values from 0.0 to 1.0 with increments.
   - Calculates the S-curve value for each time step using different combinations of `a` and `b`.
   - Prints the corresponding time (`t`) and S-curve value (`y`).
