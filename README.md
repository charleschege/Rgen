# Rgen
A fast, dead simple, zero allocation random data generator for Kenyan data.

### Modules
1. First Names: `rgen::first_name()`
2. Last Names: `rgen::last_name()`
3. Name prefix (Mr, Ms, etc) : `rgen::name_prefix()`
4. Title: `rgen::title_descriptor()`
5. Title Level: `rgen::title_level()`
6. Job Title: `rgen::title_job()`
7. Email provider: `rgen::email()`
8. Global Domain Suffix: `rgen::domain_suffix_global()`
9. Kenya Business Domain: `rgen::domain_suffix_kenya()`
10. Kenyan Government Domain: `rgen::domain_suffix_kenya_gov()`
11. Kenyan Education Institution Domain: `rgen::domain_suffix_kenya_edu()`
12. Major cities: `rgen::major_cities_kenya()`
13. Counties: `rgen::counties()`
14. User-Agent String: `rgen::user_agent_string()`
15. Industry: `rgen::industry()`
16. Profession: `rgen::profession()`
17. Co-ordinates: `rgen::coordinates_major()`
18. Country: `rgen::country()`
19. Country Code: `rgen::country_code()`
20. Street Suffix: `rgen::street_suffix()`
21. Timezone: `rgen::timezone()`

### Usage
```
extern crate rgen;

use rgen::*;

fn main() {
  let name = rgen::gen_first_name;
  let county = rgen::counties;

  println!("{} is from {} county.", name, county);
}

```
LICENSE: `APACHE-2.0`