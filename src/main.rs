use num_bigint::BigUint;//handles arbitrarily large unsigned integers
use num_integer::Integer;//offers methods for integer operations, 
//such as division with remainder.

fn main() {
    // Large number as a string
    let large_number = "83077777777738388939993993090300030030030337373878383838383";
    // Divisor as a string
    let divisor = "188889922677772777277277727727727727272772727277272";
    
    // Parse the numbers as BigUint.  //Unwrap() handles any potential mistakes
    // during parsing by halting the application if parsing fails.
    
    let num = BigUint::parse_bytes(large_number.as_bytes(), 10).unwrap();
    let div = BigUint::parse_bytes(divisor.as_bytes(), 10).unwrap();
      
    // Perform division and note the remainder.
    //div_rem is used to divide num by div, and returns a tuple
    // containing the quotient and the remainder.
    let (quotient, remainder) = num.div_rem(&div);
    
    // Print the result
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
}

