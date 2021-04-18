use std::f64::consts::PI;

trait CalculateArea {
    fn calculate(&self);
}

struct Triangle {
    bottom: f32,
    high: f32,
}

struct Rectangle {
    length: u32,
    width: u32,
}

struct Circle {
    radius: f64,
}

impl CalculateArea for Triangle {
    fn calculate(&self) {
        let area = self.bottom*self.high/2.0;
        println!("Area is {}.",area);
    }
}

impl CalculateArea for Rectangle {
    fn calculate(&self) {
        let area = self.length*self.width;
        println!("Area is {}.",area);
    }
}

impl CalculateArea for Circle {
    fn calculate(&self) {
        let area = self.radius*self.radius*2.0*PI;
        println!("Area is {}.",area);
    }
}

fn main() {
    let _triangle_a = Triangle {
        bottom: 10.0,
        high: 5.0,
    };
    let _rectangle_a = Rectangle {
        length: 10,
        width: 10,
    };
    let _circle_a = Circle {
        radius: 10.0,
    };

    _triangle_a.calculate();
    _rectangle_a.calculate();
    _circle_a.calculate();
}
