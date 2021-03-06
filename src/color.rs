use crate::vec3;
use std::io;

pub type Color = vec3::Vec3;

// impl fmt::Display for Color {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let ir: i32 = (255.999 * self.x) as i32;
//         let ig: i32 = (255.999 * self.y) as i32;
//         let ib: i32 = (255.999 * self.z) as i32;
//         write!(f, "{} {} {}", ir, ig, ib)
//     }
// }

pub fn write_color<T>(stream: &mut T, pixel_color: &Color, samples_per_pixel: i32) -> io::Result<()>
where
    T: io::Write,
{
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f64;

    let r = (scale * r).sqrt();
    let g = (scale * g).sqrt();
    let b = (scale * b).sqrt();

    let ir: i32 = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let ig: i32 = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let ib: i32 = (256.0 * b.clamp(0.0, 0.999)) as i32;

    write!(stream, "{} {} {}\n", ir, ig, ib)
}
