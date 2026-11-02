fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect()
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green"),
    ];

    // print_elements(&colors);

    // shorten_strings(&mut colors[1..3]);
    let uppercased = to_uppercase(&colors);

    println!("{:#?}", uppercased);
}
