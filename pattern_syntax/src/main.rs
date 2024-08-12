/*********************************/
// Matching Literals
/*********************************/
    fn pattern_matching_literals(){
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }

    }

/*********************************/
// Matching Named Variables
/*********************************/
    fn pattern_matching_named_variables(){
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y}");
    }

/*********************************/
// Multiple Patterns
/*********************************/
    fn pattern_matching_multiple_patterns(){

        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }

    }

/*********************************/
// Matching Ranges of Values with ..=
/*********************************/
    fn pattern_matching_range(){
        let x = 5;
        // In a match expression, 1..5 is not allowed because pattern matching with ranges requires the pattern to be inclusive or be a single value.
        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
        
        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

/***************************************/
// Destructuring to Break Apart Values
/***************************************/

    //structs
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    fn pattern_matching_destructuring_structs(){
        let p = Point { x: 0, y: 7 };

        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
        println!("{:?}", p);
    }

    //enums
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn pattern_matching_destructuring_enums(){
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}")
            }
        }
        // println!("{:?}", msg); //error: partial ownership( to resolve use match &msg {))
    }

    // Destructuring Nested Structs and Enums

    #[derive(Debug)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    #[derive(Debug)]
    enum Messages {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    fn pattern_matching_destructuring_nested(){
        // let msg = Messages::Move{x:5,y:5};
        let msg = Messages::ChangeColor(Color::Hsv(0, 160, 255));


        match msg {
            Messages::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Messages::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => println!("Printing default"),
        }
        println!("{:?}", msg); 
        // we wont be able to do this if we add varient Message::Write(text) => { println!("Text message: {text}");} inside match arm because it will take the ownership to resolve this we need to write "match &msg"
    }

    // Destructuring Structs and Tuples
    //   let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });


/*********************************/
// Ignoring values in a pattern
/*********************************/
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }
    fn ignore(){
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {setting_value:?}");


        //ignorinh specific elements
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}")
            }
        }

    }

    // we can  an Unused Variable by Starting Its Name with _ ex:  let _x = 5;

    fn binding(){
        let s= Some(String::from("Hello"));
        //  if let Some(_s) = s {
        //     println!("found a string");
        // }

        // println!("{s:?}");// in this case we will get error as s value will still be moved into _s, which prevents us from using s again. However, using the underscore by itself doesn’t ever bind to the value.
    if let Some(_) = s {
            println!("found a string");
        }
        println!("{s:?}");
    }
    //Ignoring Remaining Parts of a Value with ..
    fn ignoring_Some_parts(){
        let origin = Point { x: 0, y: 0};

        match origin {
            Point { x, .. } => println!("x is {x}"),
        }



        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
    // (.., second, ..) will give error as it wouldn't be clear 
    }


/***************************************/
// Extra Conditionals with Match Guards
/***************************************/

fn match_guard(){
    let num = Some(5);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), //4 | 5 | (6 if y) => ... match guard were applied only to the final value
        _ => println!("no"),
    }
}
/***************************************/
// @ Bindings
/***************************************/
enum MessageBinding {
    Hello { id: i32 },
}

fn binding_using_at(){
    let msg = MessageBinding::Hello { id: 5 };

    match msg {
        MessageBinding::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        MessageBinding::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        MessageBinding::Hello { id } => println!("Found some other id: {id}"),
    }
}
fn main() {
    pattern_matching_literals();
    pattern_matching_named_variables();
    pattern_matching_multiple_patterns();
    pattern_matching_range();
    pattern_matching_destructuring_structs();
    pattern_matching_destructuring_enums();
    pattern_matching_destructuring_nested();
    foo(3, 4);
    binding();
    ignoring_Some_parts();
    match_guard();
    binding_using_at();
}


