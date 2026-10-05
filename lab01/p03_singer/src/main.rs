fn main() {
    let mut cnt: i32 = 99;
    let mut bottle_variant: String = _get_bottle_word_variant(cnt);
    while cnt > 0 {
        
        println!("{cnt} {bottle_variant} of beer on the wall,");
        println!("{cnt} {bottle_variant} of beer.");
        println!("Take one down, pass it around,");

        cnt -= 1;
        if cnt == 0 {
            println!("No bottles of beer on the wall.")
        }
        else {
            bottle_variant = _get_bottle_word_variant(cnt);
            println!("{cnt} {bottle_variant} of beer on the wall.");
            println!();
        }
    }
}

fn _get_bottle_word_variant(cnt: i32) -> String {
    let mut word: String = String::from("bottles");
    if cnt == 1 {
        word = String::from("bottle");
    }
    word
}
