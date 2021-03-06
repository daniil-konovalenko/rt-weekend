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

pub fn write_color<T>(stream: &mut T, color: &Color) -> io::Result<()>
where
    T: io::Write,
{
    let ir: i32 = (255.999 * color.x) as i32;
    let ig: i32 = (255.999 * color.y) as i32;
    let ib: i32 = (255.999 * color.z) as i32;

    write!(stream, "{} {} {}\n", ir, ig, ib)
}
