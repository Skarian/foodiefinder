use colored::*;
use psl::domain_str;
use regex::Regex;
use std::{fs, path::Path};
use url::Url;

fn main() {
    let input_filename = String::from("./input_hosts.txt");
    let output_filename = String::from("./output_hosts.txt");

    let input_len = input_hosts_file_len(&input_filename);

    println!(
        "Input file has {} hosts to assess",
        input_len.to_string().green()
    );

    let hosts = process_input_hosts_file(&input_filename);

    println!(
        "Have found {} valid hosts in `{}`, and will push results to {}",
        hosts.len().to_string().green().bold(),
        &input_filename.green().bold(),
        &output_filename.green().bold()
    );

    store_output_hosts_file(hosts, &output_filename);

    println!("{}", "Have successfully outputted results!".green());
}

fn process_input_hosts_file(filename: &str) -> Vec<String> {
    let input_hosts = read_from_input_hosts_file(filename);
    let mut output_hosts: Vec<String> = Vec::new();

    for host in &input_hosts {
        let url = extract_url(host);
        match url {
            Some(valid_url) => {
                let parsed_url = Url::parse(valid_url.as_str()).unwrap();

                let host = parsed_url.host_str().unwrap();

                let domain_name = domain_str(host).unwrap();
                output_hosts.push(domain_name.to_owned());
            }
            None => println!("The url is not valid!"),
        }
    }
    output_hosts
}

fn read_from_input_hosts_file(input_filename: &str) -> Vec<String> {
    let path = Path::new(input_filename);
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn input_hosts_file_len(input_filename: &str) -> usize {
    let path = Path::new(input_filename);
    fs::read_to_string(path).unwrap().lines().count()
}

fn extract_url(url: &str) -> Option<String> {
    let re = Regex::new(r"http(s)?://[a-z0-9.-]+/?").unwrap();

    let caps = re.captures(url);
    caps.map(|cap| cap.get(0).map_or("", |m| m.as_str()).to_string())
}

fn store_output_hosts_file(hosts: Vec<String>, output_filename: &str) {
    fs::write(output_filename, hosts.join("\n")).expect("");
}
