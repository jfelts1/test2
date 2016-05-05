//James Felts
//Test file for learning rust

fn main(){
    var_bindings();
    functions();
    arrays();
    tuples();
    control_flow();
    structs();
    matches();
    patterns();
    method_calls();
    vectors();
    traits();
    println!("\nerror_handling begin");
    let file_name = "foobar.rs";
    println!("file:{}",file_name);
    match find(file_name,'.'){
        None=>println!("No file extension found."),
        Some(i)=>println!("File Extension: {}",&file_name[i+1..]),
    }

    let file2 = "abc";
    println!("file:{}",file2);
    match find(file2,'.'){
        None=>println!("No file extension found."),
        Some(i)=>println!("File Extension: {}",&file2[i+1..]),
    }

    println!("error_handling end")
}

struct Point {
    x:f32,
    y:f32,
}

struct Circle{
    x:f64,
    y:f64,
    radius:f64,
}

struct Square{
    side:f64,
}

//drop
struct Firework{
    strength:i32,
}
//drop
impl Drop for Firework{
    fn drop(&mut self){
        println!("BOOM times {}!!!",self.strength);
    }
}

impl Firework{
    fn light_firework(&self){
        println!("firework has been light!");
    }
}

//traits on generic structs
#[derive(Debug)]
struct Rectangle<T>{
    x:T,
    y:T,
    w:T,
    h:T,
}
//traits on generic structs
impl<T:PartialEq> Rectangle<T>{
    fn is_square(&self)->bool{
        self.w == self.h
    }
}

trait HasArea{
    fn area(&self)->f64;
}

impl HasArea for Circle{
    fn area(&self)->f64{
        std::f64::consts::PI*(self.radius*self.radius)
    }
}

impl HasArea for Square{
    fn area(&self)->f64{
        self.side * self.side
    }
}

//defining methods
impl Circle{

    //Associated function
    //note how it doesn't take a self parameter
    fn new(x:f64,y:f64,radius:f64)->Circle{
        Circle{
            x:x,
            y:y,
            radius:radius,
        }
    }

    fn grow(&self,increment:f64)->Circle{
        Circle{x:self.x,y:self.y,radius:self.radius + increment}
    }
}

//Builder Pattern
//allows for default values
struct CircleBuilder{
    x:f64,
    y:f64,
    radius:f64,
}
//Builder Pattern
//allows for default values
impl CircleBuilder{
    fn new()->CircleBuilder{
        CircleBuilder{x:0.0,y:0.0,radius:1.0,}
    }

    fn x(&mut self, coordinate:f64)->&mut CircleBuilder{
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate:f64)->&mut CircleBuilder{
        self.y = coordinate;
        self
    }

    fn radius(&mut self, coordinate:f64)->&mut CircleBuilder{
        self.radius = coordinate;
        self
    }

    fn finalize(&self)->Circle{
        Circle{x:self.x,y:self.y,radius:self.radius}
    }
}

enum Mess{
    One,
    Two,
    Three,
}

enum OptionalInt{
    Value(i32),
    Missing
}

fn var_bindings(){
    println!("\nvar_bindings start");
    //type inference x is and i32
    let x=5;
    println!("x's value is: {}",x);
    //explicitly stating the type is i32
    let y:i32 = 4;
    println!("y is {}",y);
    //types are not mutable by default
    let mut var = 2;
    println!("var is {}",var);
    var = 9;
    println!("var is now {}",var);
    let s = "string";
    foo2(s);
    println!("{}",s);
    println!("var_bindings end");
}

fn functions(){
    println!("\nfunctions start");
    let x :i32=5;
    //function calling
    foo(x);

    let y = 6;
    let val;
    val = sum(x,y);
    println!("sum of x and y is {}",val);
    let z:f32=2.0;
    let x1:f32 = 5.0;
    let y1:f32 =6.0;
    //functions with return values
    let res = fma(x1,y1,z);
    println!("fma of {},{},{} is {}",x1,y1,z,res);

    //function pointers
    let f=foo;
    f(7);
    //function pointers to function pointers
    let t=ptr_test;
    t(f,10);
    println!("functions end");
}

fn arrays(){
    println!("\narrays start");
    //arrays
    let a = [1,2,3];
    let mut m = [1,2,3];
    println!("a has length {}\nm has length {}",a.len(),m.len());
    println!("m[0] is:{}",m[0]);
    m[0] = 5;
    println!("m[0] now is:{}",m[0]);

    //slicing
    m[0]=1;
    let beg = &m[0..1];
    println!("beg[0] is {}",beg[0]);
    println!("arrays end");
}

fn tuples(){
    println!("\ntuples start");
    //tuple
    let tup = (4,"hello");
    println!("{} {}",tup.1,tup.0);

    let (x,y,z) = (1,2.5,"hello");
    println!("{} {} {}",x,y,z);
    println!("tuples end");
}

fn control_flow(){
    println!("\ncontrol_flow start");
    //if
    let x = 5;
    let y = if x == 5{10}else{15};
    println!("y is {}",y);


    //while loop
    let mut x = 5;
    let mut done = false;
    println!("\nwhile loop enter");
    while !done{
        x+=x-3;

        println!("x:{}",x);
        if x%5 == 0{
            done = true;
        }
    }
    println!("while loop end\n\nfor loop enter");
    for i in 0..10 {
        println!("i:{}", i);
    }
    println!("for loop end\n\nenumerate example");
    //enumerate
    for (i,j) in (5..10).enumerate(){
        println!("times ran:{} value:{}",i,j);
    }
    println!("enumerate end\n\nloop enter");
    x = 5;
    loop {
        x+=x-3;
        println!("x:{}",x);
        if x % 5 == 0{
            break;
        }
    }
    println!("loop end\n\nodd values from 0 to 10 using continue");

    for i in 0..10{
        if i %2 == 0{
            continue;
        }
        println!("i:{}",i);
    }
    println!("control_flow end");
}

fn structs(){
    println!("\nstructs start");
    //struct
    let origin = Point{x:0.0,y:0.0};
    println!("Origin is at ({},{})",origin.x,origin.y);
    //mutable struct
    let mut p1 = Point{x:1.0,y:1.2};
    println!("p1({},{})",p1.x,p1.y);
    p1.x += 4.6;
    println!("p1({},{})",p1.x,p1.y);

    //update syntax
    let mut p2 = Point{x:1.2,y:1.3};
    println!("\np2({},{})",p2.x,p2.y);
    p2 = Point{x:2.2,..origin};
    println!("p2({},{})",p2.x,p2.y);
    println!("structs end");
}

fn matches(){
    //match
    println!("\nmatches begin");
    let x = 5;
    match x{
        1=>println!("1"),
        2=>println!("2"),
        3=>println!("3"),
        4=>println!("4"),
        5=>println!("5"),
        _=>println!("other"),
    }
    //can assign the return value from matches to variables
    let num = match x{
        1=>"one",
        2=>"two",
        3=>"three",
        4=>"four",
        5=>"five",
        _=>"other",
    };
    println!("{}",num);

    let mut m = Mess::One;
    take_mess(m);
    m = Mess::Two;
    take_mess(m);
    m = Mess::Three;
    take_mess(m);

    println!("matches end");
}

fn patterns(){
    //patterns
    println!("\npatterns start");
    //multiple patterns matched
    let mut x = 1;
    println!("x is {}",x);
    match x{
        1|2=>println!("one or two"),
        3=>println!("three"),
        _=>println!("not 1,2,or 3"),
    }
    x=2;
    println!("x is {}",x);
    match x{
        1|2=>println!("one or two"),
        3=>println!("three"),
        _=>println!("not 1,2,or 3"),
    }
    x=3;
    println!("x is {}",x);
    match x{
        1|2=>println!("one or two"),
        3=>println!("three"),
        _=>println!("not 1,2,or 3"),
    }
    x=4;
    println!("x is {}",x);
    match x{
        1|2=>println!("one or two"),
        3=>println!("three"),
        _=>println!("not 1,2,or 3"),
    }

    let p = Point{x:1.0,y:2.0};
    match p{
        Point {x,y}=>println!("p:({},{})",x,y),
    }

    //match a range of values with ...
    let t = 1;
    match t{
        1 ... 5=>println!("valid value"),
        _ => println!("invalid value"),
    }

    //can do this with chars
    let c = 'f';
    match c{
        'a'...'j'=>println!("a through j"),
        'k'...'z'=>println!("k through z"),
        _=>println!("something else"),
    }

    //can bind ranges to names to get the exact value matched
    let o = 2;
    match o{
        v@1...5=>println!("value is {}",v),
        _=>println!("other value"),
    }

    //can introduce guards by including an if expression
    let mut oi = OptionalInt::Value(5);
    println!("oi is 5");
    match oi{
        OptionalInt::Value(i) if i >5=>println!("value is bigger than 5"),
        OptionalInt::Value(..)=>println!("got and int"),
        OptionalInt::Missing =>println!("didn't get an int"),
    }
    oi = OptionalInt::Value(6);
    println!("oi is now 6");
    match oi{
        OptionalInt::Value(i) if i >5=>println!("value is bigger than 5"),
        OptionalInt::Value(..)=>println!("got and int"),
        OptionalInt::Missing =>println!("didn't get an int"),
    }
    oi = OptionalInt::Missing;
    println!("oi is now missing");
    match oi{
        OptionalInt::Value(i) if i >5=>println!("value is bigger than 5"),
        OptionalInt::Value(..)=>println!("got and int"),
        OptionalInt::Missing =>println!("didn't get an int"),
    }

    println!("patterns end");
}

fn method_calls(){
    println!("\nmethod_calls begin");
    //method calls
    let c = Circle{x:0.0,y:0.0,radius:2.0};
    println!("pos:({},{}), area:{}",c.x,c.y,c.area());
    //chaining method calls
    println!("pos:({},{}), area:{}",c.x,c.y,c.grow(2.0).area());
    //note since the grow method returned a new Circle c isn't changed
    println!("pos:({},{}), area:{}",c.x,c.y,c.area());
    let c2 = Circle::new(0.0,0.0,3.0);
    println!("pos:({},{}), area:{}",c2.x,c2.y,c2.area());
    //Builder pattern allows for default values
    let c3 = CircleBuilder::new().x(2.0).y(1.0).finalize();
    //radius defaults to 1.0
    println!("pos:({},{}), area:{}",c3.x,c3.y,c3.area());
    let c4 = CircleBuilder::new().radius(5.0).finalize();
    //defaults to x = 0 y = 0
    println!("pos:({},{}), area:{}",c4.x,c4.y,c4.area());
    println!("method_calls end");
}

fn vectors(){
    println!("\nvectors begin");
    //vectors
    let mut v = vec![1,2,3,4,5];
    println!("The third element of v is {}",v[2]);
    //iterating over vectors
    for i in &v{
        println!("A reference to {}",i);
    }
    v.pop();
    for i in &mut v {
        println!("A mutable reference to {}", i);
    }
    v.push(6);
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }

    println!("vectors end");
}

fn traits(){
    println!("\ntraits begin");
    let c = Circle{x:0.0,y:0.0,radius:2.0};
    print_area(c);
    let s = Square{side:2.0};
    print_area(s);

    let mut r = Rectangle{
        x:0,
        y:0,
        w:5,
        h:5,
    };

    println!("rectangle at ({},{}) is square? {}",r.x,r.y,r.is_square());
    r.h = 7;
    println!("r.h is now {}",r.h);
    //{:?} uses the debug format
    println!("{:?} is square? {}",r,r.is_square());

    //note that the drop happens when tnt leaves scope
    {
        let tnt = Firework { strength: 100 };
        tnt.light_firework();
    }
    println!("traits end");
}

fn foo(x:i32){
    println!("foo got: {}",x);
}

fn foo2(t:&str){
    println!("{}",t);
}

fn ptr_test(f:fn(i32),x:i32){
    f(x);
}

fn sum(x:i32, y:i32)->i32{
    x+y
}

fn fma(x:f32, y:f32, z:f32)->f32{
    x+(y*z)
}

fn take_mess(m:Mess){
    //match works on enums
    match m{
        Mess::One=>println!("One"),
        Mess::Two=>println!("Two"),
        Mess::Three=>println!("Three"),
    };
}

//because T requires the trait HasArea shape is guaranteed to have the function area
fn print_area<T:HasArea>(shape:T){
    println!("The area of the shape is {}",shape.area());
}

fn find(haystack: &str,needle: char)->Option<usize>{
    for (offset,c)in haystack.char_indices(){
        if c== needle{
            return Some(offset);
        }
    }
    None
}