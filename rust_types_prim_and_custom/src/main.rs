#[allow(unused_assignments)]
fn main() {
    println!("Hello, world!");
    prim_types();
    custom_types();
    rust_enum_types();
    rust_enum_option_types();    
}

fn rust_enum_option_types() {
    let mut i : Option<i32> = Some(-12342);
    match i {
        Some(i) =>  println!("The option is : {}", i),
        None => println!("The option is a none"),
    }
    i = Some(5);
    if let Some(5) = i {
        println!("that is a match for the option and the value is {:?}", i);
    }
}

fn rust_enum_types() {
    // let color = Color::RED;  // compiles, but on runtime, we get a panic stating "not yet implemented! and displays the message in todo!()"
    // let color = 0; // so C/C++ enum value (starting from 0) does not hold true in RUST :'
    // enum_types(color);  // after the panic, the further code is not run! 
    // note: the enum color is defined later in the code, but can used earlier.
    let mut new_color = Color::BLUE; 
    enum_types(&new_color);
    if new_color.green_base() {
        println!("Color is a part of green!");
    }
    else {
        println!("Color is a not part of green!"); 
    }
    new_color = Color::GREEN;
    println!("Color {:?} is green itself: {}", new_color, new_color.is_green());
    
    new_color = Color::GOLD(String::from("Gold is a not a color, it is metal!")); 
    new_color.is_gold();
}


#[allow(dead_code)] // to suppress dead code warning
#[derive(Debug)]
enum Color {
    YELLOW,
    GREEN,
    BLUE,
    RED,
    GOLD(String), 
    MAGENTA,
    LILA
}

// define functions for enums 
impl Color {
    fn green_base (&self) -> bool {
        match self {
            Color::YELLOW => true,
            Color::BLUE => true,
            Color::GREEN => true,    
            _ => false,
        }
    }

    fn is_green(&self) -> bool {
        if let Color::GREEN = self {
            return true;
        }
        false
    }

    fn is_gold(&self) {
        if let Color::GOLD(s) = self {
            println!("{}", s);
        }    
    }
}


fn enum_types(color: &Color)  {
    match color {
        Color::YELLOW => {
            println!("Yellow");
        }
        Color::GREEN => {
            println!("Green");
        }
        Color::BLUE => {
            println!("Blue");
        }
        Color::RED => todo!("yet to define!"), // todo items, for future development. Cool!

        // rest color are not required 
        _ => println!(".. the rest are not valued!!!"),  //  _  is equivalent to default case in  C/C++
    };
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