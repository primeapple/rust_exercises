const DAY_NUMBERING: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eigth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
];
const PRESENTS: [&str; 12] = [
    "And a partridge in a pear tree",
    "Two turtledoves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings (five golden rings)",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"
];

fn main() {
    for index in 0..=11 {
        println!("On the {} day of Christmas, my true love sent to me",
            DAY_NUMBERING[index]);
        for index in (0..=index).rev() {
            println!("{}", PRESENTS[index]);
        }
        println!("");
    }
}
