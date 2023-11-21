
//https://raytracing.github.io/books/RayTracingInOneWeekend.html

use linya::{Bar, Progress};
use std::io::Write;
use std::fs::OpenOptions;



#[derive(Debug,Clone)]
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
//multiply
impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let x = self.x() * other.x();
        let y = self.y() * other.y();
        let z = self.z() * other.z();

        Self {
             e: [x,y,z],
        }
    }
}

//divide
impl std::ops::Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let x = self.x() / other.x();
        let y = self.y() / other.y();
        let z = self.z() / other.z();

        Self {
             e: [x,y,z],
        }
    }
}

//utlity functions 



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
            let ir = (255.99 * r ) as i32;
            let ig = (255.99 * g ) as i32;
            let ib = (255.99 * b ) as i32;
            f.write_all(format!("{} {} {} \n",ir,ig,ib).as_bytes()).expect("Unable to write data");
            //fs::write(file_name,format!("{} {} {}", ir,ig,ib))?;
        }    
    }  
    //Ok(())
}
