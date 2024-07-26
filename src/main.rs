fn main() {
    let month = ["second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let changes = ["Two turtle doves,", "Three French hens,", "Four calling birds,", "Five golden rings,", "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];
    println!("On the first day of Christmas,");
    println!("my true love gave to me");
    println!("A partridge in a pear tree.");
    for day in 0..11 {
        println!("On the {} day of Christmas,", month[day]);
        println!("my true love gave to me");
        for i in (0..=day).rev() {
            println!("{}", changes[i]);
        }
        println!("And a partridge in a pear tree.");
    }
}
