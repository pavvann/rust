use std::collections::HashMap;

fn main() {

    let text = "visit beyondclub.xyz :)";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);
    
    /*
    // 
    // let blue = String::from("Blue");
    // let yellow = String::from("Yellow");

    // let mut scores = HashMap::new();

    // scores.insert(blue, 10);
    // scores.insert(yellow, 50 );
    // 

    // 
    // let hello = String::from("नमस्ते");

    // // bytes
    // for b in hello.bytes() {
    //     println!("{}",b);
    // }

    // // scalar 
    // for c in hello.chars() {
    //     println!("{}", c);
    // }
     
    // // Graphene clusters
    // for g in hello.graphemes(true) {
    //     println!("{}", g);
    // }
    // 

    // 
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // match &row[1] {
    //     SpreadsheetCell::Int(i) => println!("{}", i),
    
    //     _ => println!("Not an integer")
    // };
    // 

    // 
    // let mut v = vec![1,2,3,4,5];
    
    // for i in &mut v {
    //     *i += 50;
    // }
    // for i in &v {
    //     println!("{}", i);
    // }
    
    // let third = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }
    // 
    */

}
