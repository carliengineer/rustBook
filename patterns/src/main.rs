
struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}



fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}


fn main() {
    //we use patterns in the arms of match expressions.
    //The downside of using if let expressions is that the compiler doesn’t check exhaustiveness, whereas with match expressions it does.

    let favorite_color: Option<&str> = None;
    let is_tuesday = true;
    let age: Result<u8, _> = "34".parse();


    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
        //if let can also introduce shadowed variables in the same way that match arms can:
        //the line if let Ok(age) = age introduces a new shadowed age variable that contains the value inside the Ok variant.
    } else if let Ok(age) = age {
        //we can’t combine these two conditions into if let Ok(age) = age && age > 30
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }


    //while let conditional loop allows a while loop to run for as long as a pattern continues to match
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }



    let v = vec!['a', 'b', 'c'];
    //we use the enumerate method to adapt an iterator to produce a value and that value’s index in the iterator, placed into a tuple
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //Patterns come in two forms: refutable and irrefutable.
    // Patterns that will match for any possible value passed are irrefutable. An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match.
    // Patterns that can fail to match for some possible value are refutable.example: Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.


    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }


    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        //introduces a new variable named y that will match any value inside a Some value.
        // Because we’re in a new scope inside the match expression, this is a new y variable, not the y we declared at the beginning with the value 10.
        // This new y binding will match any value inside a Some, which is what we have in x. Therefore, this new y binds to the inner value of the Some in x.
        // That value is 5, so the expression for that arm executes and prints Matched, y = 5. If x had been a None value instead of Some(5), the patterns in the first two arms wouldn’t have matched.
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }



    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        6 | 7 => println!("six or seven"),
        _ => println!("something else"),
    }


    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    //there is a shorthand for patterns that match struct fields: you only need to list the name of the struct field, and the variables created from the pattern will have the same names
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }


    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }

    //you can tell Rust not to warn you about the unused variable by starting the name of the variable with an underscore
    let _x = 5;

    let origin = Point2 { x: 0, y: 0, z: 0 };
    //ignoring parts of value
    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }

    //match guard
    let num = Some(4);
    match num {
        //A match guard is an additional if condition specified after the pattern in a match arm that must also match, along with the pattern matching, for that arm to be chosen.
        //There is no way to express the if x < 5 condition within a pattern, so the match guard gives us the ability to express this logic.
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }


    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        //below doesn’t introduce a new variable y that would shadow the outer y, meaning we can use the outer y in the match guard
        // Instead of specifying the pattern as Some(y), which would have shadowed the outer y, we specify Some(n).
        //This creates a new variable n that doesn’t shadow anything because there is no n variable outside the match.
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }


    //The at operator (@) lets us create a variable that holds a value at the same time we’re testing that value to see whether it matches a pattern.
    //we want to test that a Message::Hello id field is within the range 3..=7. But we also want to bind the value to the variable id_variable so we can use it in the code associated with the arm
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
    //By specifying id_variable @ before the range 3..=7, we’re capturing whatever value matched the range while also testing that the value matched the range pattern.

}
