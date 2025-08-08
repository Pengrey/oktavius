use regex::Regex;
use serde::Deserialize;

// These structs correctly model the JSON structure. No changes are needed here.
#[derive(Deserialize, Debug)]
struct Data {
    title: String,
    sections: Vec<Section>,
}

#[derive(Deserialize, Debug)]
struct Section {
    fields: Vec<Field>,
}

#[derive(Deserialize, Debug)]
struct Field {
    key: String,
    value: String,
    _label: Option<String>,
}

pub fn extract_credentials_chrome(text: &str, verbose: bool) {
    let mut strings_list: Vec<String> = Vec::new();

    let re = Regex::new(r#"("title":.*?,"format":"url","key":"field\.website\.url","label":"Login URL","value":"([^"]+?)".*"key":"field\.login\.username","label":"Username","value":"([^"]+?)".*"key":"field\.login\.password","secret":true,"label":"Password","value":"([^"]+?)".??}]}]})"#).unwrap();

    for cap in re.captures_iter(text) {
        if let Some(group1) = cap.get(1) {
            let mut matched_str = group1.as_str().to_string();

            if let Some(cut_index) = matched_str.find("}  ") {
                matched_str = matched_str[..cut_index + 1].trim_end().to_string();
            } else {
                matched_str = matched_str.trim_end().to_string();
            }

            // Fix json
            matched_str = format!("{{ {}", &matched_str);

            if !strings_list.contains(&matched_str) && matched_str.len() > 20 {
                let result: Result<Data, _> = serde_json::from_str(&matched_str);

                match result {
                    Ok(credential) => {
                        println!("[+] Found Credential:");

                        let mut url = None;
                        let mut user = None;
                        let mut password = None;

                        for section in &credential.sections {
                            for field in &section.fields {
                                match field.key.as_str() {
                                    "field.website.url" => url = Some(field.value.clone()),
                                    "field.login.username" => user = Some(field.value.clone()),
                                    "field.login.password" => password = Some(field.value.clone()),
                                    _ => {}
                                }
                            }
                        }
                        println!("\t[>] Title: {}", credential.title);

                        if let Some(u) = url {
                            println!("\t[>] URL: {}", u);
                        } else {
                            println!("\t[>] URL: not found");
                        }
                        if let Some(u) = user {
                            println!("\t[>] User: {}", u);
                        } else {
                            println!("\t[>] User: not found");
                        }
                        if let Some(p) = password {
                            println!("\t[>] Password: {}", p);
                        } else {
                            println!("\t[>] Password: not found");
                        }
                    }
                    Err(_e) => {
                        if verbose {
                            println!("[+] Found Credential:");
                            println!("[!] Failed to parse JSON: {}", matched_str);
                        }
                    }
                }

                strings_list.push(matched_str);
            }
        }
    }
}

