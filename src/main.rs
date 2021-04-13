extern crate requests;
use threadpool::ThreadPool;
//use std::sync::mpsc::{channel, RecvError};

use requests::ToJson;
use std::{io::{self, BufRead}};

fn main() {
    let stream = io::stdin();
    let pool = ThreadPool::new(10);
    //let (tx, rx) = channel();
    for domain in stream.lock().lines() {
        pool.execute(|| {
            check_av(&domain.unwrap())
        });
    }
    pool.join();
}

fn check_av(domain: &String) {
    let url = format!("https://ae.godaddy.com/domainfind/v1/search/exact?key=dpp_search&partialQuery={}&q={}&req_id=1616029383106&solution_set_ids=dpp-us-solution-tier1%2Cdpp-intl-solution-tier4%2Cdpp-intl-solution-tier6%2Co365-solutionset-tier3%2Cdpp-us-solution-fixed-tier4&itc=dpp_absol1", &domain, &domain);
    let res = requests::get(url);

    match res  {
        Ok(res) =>  {
            let status = res.json();
            match status {
                Ok(status) => {
                    println!("{}: {}", &domain, &status["ExactMatchDomain"]["AvailabilityLabel"]);
                },
                Err(_er) => println!("{}: Rate-Limit", &domain)
            };
        },
        Err(_err) => println!("{}: Request Error", &domain)
    };
}
