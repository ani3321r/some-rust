fn main() {
    // Extend a collection
    let grade = Some("a+");
    let mut grades = vec!["b+", "c-", "d"];

    // it extends a collection with the contents of an iterator
    grades.extend(grade);

    println!("{grades:?}");


    //extend an iterator
    let grades1 = vec!["a-", "b+", "d-"];
    for grade in grades1.iter().chain(grade.iter()){
        println!("{grade}");
    }


    // filter out none variants
    let grades2 = vec![Some("a-"), None, Some("c+"), None];
    let grades2: Vec<&str> = grades2.into_iter().flatten().collect();

    println!("{grades2:?}");


    // map and filter out none variants
    let grades3 = ["4.5", "b-", "5.0", "a+", "3.9"];
    let grades3: Vec<f32> = grades3
                                .iter()
                                .filter_map(|s| s.parse().ok())
                                .collect();
    println!("{grades3:?}");
}