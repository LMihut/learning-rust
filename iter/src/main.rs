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

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el))
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green"),
    ];

    // print_elements(&colors);
    // shorten_strings(&mut colors[1..3]);

    // let uppercased = to_uppercase(&colors);
    // println!("{:#?}", uppercased);

    let mut destination = vec![];
    move_elements(colors, &mut destination);
    println!("{:#?}", destination);
}
