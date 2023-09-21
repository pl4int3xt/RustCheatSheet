fn main(){
  println!("{:?}", meeting("Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn"));
}

fn meeting(s: &str) -> String {
  let mut guests: Vec<(&str, &str)> = s
    .split(';')
    .map(|name| {
      let parts: Vec<&str> = name.split(':').collect();
        (parts[1], parts[0])
    })
    .collect();
  guests.sort_by(|a, b| {
    if a.0 != b.0 {
        a.0.cmp(&b.0)
    } else {
        a.1.cmp(&b.1)
    }
  });

  let result: Vec<String> = guests
    .into_iter()
    .map(|(last_name, first_name)| format!("({}, {})", last_name.to_uppercase(), first_name.to_uppercase()))
    .collect();

  result.join("")

}