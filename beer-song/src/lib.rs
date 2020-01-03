pub fn verse(n: u32) -> String {
    if n == 0 {
        String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    } else if n == 1 {
        String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
    } else {
        let mut res = String::new();
        res.push_str(&n.to_string());
        res.push_str(" bottles of beer on the wall, ");
        res.push_str(&n.to_string());
        res.push_str(" bottles of beer.");
        res.push_str("\nTake one down and pass it around, ");
        res.push_str(&(n - 1).to_string());
        if n - 1 == 1 {
            res.push_str(" bottle");
        } else {
            res.push_str(" bottles");
        }
        res.push_str(" of beer on the wall.\n");
        res
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut res = String::new();
    let count = start - end + 1;
    let mut curr = start + 1;
    for j in 0..count {
        curr -= 1;
        res.push_str(&verse(curr));
        if j != count - 1 {
            res.push_str("\n");
        }
    }
    res
}
