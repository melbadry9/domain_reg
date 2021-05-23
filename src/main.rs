use clap::{App, Arg};
use reqwest;
use serde_json::{Map, Value};
use std::io::{self, BufRead};
use threadpool::ThreadPool;

fn main() {
    let args = App::new("Check Domain Availability")
        .version("0.1")
        .author("Mohamed Elbadry <me@melbadry9.xyz>")
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .value_name("THREADS")
                .help("Sets number of threads")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .value_name("DOMAIN")
                .help("Sets domain to check")
                .takes_value(true),
        )
        .get_matches();

    if !(args.is_present("domain")) {
        let stream = io::stdin();
        let pool = ThreadPool::new(
            args.value_of("threads")
                .unwrap_or("3")
                .parse::<usize>()
                .unwrap_or(3),
        );
        for domain in stream.lock().lines() {
            pool.execute(|| check_av(&domain.unwrap()));
        }
        pool.join();
    } else {
        check_av(&args.value_of("domain").unwrap().to_string())
    }
}

fn check_av(domain: &String) {
    let url = format!("https://ae.godaddy.com/domainfind/v1/search/exact?key=dpp_search&partialQuery={}&q={}&req_id=1616029383106&solution_set_ids=dpp-us-solution-tier1%2Cdpp-intl-solution-tier4%2Cdpp-intl-solution-tier6%2Co365-solutionset-tier3%2Cdpp-us-solution-fixed-tier4&itc=dpp_absol1", &domain, &domain);
    let res = reqwest::blocking::get(url);

    match res {
        Ok(res) => {
            if res.status().is_success() {
                let js: Value = res.json().unwrap();
                let obj: Map<String, Value> = js.as_object().unwrap().clone();
                println!(
                    "{}: {}",
                    &domain,
                    &obj["ExactMatchDomain"]["AvailabilityLabel"]
                        .as_str()
                        .unwrap()
                )
            }
        }
        Err(_err) => println!("{}: Request Faild", &domain),
    };
}
