// Diamond
// Neil Crago <n.j.crago@gmail.com>
// 02/05/24
//
// Quick code to generate
// 1.) a S-curve
// 2.) a front-loaded and
// 3.) a back-loaded s-curve
// 4.) reveals the `california envelope` when the resulting CSV is plotted
//

use std::fs::OpenOptions;
use std::io::Write;
mod scurve;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Write data to the output file, "diamond.csv"
    let out_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("diamond.csv");
    let errheader = writeln!(
        out_file.as_ref().expect("REASON"),
        "time,S-Curve,Front Loaded,Back Loaded"
    );
    if errheader.is_err() {
        panic!("Error writing to file: {errheader:?}");
    }

    // logistic function s-curve
    let pv2d = scurve::scurves::do_s_curve(&100);

    let mut i = 0;
    for oo in pv2d {
        i += 1;
        let errval = writeln!(
            out_file.as_ref().expect("REASON"),
            "{},{},{},{}",
            i,
            oo.0, // S-Curve
            oo.1, // Front Loaded
            oo.2  // Back Loaded
        );

        if errval.is_err() {
            panic!("Error writing to file: {errval:?}");
        }
    }
    Ok(())
}
