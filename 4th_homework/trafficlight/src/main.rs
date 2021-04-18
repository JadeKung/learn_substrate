trait LightTime {
    fn get_light_time(&self) -> u8;
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl LightTime for TrafficLight {
    fn get_light_time(&self) -> u8 {
        let light = &self;
        match light {
            TrafficLight::Red => 1,
            TrafficLight::Green => 2,
            TrafficLight::Yellow => 3,
        }
    }
}

fn main() {
    let light_red = TrafficLight::Red;
    let time_red = light_red.get_light_time();
    let light_green = TrafficLight::Green;
    let time_green = light_green.get_light_time();
    let light_yellow = TrafficLight::Yellow;
    let time_yellow = light_yellow.get_light_time();

    println!("TrafficLight is red,Time is {}",time_red);
    println!("TrafficLight is green,Time is {}",time_green);
    println!("TrafficLight is yellow,Time is {}",time_yellow);
}
