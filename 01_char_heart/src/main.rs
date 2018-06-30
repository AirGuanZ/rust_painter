const IMG_W : i32 = 60;
const IMG_H : i32 = 31;

const CHARS : [char; 8] = ['.', ':', ':', '-', '+', '=', '*', '#'];

fn heart_equ(x: f64, y: f64, z: f64) -> f64 {
	let cubic = |s| { s * s * s };
	let cy = cubic(y);
	cubic(x * x + 9.0 / 4.0 * z * z + y * y - 1.0)
		- x * x * cy - 9.0 / 80.0 * z * z * cy
}

fn solve_z(func: fn(f64, f64, f64)->f64, x: f64, y: f64) -> f64 {
	let mut lbound = 0.001;
	let pos_start = func(x, y, 0.0) > 0.0;

	while lbound < 5.0 && (func(x, y, lbound) > 0.0) == pos_start {
		lbound += 0.001;
	}
	
	if lbound >= 5.0 { std::f64::INFINITY } else { lbound }
}

fn main() {
	for y in 0..IMG_H {
		for x in 0..IMG_W {
			let sx = x as f64 / (IMG_W as f64 - 1.0) * 3.0 - 1.5;
			let sy = - (y as f64 / (IMG_H as f64 - 1.0)) * 3.0 + 1.5;
			let s = solve_z(heart_equ, sx, sy);

			if s.is_infinite() {
				print!(" ");
			} else {
				let c = (-0.557 * sx + 0.743 * sy + 0.335 * s) /
						(sx * sx + sy * sy + s * s).sqrt() * 1.025;
				let c = (c.min(1.0).max(0.0) * (CHARS.len() - 1) as f64) as usize;
				print!("{}", CHARS[c]);
			}
		}
		println!("");
	}
}
