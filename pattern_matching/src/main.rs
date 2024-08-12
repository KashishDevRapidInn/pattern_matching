// match VALUE {
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
// }

/*********************************/
// Option
/*********************************/
fn pattern_matching_on_options(){
    let num:Option<u8>= Some(5);
    match num{
        Some(value)=> println!("{:?}", num),
        None=> println!("None value",)
    }
}

/*********************************/
// pattern_matching_on_conditionals
/*********************************/
fn pattern_matching_on_conditionals(){
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    // Error: You should use expect on the Result itself, not on the type annotation.
    // let age: Result<u8, _> = "34".parse().expect("Failed to parse");
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

/*********************************/
// while let Conditional Loops
/*********************************/
fn pattern_matching_on_while_let(){

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    // stack.pop() return an option
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

}

/*********************************/
// for Loops
/*********************************/
fn pattern_matching_on_loops(){
    let v = vec!['a', 'b', 'c'];
    //  The first value produced is the tuple (0, 'a'). When this value is matched to the pattern (index, value), index will be 0 and value will be 'a', printing the first line of the output.
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

}

/*********************************/
// let Statements
/*********************************/
fn pattern_matching_on_let(){
    let x = 5;
    let (x, y, z) = (1, 2, 3);
    // let (x, y) = (1, 2, 3); // Error
    let (x, y, _) = (1, 2, 3);
    println!("{},{},{}", x,y,z);
}
/*********************************/
// Function Parameters
/*********************************/
fn pattern_matching_on_fn(&(x, y): &(i32, i32)){
    println!("Current location: ({x}, {y})");
}

fn main() {
    pattern_matching_on_options();
    pattern_matching_on_conditionals();
    pattern_matching_on_while_let();
    pattern_matching_on_loops();
    pattern_matching_on_let();
    let point = (3, 5);
    pattern_matching_on_fn(&point);
}


