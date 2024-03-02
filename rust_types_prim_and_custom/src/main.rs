fn main() {
    println!("Hello, world!");
    prim_types();
    custom_types();
}

fn prim_types() {
    let _i: i32 = 45;  // integer
    let _f: f32 = 72.5; // float
    let _u: u64 = 733244; // usigned 
    let _c: char = 'c'; // char
    let _b: bool = true; // boolean
}

#[derive(Debug)]
struct Person {
    name: String,
    suid: i32,
}

#[derive(Debug)]  // can use println!() to print custom structs
struct Univ {
    student: Person,
    address: String,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn compare_borrowing(&self, other: &Rectangle) -> bool {
        self.area() > other.area()     
    }

    fn compare_owning(&self, other: Rectangle) -> bool {
        self.area() > other.area()     
    }
}

fn custom_types() {
    let name: String = String::from("Alex");
    let address: String = String::from("London");
    let student = Person{name: name, suid: 1234};
    let university = Univ{address: address, student: student};

    println!("{:?}", university);
    println!("Name: {} Id:{} Addr:{}", university.student.name, university.student.suid, university.address);

    let rect = Rectangle{width: 30, height: 50};
    println!("Area: {}", rect.area());

    let new_rect = Rectangle {width: 10, height: 20};
    println!{"{} {}", rect.compare_borrowing(&new_rect), rect.compare_owning(new_rect)};

    let mut r1 : Rectangle = Rectangle {height: 10, width: 25};
    r1.height = 45;
    println!("New Area: {} Vol: {}", r1.area(), rect_volume(100, &r1));
}

// passing struct as being borrowed
fn rect_volume(breadth: u32, r: &Rectangle) -> u32 {
    r.area()*breadth
}