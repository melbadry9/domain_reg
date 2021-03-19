extern crate requests;

use requests::ToJson;
use std::{io::{self, BufRead}};

fn main() {
    let stream = io::stdin();
    for domain in stream.lock().lines() {
        check_av(&domain.unwrap());
    }
}

fn check_av(domain: &String) {
    let url = format!("https://ae.godaddy.com/domainfind/v1/search/exact?key=dpp_search&partialQuery={}&q={}&req_id=1616029383106&solution_set_ids=dpp-us-solution-tier1%2Cdpp-intl-solution-tier4%2Cdpp-intl-solution-tier6%2Co365-solutionset-tier3%2Cdpp-us-solution-fixed-tier4&itc=dpp_absol1", &domain, &domain);
    let res = requests::get(url);

    match res  {
        Ok(res) =>  println!("{}: {}", &domain, res.json().unwrap()["ExactMatchDomain"]["AvailabilityLabel"]),
        Err(_err) => println!("{}: ERROR", &domain)
    };
}
