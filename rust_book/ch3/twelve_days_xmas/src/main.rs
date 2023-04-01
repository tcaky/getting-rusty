// Twelve Days of Christmas
// On the first day of Christmas, my true love sent to me
// A partridge in a pear tree.

// On the second day of Christmas, my true love sent to me
// Two turtle doves
// and a partridge in a pear tree.

// On the third day of Christmas, my true love sent to me
// Three French hens, two turtle doves
// And a partridge in a pear tree.

// On the fourth day of Christmas, my true love sent to me
// Four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree.

// On the fifth day of Christmas, my true love sent to me
// Five golden rings.
// Four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree.

// On the sixth day of Christmas, my true love gave to me
// Six geese a-laying,
// Five golden rings.
// Four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree.

// On the seventh day of Christmas, my true love gave to me
// Seven swans a-swimming, six geese a-laying,
// Five golden rings.
// Four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree.

// On the eighth day of Christmas, my true love gave to me
// Eight maids a-milking, seven swans a-swimming, six geese a-laying,
// Five golden rings.
// Four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree.

// On the ninth day of Christmas, my true love gave to me
// Nine ladies dancing, eight maids a-milking, seven swans a-swimming, six geese a-laying,
// Five golden rings.
// Four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree.

// On the tenth day of Christmas, my true love gave to me
// Ten lords a-leaping, nine ladies dancing, eight maids a-milking, seven swans a-swi'mmi'ng, six geese a-laying,
// Five golden rings.
// Fou'r calling birds, three French hens, two turtle doves
// And a partridge in a pear tree.

// On the eleventh day of Christmas, my true love gave to me
// Eleven pipers piping, ten lords a-leaping, nine ladies dancing, eight maids a-milking, seven swans a-swimming,
// Six geese a-laying,
// Five golden rings.
// Four calling birds, three French hens, two turtle doves And a partridge in a pear tree.

// On the twelfth day of Christmas, my true love gave to me
// Twelve drummers drumming, eleven pipers piping, ten lords a-leaping, nine ladies dancing, eight maids a-milking, seven swans a-swimming, six geese a-laying,
// Five golden rings.
// Four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree

fn main() {

    let values = [
        (1, "first", "a partridge in a pear tree"),
        (2, "second", "two turtle doves"),
        (3, "third", "three French hens"),
        (4, "fourth", "four calling birds"),
        (5, "fifth", "five golden rings"),
        (6, "sixth", "six geese a-laying"),
        (7, "seventh", "Seven swans a-swimming"),
        (8, "eighth", "eight maids a-milking"),
        (9, "ninth", "nine ladies dancing"),
        (10, "tenth", "ten lords a-leaping"),
        (11, "eleventh", "eleven pipers piping"),
        (12, "twelfth", "twelve drummers drumming"),
    ];   
    // let (value, pos_as_text, sentence_fragment) = values[0];
    // println!("{} {} {}", value, pos_as_text, sentence_fragment);
    // for (value, pos_as_text, sentence_fragment) in values.iter() {
    //     println!("{}", value);
    // } 

    for day in 0..12 {
        let (value, pos_as_text, sentence_fragment) = values[day];
        println!("{} {} {}", value, pos_as_text, sentence_fragment);
        println!("On the {} day of Christmas my true love gave to me", pos_as_text);
    
    }
}
