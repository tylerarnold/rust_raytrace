
//https://raytracing.github.io/books/RayTracingInOneWeekend.html

use linya::{Bar, Progress};
use std::io::Write;
use std::fs::OpenOptions;
 use std::fs::File;


#[derive(Debug,Clone,Copy)]
pub struct Vec3 {
    e: [f64; 3], 
}

impl Default for Vec3 {
    fn default()-> Vec3 {
        Vec3 {
            e: [0.0; 3],
        }

    }
}
impl Vec3 {
   
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            e: [x,y,z],
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    fn length_squared(&self) -> f64 {
        let ls = self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2];
        ls 
    }

    pub fn length(&self) -> f64 {
        let ls = self.length_squared();
        ls.sqrt()
    }

} 
//negate 

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let x = -self.x();
        let y = -self.y();
        let z = -self.z();

        Self {
             e: [x,y,z],
        }
    }
}

//Operand overide 

// add two Vec3
impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        let z = self.z() + other.z();

        Self {
             e: [x,y,z],
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        let z = self.z() - other.z();

        Self {
             e: [x,y,z],
        }
    }
}

//multiply
impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x() * rhs.x();
        let y = self.y() * rhs.y();
        let z = self.z() * rhs.z();

        Self {
             e: [x,y,z],
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        let x = self.x() * t;
        let y = self.y() * t;
        let z = self.z() * t;

        Self {
             e: [x,y,z],
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let x = rhs.x() * self;
        let y = rhs.y() * self;
        let z = rhs.z() * self;

        Vec3 {
             e: [x,y,z],
        }
    }
}


impl std::ops::MulAssign<f64> for Vec3 {
    
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] =  self.e[0] * rhs;
        self.e[1] =  self.e[1] * rhs;
        self.e[2] =  self.e[2] * rhs;
    }
}

//divide
impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let d = 1.0 / rhs;
        let x = self.x() * d;
        let y = self.y() * d;
        let z = self.z() * d;

        Self {
             e: [x,y,z],
        }
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    
    fn div_assign(&mut self, rhs: f64) {
        let d = 1.0 / rhs;
        self.e[0] =  self.e[0] * d;
        self.e[1] =  self.e[1] * d;
        self.e[2] =  self.e[2] * d;
    }
}


//functions 
pub fn  dot(u:&Vec3, v:&Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn  cross(u:&Vec3, v:&Vec3) -> Vec3 {
    Vec3{
        e: [(u.e[1] * v.e[2] - u.e[2] * v.e[1]),
            (u.e[2] * v.e[0] - u.e[0] * v.e[2]),
            (u.e[0] * v.e[1] - u.e[1] * v.e[0]),]
        }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    let len = v.length();
    *v / len 
}

// 3.1 Color Utility Functions
type Color = Vec3;

pub fn write_color(file: &mut File, c: Color) {
    let ir = (255.99 * c.x() ) as i32;
    let ig = (255.99 * c.y() ) as i32;
    let ib = (255.99 * c.z() ) as i32;
    file.write_all(format!("{} {} {} \n",ir,ig,ib).as_bytes()).expect("Unable to write data");
}


#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(1.0,2.0,3.0);
        let v2 = Vec3::new(2.0,4.0,8.0);
        let v3 = v1 * v2;
        assert_eq!(round_two_digits(v3.x()), 2.0);
        assert_eq!(round_two_digits(v3.y()), 8.0);
        assert_eq!(round_two_digits(v3.z()), 24.0);
    }

    #[test]
    fn test_mul_f64() {
        let v1 = Vec3::new(1.0,2.0,3.0);
        let v2 = v1 * 2.0;
        assert_eq!(round_two_digits(v2.x()), 2.0);
        assert_eq!(round_two_digits(v2.y()), 4.0);
        assert_eq!(round_two_digits(v2.z()), 6.0);
        let v1 = Vec3::new(1.0,2.0,3.0);
        let v2 = 2.0 * v1;
        assert_eq!(round_two_digits(v2.x()), 2.0);
        assert_eq!(round_two_digits(v2.y()), 4.0);
        assert_eq!(round_two_digits(v2.z()), 6.0);

    }


    #[test]
    fn test_mul_assign() {
        let mut v1 = Vec3::new(1.0,2.0,3.0);
        v1 *= 2.0;
        assert_eq!(round_two_digits(v1.x()), 2.0);
        assert_eq!(round_two_digits(v1.y()), 4.0);
        assert_eq!(round_two_digits(v1.z()), 6.0);

    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new(4.0,8.0,12.0);
        let d = v1 / 2.0;
        assert_eq!(round_two_digits(d.x()), 2.00);
        assert_eq!(round_two_digits(d.y()), 4.00);
        assert_eq!(round_two_digits(d.z()), 6.00);
    }

    #[test]
    fn test_length() {
        let v1 = Vec3::new(2.0,3.0,4.0);
        assert_eq!(round_two_digits(v1.length()), 5.39);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0,1.0,1.0);
        let v2 = Vec3::new(1.0,2.0,3.0);
        assert_eq!(round_two_digits(dot(&v1,&v2)), 6.00);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0,5.0,2.0);
        let v2 = Vec3::new(2.0,10.0,4.0);
        let c = cross(&v1,&v2);
        assert_eq!(round_two_digits(c.x()), 0.00);
        assert_eq!(round_two_digits(c.y()), 0.00);
        assert_eq!(round_two_digits(c.z()), 0.00);
    }

    #[test]
    fn test_unit() {
        let v1 = Vec3::new(1.0,1.0,1.0);
        assert_eq!(round_two_digits(v1.length()), 1.73);
        let u = unit_vector(&v1);   
        assert_eq!(round_two_digits(u.x()), 0.58);
        assert_eq!(round_two_digits(u.y()), 0.58);
        assert_eq!(round_two_digits(u.z()), 0.58);

    }
}


fn main() {
    let image_width = 256;
    let image_height = 256;
    let file_name = "./image.ppm";

    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true) // Optionally create the file if it doesn't already exist
        .open(file_name)
        .expect("Unable to open file");
    

    f.write_all("P3\n".as_bytes()).expect("Unable to write data");
    f.write_all(format!("{} {}\n", image_width,image_height).as_bytes()).expect("Unable to write data");
    f.write_all("255\n".as_bytes()).expect("Unable to write data");
    
    let mut progress = Progress::new();

    let bar: Bar = progress.bar(50, "Rendering");

    for j in 0..image_height {
        progress.set_and_draw(&bar, j);
        for i in 0..image_width {
            

            let r = (i as f64) / ((image_width -1) as f64);
            let g = (j as f64) / ((image_height -1) as f64);
            let b = 0.0;
            let pixel_color = Color::new(r,g,b);
            write_color(&mut f,pixel_color);
        }    
    }  
    //Ok(())
}
