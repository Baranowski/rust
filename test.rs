#![allow(dead_code)]
/**
 * 16 Methods
 * http://doc.rust-lang.org/tutorial.html#methods
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

static mut DUMP_FILE: Option<File> = None;
struct StackDebug {
    s: &'static str,
}

impl StackDebug {
    pub fn new(s: &'static str) -> StackDebug {
        unsafe{
            if let None = DUMP_FILE {
                DUMP_FILE = Some(File::create("dump_file.txt").unwrap());
            }
            write!(DUMP_FILE.as_mut().unwrap(), "+{}\n", &s).unwrap();
        }
        StackDebug{s}
    }
}

impl Drop for StackDebug {
    fn drop(&mut self) {
        unsafe{
            write!(DUMP_FILE.as_mut().unwrap(), "-{}\n", &self.s).unwrap();
        }
    }
}

#[derive(Debug)]
struct Point {
	x: f64,
	y: f64
}
#[derive(Debug)]
enum Shape {
	Circle { center: Point, radius: f64 },
	Rectangle { top_left: Point, bottom_right: Point }
}
impl Shape {
	fn draw(&self) {
            let _guard = StackDebug::new("Shape::draw");
		match *self {
			Shape::Circle{center: ref p, radius: ref f} => draw_circle(p, f),
			Shape::Rectangle{top_left: ref p1, bottom_right: ref p2} => draw_rectangle(p1, p2)
		}
	}
	pub fn new_circle(area: f64) -> Shape {
            let _guard = StackDebug::new("Shape::new_circle");
		let center = Point{x: 0.0, y: 0.0};
		let radius = (area / PI).sqrt();
		Shape::Circle{center: center, radius: radius}
	}
}
fn draw_circle(p: &Point, f: &f64) {
            let _guard = StackDebug::new("draw_circle");
	println!("draw_circle({:?}, {:?})", p, f);
}
fn draw_rectangle(p1: &Point, p2: &Point) {
            let _guard = StackDebug::new("draw_rectangle");
	println!("draw_rectangle({:?}, {:?})", p1, p2);
}
fn main() {
            let _guard = StackDebug::new("main");
	let c = Shape::Circle{center: Point { x: 1.0, y: 2.0 }, radius: 3.0};
	c.draw();

	let r = Shape::Rectangle{top_left: Point{x: 1.0, y: 2.0}, bottom_right: Point{x: 2.0, y: 3.0}};
	r.draw();

	let c2 = Shape::new_circle(42.5);
	println!("c2={:?}", c2);
	c2.draw();
}

