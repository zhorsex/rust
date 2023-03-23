//5题：为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同

// 定义交通信号灯枚举
enum TrafficLight {
    Red, // 红灯
    Yellow, // 黄灯
    Green, // 绿灯
}

// 定义时间持续 trait
trait TimeDuration {
    fn duration(&self) -> u8; // 返回时间的方法
}

// 实现时间持续 trait
impl TimeDuration for TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Red => 60, // 红灯持续60秒
            TrafficLight::Yellow => 10, // 黄灯持续10秒
            TrafficLight::Green => 45, // 绿灯持续45秒
        }
    }
}

//6题：实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
// 计算u32类型的数组中所有元素的和
fn sum_u32(nums: &[u32]) -> Option<u32> {
    // 使用try_fold方法，将数组中的每个元素累加到acc中
    nums.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}
  
  
//7题：实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束
// 定义一个可以计算面积的trait
trait Area {
    fn area(&self) -> f64;
}

// 定义一个圆形结构体
struct Circle {
    radius: f64,
}

// 实现圆形结构体的面积计算方法
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 定义一个三角形结构体
struct Triangle {
    base: f64,
    height: f64,
}

// 实现三角形结构体的面积计算方法
impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 定义一个正方形结构体
struct Square {
    side: f64,
}

// 实现正方形结构体的面积计算方法
impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 定义一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数
fn print_area<T: Area>(shape: T) {
    println!("该图形的面积为: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 2.0 };
    let triangle = Triangle { base: 3.0, height: 4.0 };
    let square = Square { side: 2.5 };

    // 调用打印图形面积的函数，分别传入圆形、三角形、正方形
    print_area(circle);
    print_area(triangle);
    print_area(square);
}

