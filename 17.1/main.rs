fn main() {
    println!("hello world");
    let goal = 12;
    let mut counter = 0;
    for touchdown in 0..((goal / 7) + 1) {
        for field_goals in 0..((goal / 3) + 1) {
            for safeties in 0..((goal / 2) + 1) {
                if safeties * 2 + field_goals * 3 + touchdown * 7 == goal {
                    println!("{} x safeties + {} x field goals + {} x touchdown", safeties, field_goals, touchdown);
                    counter += 1;
                }
            }
        }
    }
    println!("number of possible combination: {}", counter);
}
