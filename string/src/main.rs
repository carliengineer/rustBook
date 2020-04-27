/*
    Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str. 
    String slices, which are references to some UTF-8 encoded string data stored elsewhere
    The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
    both String and string slices are UTF-8 encoded.
    Owned vs Borrowed
*/

fn main() {

    let mut hello = String::from("Dobrý den");
    let gok = " Gokhan";
    hello.push_str(gok);

    println!("{}", hello);

    // gok is still accessible after usage of push_str, because it doesn't take the ownership. It works with the str
    println!("{}", gok);
 
    let gok2 = String::from(" Sagirlar");
    // + operation requires borrowing for gok2 with & symbol
    //we can only add a &str to a String; we can’t add two String values together.
    let merge = hello + &gok2;
    //The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str. 
    //When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]

    println!("{}", merge);
    // !! Unlike gok2, hello cant be used. Because it's value is borrowed/moved by merge.
    //println!("{}", hello);
    println!("{}", gok2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //the format function below sets s to tic-tac-toe:
    let s = format!("{}-{}-{}", s1, s2, s3);

    /* Rust doesn't allow indexing for strings. Mainly due to different UTF-8 encodings of different letters, like russian,
    and accessing time to string character is not guaranteed to be constant O(1). So following returns an error: */
    // let char_1 = &s[0];

    /*To be more specific in your indexing and indicate that you want a string slice, rather than indexing using [] with a single number,
     you can use [] with a range to create a string slice containing particular bytes:*/
    let hello_rus = "Здравствуйте";
    let char_2 = &s[0..4];
    //What would happen if we used &hello[0..1]? The answer: Rust would panic at runtime in the same way as if an invalid index were accessed in a vector.
    //So, you should use ranges to create string slices with caution, because doing so can crash your program.

    //If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the chars method.:
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }


    //The bytes method returns each raw byte, which might be appropriate for your domain:
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    


}
