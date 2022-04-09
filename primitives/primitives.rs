fn main()
{
    println!("I'm becoming a rustacean!");

    // Adding the '_'s to suppress rustc warnings.

    // Variables can be type annotated.
    let _my_bool: bool = true;

    //Regular annotation
    let _my_float: f64 = 1.0;
    // Suffix annotation
    let _my_int = 5i32;

    // Uses default type if not annotated.
    let _def_float = 3.0; // f64
    let _def_int   = 23;  // i32

    // Types can also be inferred
    let mut inferred = 12; // Type i64 inferred from line below
    inferred = 4294967296i64;

    // You can change the value of a mutable variable, but not
    // the type.
    let mut _changeme = 12; // mutable i32
    _changeme = 42;

    // This will result in error.
    // changeme = true;
    let _changeme = true; // works

    println!("{}", inferred);
}
