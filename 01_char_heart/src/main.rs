fn heart_equ(x: f64, y: f64, z: f64) -> f64 {
    let cubic = |s| { s * s * s };
    cubic(x * x + 9.0 / 4.0 * z * z + y * y - 1.0)
        - x * x * cubic(y) - 9.0 / 80.0 * z * z * cubic(y)
}

fn solve_z(func: fn(f64, f64, f64)->f64, x: f64, y: f64) -> (bool, f64) {
    let mut lbound = 0.001;
    let pos_start = func(x, y, 0.0) > 0.0;

    while lbound < 5.0 && (func(x, y, lbound) > 0.0) == pos_start {
        lbound += 0.001;
    }
    
    if lbound >= 5.0 { (false, 0.0) } else { (true, lbound) }
}

fn shade_pixel(sx: f64, sy: f64) -> char {
    match solve_z(heart_equ, sx, sy) {
        (true, sz) => {
            const CHARS : [char; 8] = ['.', ':', ':', '-', '+', '=', '*', '#'];
            let c = (-0.557 * sx + 0.743 * sy + 0.335 * sz) / (sx * sx + sy * sy + sz * sz).sqrt() * 1.025;
            CHARS[(c.min(1.0).max(0.0) * (CHARS.len() - 1) as f64) as usize]
        },
        _ => ' '
    }
}

fn main() {
    const IMG_W: f64 = 3.0;
    const IMG_H: f64 = 3.0;
    const SCR_W: i32 = 60;
    const SCR_H: i32 = 30;

    for y in 0..SCR_H {
        for x in 0..SCR_W {
            let sx =   x as f64 / (SCR_W as f64 - 1.0) * IMG_W - (IMG_W / 2.0);
            let sy = - y as f64 / (SCR_H as f64 - 1.0) * IMG_H + (IMG_H / 2.0);
            print!("{}", shade_pixel(sx, sy));
        }
        println!("");
    }
}
