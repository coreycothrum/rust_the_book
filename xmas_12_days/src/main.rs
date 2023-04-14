fn main() {
    let lyrics = vec![
        "a partridge in a pear tree",
        "two turtle doves and",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for nth in 0..12 {
        println!("on day {} of christmas, my true love gave to me:", nth+1);
        for day in (0..=nth).rev() {
            println!("{}", lyrics[day]);
        }
    }
}
