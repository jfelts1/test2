fn main(){
    let x :i32=5;
    println!("x's value is: {}",x);
    //types are not mutable by default
    let mut var = 2;
    println!("var is {}",var);
    var = 9;
    println!("var is now {}",var);
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

    //arrays
    let a = [1,2,3];
    let mut m = [1,2,3];
    println!("a has length {}\nm has length {}",a.len(),m.len());
    println!("m[0] is:{}",m[0]);
    m[0] = 5;
    println!("m[0] now is:{}",m[0]);
}

fn foo(x:i32){
    println!("foo got: {}",x);
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