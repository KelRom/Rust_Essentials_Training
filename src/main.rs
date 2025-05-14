// how to input modules
use std::{fs, io::Write, mem, env, fmt, any};
use rand::prelude::*; // brings in the most commonly used methods/function for this crate. Asteriks is a wild card meaning to bring in all paths mathching the prefix
// use rand::random // this will use just this specific function that we need from the library

fn main() {
    let a: f32 = 10.0;
    let b: f32 = 3.0;
    let c: f32  = a / b;
    
    // println formating---------------------------------------------------------------------------------------------------------------------------------------------
    println!("c is {:.3}", c); // tells the println macro to print three decimal places the colon specifies we want to use special formatting
    println!("c is {:8.3}", c); // tells the println macro to print using 8 characters total and three decimal places
    println!("c is {:08.3}", c); // tells the println macro to print 0s as padding using 8 characters total and three decimal places
    println!("c is {0:08.3}\na is {1}\nonce again, c is {0}\n", c, a); // first number/or number before the colon is the positions of the variables to use.

    //Bitwise Operations---------------------------------------------------------------------------------------------------------------------------------------------
    let mut value: u8 = 0b1111_0101u8;
    println!("value is {value}"); // prints the binary value as a normal number: 245
    println!("value is {:08b}", value); // formats to print in binary, 0 shows any leading zero, 8 is to show 8 bits or values, lowercase b says to display in binary

    value = !value; // This will invert the bits
    println!("value is {:08b}", value);

    value = value & 0b1111_0111; // Using bitwise AND to clear the position at bit 3. Will print 00000010
    println!("value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000); // Check bit 6
    println!("bit 1 is {}", value & 0b0000_0010); // Check bit 1, this is basically checking to see it the bit is on or not. 

    value = value | 0b0100_0000; // Turns on the 6 bit
    println!("value is {:08b}", value);

    value = value ^ 0b0101_0101; //XOR
    println!("value is {:08b}", value);

    value = value << 4; // Left shift value by 4 bits, value comes first then what you want to shift it by
    println!("value is {:08b}", value);

    value = value >> 2; 
    println!("value is {:08b}", value); // Right shift value by 2 bits, value comes first then what you want to shift it by

    // Boolean data types--------------------------------------------------------------------------------------------------------------
    let a: bool = true;
    let b: bool = false;

    println!("a is {a} and b is {b}");
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^ b);

    let c: bool = (a ^ b) || (a & b); // short circuiting logical operator (very cool)
    println!("c is {c}");
    

    let letter: char = 'a';
    let number: char = '1';
    let finger: char = '\u{261D}'; // This is how you would type in a unicode value
    println!("{letter}\n{number}\n{finger}");

    // Compound Data types ----------------------------------------------------------------------------------------------------------------
    // Array
    let mut letters: [char; 3] = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter: char = letters[0];
    println!("first_letter is {first_letter}");

    let numbers: [i32; 5];  // how to make an array first you put the datatype and then how many number
    numbers = [0; 5]; // This is a repeat expression
    println!("last number is {}", numbers[4]);

    // Multidimensional Array
    let parking_lot: [[i32; 3]; 2] = [[1, 2, 3], 
                                      [4, 5, 6]]; // to specific the array list the inner array first, with datatypes and how much it can hold and after specific in the outer array how many arrays you will hold.
    
    let number: i32 = parking_lot[0][1]; // This is how you access a multidimensional array.
    println!("number is {}", number);

    // Tuples (can mix different datatypes)
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x'); // this is how to initialize a type
    stuff.0 += 3;
    let first_item: u8 = stuff.0; // this is how you would access a specific element. Just change the number after the . to the element you are tring to access. 
    println!("first_item is {first_item}");

    let (_a, b, _c): (u8, f32, char) = stuff;
    println!("b is {b}");

    // Using Functions-------------------------------------------------------------------------------------------------------------------------
    say_hello();

    let x: u8 = 1;
    let y: u8 = 2;
    say_the_sum(x, y);

    println!("result is {:?}", square(13)); // colon and question mark is special debugging formating

    let make_x_odd: bool  = true;
    let _x: u8 = if make_x_odd {1} else {2}; // conditional assignment 

    // Loops-------------------------------------------------------------------------------------------------------------------------------------
    let mut count: i32 = 0;
    let result: i32 = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {count}"); 
    }; // this works because loops are expressions and you can return an expression. Since we are assign count to result this becomes a statement which needs a semicolon.

    println!("After the loop!");
    println!("result is {result}");

    // While loops do not have the ability to return a value!!!!!!!!
    // For Loops to loop through a collection
    let message: [char; 5] = ['h', 'e', 'l', 'l', 'o'];
    for (index, item) in message.iter().enumerate() {
        println!("item {index} is {item}");
    } // Enumerate method will give me the index of the loop. So need to deconstruct a tuple that has the value of the index and the value.

    // loops for a range
    for number in 0..5 {
        println!("number is {number}");
    } // 5 is not inclusive, to make inclusive add and equal sign right after the two ..

    // Strings -------------------------------------------------------------------------------------------------------------------------------------
    let _word: &str= "Hello";  // This is a string literal, immutable, has to known at complime time
    let mut message: String = String::from("Earth"); // This is a string type, can be mutable, dynamic in size, stored on the heap
    println!("message is {message}");
    message.push_str(" is home.");
    println!("message is {message}");

    // Ownership, scopes, moving, cloning and copying data-------------------------------------------------------------------------------------------
    /*let outer_planet: String;
    {
        // Rust does not automatically copy data. It just moves its from one variable to another. To make a deep copy use clone.
        let mut inner_planet: String = String::from("Mercury");
        //outer_planet = inner_planet; // Moves data to other variables
        outer_planet = inner_planet.clone(); // Makes a deep copy
        inner_planet.clear(); // since the data is cloned outer planet will not be changed
        println!("inner_planet is {inner_planet}"); // This will not complie if data is moved, if copied it complies, this is to show how ownership and borrowship works, the value was transferred to outer planet, invalidating/deleting inner planet

    }*/
    // ints work differently then strings no need to use the clone method and we could just assign the variable
    // works different no issue with borrower or owner since data lives on the stack
    // this basically copies everything
    // copy is done automically since it is on the stack and the size is known 
    let outer_planet: i32;

        let mut inner_planet: i32 = 1;
        outer_planet = inner_planet; // Moves data to other variables
        inner_planet += 1; // since the data is cloned outer planet will not be changed
        println!("inner_planet is {inner_planet}"); // This will not complie if data is moved, if copied it complies, this is to show how ownership and borrowship works, the value was transferred to outer planet, invalidating/deleting inner planet

    println!("outer_planet is {outer_planet}");

    // References -----------------------------------------------------------------------------------------------------------------------------------
    // when using a mutable reference you cannot create other references
    // this helps prevent data races
    let mut rocket_fuel: String = String::from("RP-1 ");
    let length: usize = process_fuel(&mut rocket_fuel);
    println!("rocket_fuel is {rocket_fuel} and length is {length}");

    let rocket_fuel: String = produce_fuel();
    println!("rocket_fuel is {rocket_fuel}");

    // Slice ----------------------------------------------------------------------------------------------------------------------------------------
    /*
        references to a contiguous section of a collection
        string literal are slices
     */

    let message: String = String::from("Greetings from Earth!");
    println!("message is {message}");

    let last_word: &str = &message[15..15 + 5];
    println!("last_word is {last_word}");

    // slices for array
    let planets: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_planets are {:?}", inner_planets);

    // Using functions that returns slices
    // Strings can be used as string slices since it has all the information needed to work but you can not pass a string slice into a String parameter
    let message: String = String::from("Greetings from Earth!");
    let first_word: &str = get_first_word(&message);
    println!("first_word is {first_word}");

    // Standard input
    /*
    No error handling code yet. Will be covered later

    let mut buffer = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {buffer}");

    let number: i32 = buffer.trim().parse().unwrap(); This will take trailing and starting white spaces away, and converts/parses it into an it.
    println!("number + 1 is {}", number + 1);

     */

    // Using Rand and importing/using crates ---------------------------------------------------------------------------------------------------------
   let number: f64 = rand::random::<f64>();
   println!("number is  {number}");

   let number: i32 = rand::rng().random_range(1..11); // returns a random number between 1 inclusive and 11 exclusive, so basically 1-10
   println!("number is {number}");


   // Command Line Argument ------------------------------------------------------------------------------------------------------------------------
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments.");
        return;
    }
    
    for(index, argument) in env::args().enumerate() {
        println!("argument {index} is {argument}");
    }
    let arg2: String = env::args().nth(2).unwrap();
    println!("arg2 is {arg2}");

    // Reading from files -------------------------------------------------------------------------------------------------------------------
    let contents: String = fs::read_to_string("planets.txt").unwrap();
    println!("contents is {contents}");

    for line in contents.lines() {
        println!("line is {line}");
    }

    // Reads file information and stores it in bytes
    let contents: Vec<u8>= fs::read("planets.txt").unwrap();
    println!("contents is {:?}", contents);

    // Writing to files
    let mut speech: String =  String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech).unwrap();

    // Commented so pluto is not constantly added to file
    //let mut file: fs::File  = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
    //file.write(b"\nPluto"); // This allows the text to be intrepreted as a collection of byte values

    // Struct ------------------------------------------------------------------------------------------------
    let mut vehicle: Shuttle = Shuttle {
        name: String::from("Endeavor"), 
        crew_size: 7,
        propellant: 835958.0
    };

    // the .. will initilize the other values based off first vehicle, called the struct update syntax
    let vehicle2: Shuttle = Shuttle {
        ..vehicle.clone() // used because all values are initilized from the first vehicle, which contains a string, which by default moves so it will change ownership unlike the other fields in the struct
    };
    
    vehicle.crew_size = 6;
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);

    let mut vehicle: Shuttle = Shuttle::new("Endeavour"); // Using the associated function
    let vehicle_name: &str = vehicle.get_name();
    println!("vehicle_name is {vehicle_name}");
    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);

    // tuple struct ------------------------------------------------------------------------------------------------------------------------
    let red: Color = Color(255, 0, 0);
    println!("First value is {}", red.0);
    
    let coord: Point = Point(4, 5, 6);
    let y: u8 = get_y(coord);
    println!("y is {y}");

    // Generic Types-------------------------------------------------------------------------------------------------------------------------
    let rect: Rectangle<u8, u8> = Rectangle {
        width: 1u8, 
        height: 3u8
    };

    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_width());

    println!("biggest is {}", get_biggest(1.2, 2.3));

    // Box<T> data type, allows to store data on the heap instead of on the stack, are considered a smart pointer
    // has ownership of the data it points to 
    // when goes out of scope it deallocates the heap memory
       let vehicle: Shuttle = Shuttle {
        name: String::from("Atlantis"), 
        crew_size: 7,
        propellant: 835958.0
    };

    println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));
    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle); // this will take ownership of the vehicle and make the vehicle no longer valid
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle));
    println!("boxed_vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle)); // deference operator just like c++ nothing new here

    let unboxed_vehicle: Shuttle = *boxed_vehicle; // pass data back to stack and pass ownership to this unboxed_vehicle
    println!("unboxed_vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));

    // Traits -------------------------------------------------------------------------------------------------------------------------------
    let hubble: Satellite = Satellite {
        name: String::from("Hubble Telescope"), 
        velocity: 4.72 
        };

    let iss: SpaceStation = SpaceStation { 
        name: String::from("International Space Station"), 
        crew_size: 6, 
        altitude: 254 
    };

    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());

    let hubble: Satellite = Satellite {
        name: String::from("Hubble Telescope"), 
        velocity: 4.72 
    };

    let gps: Satellite = Satellite {
        name: String::from("GPS"), 
        velocity: 2.42 
    };

    println!("hubble == gps is {}", hubble == gps);
    println!("hubble == gps is {}", hubble > gps);

    // Trait bounds -------------------------------------------------------------------------------------------------------------------------
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([13]);

    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);

    println!("output is {}", get_displayable());

    // Lifetimes ----------------------------------------------------------------------------------------------------------------------------
    // lifetime convention is using 'single lowercase letter
     let result: &str;
     let propellant1: String = String::from("RP-1");
     {
        let propellant2: String = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);

     }
    println!("result is {result}");

    let vehicle: Shuttle2 = Shuttle2 { 
        name: "Endeavour" 
    };

    let sender: &str = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {sender}");
    
}







// Creating Functions --------------------------------------------------------------------------------------------------------------------------
fn say_hello() -> () {
    println!("Hello!");
    say_number(13);
}

fn say_number(number: i32) -> () {
    println!("number is {number}");
}

fn say_the_sum(a:u8, b: u8) -> () {
    let sum: u8 = a + b; 
    println!("sum is {sum}");
}

// Functions with return values
fn square(x: i32) -> (i32, i32) {
    println!("squaring {x}");
    (x, x * x) // dont need to add a semicolon since it is an expression and Rust will return this value
}

// Function with ownership and borrowing
fn process_fuel(propellant: &mut String) -> usize {
    println!("processing propellant {propellant}... ");
    propellant.push_str(" is highly flammable!");
    let length: usize = propellant.len();
    length
}

fn produce_fuel() -> String {
    let new_fuel: String = String::from("RP-1");
    new_fuel
}

// Slices as functions parameters --------------------------------------------------------------------------------------------------
fn get_first_word(s: &str) -> &str {
    let bytes: &[u8]  = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space
        }
    }

    &s
}

// Structs --------------------------------------------------------------------------------------------------------------------------
#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

// this will allow us to give functionality to a type, used to create methods and associated functions
impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) -> () {
     self.propellant += gallons;   
    }

    // associated function
    fn new(name: &str) -> Shuttle {
        Shuttle { 
            name: String::from(name),
            crew_size: 7, 
            propellant: 0.0 
        }
    }
}

// Tuple Struct -------------------------------------------------------------------------------------------------------
struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

// Generic Types ------------------------------------------------------------------------------------------------------
#[derive(Debug)]
struct Rectangle <T, U> {
    width: T,
    height: U,
}

// Generic Functions for structs
impl <T, U> Rectangle <T, U> {

    fn get_width(&self) -> &T {
        &self.width
    } 
}

// Generic Functions for a specific type
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

// Generic Functions ------------------------------------------------------------------------------------------------------
// ParitalOrd is a trait that represents values that can be compared
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// Traits -------------------------------------------------------------------------------------------------------------------
// a collection of methods, data types can implement a traits
// Generics use traits to specify the capabilities of unknown data types
// Similar to interfaces in c++ and java
#[derive(PartialEq, PartialOrd)] // This means to derive from a certain trait, can derive more traits by separating by commas
struct Satellite {
    name: String,
    velocity: f64
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32
}

trait Description {
    fn describe(&self) -> String {
        String::from("an object flying through space") // This is a default implementation of the trait
    }    
}

impl Description for Satellite {
    /*fn describe(&self) -> String {
        format!("The {} flying at {} miles per second!", self.name, self.velocity)
    }*/ // Only done just to test the default implementation of the Description trait
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("The {} flying {} miles high with {} crew members", self.name, self.altitude, self.crew_size)
    }
}

// Trait Bounds -------------------------------------------------------------------------------------------------------------------------
fn print_type<T: fmt::Debug>(item: T) {
    println!("{:?} is {}", item, any::type_name::<T>());
}
// implementing multiple trait bounds ---------------------------------------------------------------------------------------------------

//fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
// How to use a where clause, used to make code in parameters look nicer instead of how it looks about
fn compare_and_print<T, U>(a: T, b: U) 
    where T: fmt::Display + PartialEq + From<U>, 
          U: fmt::Display + PartialEq + Copy 
          {
    if a == T::from(b) {
        println!("{a} is equal to {b}");
    } else {
        println!("{a} is NOT equal to {b}");
    }
}

// Return types with implemented traits --------------------------------------------------------------------------------------------------
fn get_displayable() -> impl fmt::Display {
    13
}

 // Lifetimes ----------------------------------------------------------------------------------------------------------------------------
 // tells the compiler how the lifetimes of the input parameters relate to each other
 /*
    Ellision rules
    1. Each input parameter that is a REFERENCE is assigned its own lifetime
    2. If there is exactly one input lifetime, assign it to all output lifetimes
    3. If there is &self or &mut self input parameter, its lifetime will be assigned to all output lifetimes.
  */
 fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
 }

 struct Shuttle2<'a> {
    name: &'a str
}

impl <'a> Shuttle2 <'a>  {
    fn send_transmission(&self, msg: &str) -> &str {
        println!("transmitting message: {msg}");
        self.name
    }
}