//James Felts
//Test file for learning rust

fn main(){
    var_bindings();
    functions();
    arrays();
    tuples();
    control_flow();
    structs();
}

struct Point {
    x:f32,
    y:f32,
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
