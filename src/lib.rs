#![allow(warnings)]
extern crate rand;
extern crate signifix;

use {
    rand::{thread_rng, Rng}
};

mod data;

use data::{
    FIRST_NAMES, LAST_NAMES, NAME_PREFIX, TITLE_DESCRIPTOR, TITLE_LEVEL, TITLE_JOB, EMAIL_PROVIDER, DOMAIN_SUFFIX_GLOBAL, DOMAIN_SUFFIX_KENYA, DOMAIN_SUFFIX_KENYA_EDUCATION, DOMAIN_SUFFIX_KENYA_GOVERMENT, MAJOR_CITIES_KENYA, COUNTIES, USER_AGENT, PASSWORD_CHARS, INDUSTRY, PROFESSION, COORDIANTES_MAJOR, COUNTRY, COUNTRY_CODE, STREET_SUFFIX, TIMEZONE, };

fn random_gen(data_type: &'static[&'static str]) -> &str {
    let size = data_type.len();
    let data_type = data_type[thread_rng().gen_range(0, size)];

    data_type
}

pub fn first_name() -> &'static str {
    random_gen(FIRST_NAMES)
}

pub fn last_name() -> &'static str {
    random_gen(LAST_NAMES)
}

pub fn name_prefix() -> &'static str {
    random_gen(NAME_PREFIX)
}

pub fn title_descriptor() -> &'static str {
    random_gen(TITLE_DESCRIPTOR)
}

pub fn title_level() -> &'static str {
    random_gen(TITLE_LEVEL)
}

pub fn title_job() -> &'static str {
    random_gen(TITLE_JOB)
}

pub fn email_provider() -> &'static str {
    random_gen(EMAIL_PROVIDER)
}

pub fn domain_suffix_global() -> &'static str {
    random_gen(DOMAIN_SUFFIX_GLOBAL)
}

pub fn domain_suffix_kenya() -> &'static str {
    random_gen(DOMAIN_SUFFIX_KENYA)
}

pub fn domain_suffix_kenya_gov() -> &'static str {
    random_gen(DOMAIN_SUFFIX_KENYA_GOVERMENT)
}

pub fn domain_suffix_kenya_edu() -> &'static str {
    random_gen(DOMAIN_SUFFIX_KENYA_EDUCATION)
}

pub fn major_cities_kenya() -> &'static str {
    random_gen(MAJOR_CITIES_KENYA)
}

pub fn counties() -> &'static str {
    random_gen(COUNTIES)
}

pub fn user_agent_string() -> &'static str {
    random_gen(USER_AGENT)
}

pub fn industry() -> &'static str {
    random_gen(INDUSTRY)
}

pub fn profession() -> &'static str {
    random_gen(PROFESSION)
}

pub fn coordinates_major() -> &'static str {
    random_gen(COORDIANTES_MAJOR)
}

pub fn country() -> &'static str {
    random_gen(COUNTRY)
}

pub fn country_code() -> &'static str {
    random_gen(COUNTRY_CODE)
}

pub fn street_suffix() -> &'static str {
    random_gen(STREET_SUFFIX)
}

pub fn timezone() -> &'static str {
    random_gen(TIMEZONE)
}



#[test]
fn test_random_data() {
    println!("FIRST NAME: {}\nLAST NAME: {}\nNAME PREFIX: {}\nEMAIL: {}@{}\nDOMAIN: {}\nCOUNTY: {}",
        first_name(), last_name(), name_prefix(), String::from("cc"), email_provider(), domain_suffix_kenya(), counties()
    );
}
