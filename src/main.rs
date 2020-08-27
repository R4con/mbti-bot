extern crate reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut output_list: Vec<String> = Vec::new();

    let response = reqwest::blocking::get("https://www.tumblr.com/search/%23intj")?;
        println!("Status: {}", response.status());
        let body = response.text()?;
        let body_list: Vec<&str> = body.split("<img").collect();
        println!("{}", body);

        //filter for all tags
        for item in body_list {
            let split_str: Vec<&str> = item.split(">").collect();
            let mut full_tag = split_str[0].trim().to_string();
            

            match full_tag.find("src=\"") {
                Some(index) => {
                    let mut right_tag = full_tag.split_off(index+5);

                    match right_tag.find("\"") {
                        Some(index) => {
                            let left_tag = right_tag.split_off(index);
                            output_list.push(right_tag);
                        },
                        None => {}
                    }
                },
                None => {}
            }
        }

        let mut count = 0;
        for item in output_list {
            println!("{}: '{}'", count, item);
            count += 1;
        }

    Ok(())
}
