use reqwest;
use scraper::{Html, Selector};
use rayon::prelude::*;

#[derive(Debug)]
struct Class {
    class: String,
    wins: i32,
    deaths: i32,
    win_percent: f64,
}

impl Class {
    fn new(class: String, wins: i32, deaths: i32) -> Self {
        let win_percent: f64 = (wins as f64 / deaths as f64) * 100f64;
        Class {
            class,
            wins,
            deaths,
            win_percent,
        }
    }

    fn display(&self) {
        println!("{} |{} |{} |{}", self.class, self.wins, self.deaths, self.win_percent);
    }
}

fn main() {
    let classes = vec!["adventurer", "alchemist", "annihilator", "anorithil", "arcane blade", "archer", "archmage", "berserker",
                       "brawler", "bulwark", "corruptor", "cultist of entropy", "cursed", "demonologist", "doombringer", "doomed",
                       "gunslinger", "marauder", "mindslayer", "necromancer", "oozemancer", "paradox mage", "possessor", "psyshot",
                       "reaver", "rogue", "sawbutcher", "shadowblade", "skirmisher", "solipsist", "stone warden", "summoner",
                       "sun paladin", "temporal warden", "wanderer", "wyrmic", "writhing one"];

    println!("Class           | Wins  | Deaths  | Percentage");

    classes.par_iter().for_each(|x| get_class_stats(x).display());
}

fn get_class_stats(class: &str) -> Class {
    let mut page_num = 0;
    let mut wins = 0;

    let class_num = class_to_num(class);

    let url = format!("https://te4.org/characters-vault?tag_name=&tag_level_min=&tag_level_max=&tag_winner=winner&tag_difficulty[]=36&tag_class[]={}&tag_game[]=699172&page=0", class_num);

    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();

    let document = Html::parse_document(&body);
    let selector_odd = Selector::parse("tr.odd").unwrap();
    let selector_even = Selector::parse("tr.even").unwrap();

    let mut elements: Vec<_> = document
        .select(&selector_odd)
        .chain(document.select(&selector_even))
        .map(|x| x.inner_html())
        .collect();

    while &elements[0] != "<td colspan=\"3\">No characters available.</td> " {
        println!("Doing page no. {} for {} (Winners)", page_num, class);

        for _element in &elements {
            wins += 1;
        }

        page_num += 1;

        let url = format!("https://te4.org/characters-vault?tag_name=&tag_level_min=&tag_level_max=&tag_winner=winner&tag_difficulty[]=36&tag_class[]={}&tag_game[]=699172&page={}", class_num, page_num);

        let resp = reqwest::blocking::get(url).unwrap();
        assert!(resp.status().is_success());

        let body = resp.text().unwrap();

        let document = Html::parse_document(&body);

        elements = document
            .select(&selector_odd)
            .chain(document.select(&selector_even))
            .map(|x| x.inner_html())
            .collect();
    }

    if elements[0].as_str() == "<td colspan=\"3\">No characters available.</td> " {
        println!("Reached end of {} winners.", class);
    }


    let mut page_num = 0;
    let mut deaths = 0;

    let url = format!("https://te4.org/characters-vault?tag_name=&tag_level_min=&tag_level_max=&tag_dead=dead&tag_difficulty[]=36&tag_class[]={}&tag_game[]=699172&page=0", class_num);

    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();

    let document = Html::parse_document(&body);
    let selector_odd = Selector::parse("tr.odd").unwrap();
    let selector_even = Selector::parse("tr.even").unwrap();

    let mut elements: Vec<_> = document
        .select(&selector_odd)
        .chain(document.select(&selector_even))
        .map(|x| x.inner_html())
        .collect();

    while &elements[0] != "<td colspan=\"3\">No characters available.</td> " {
        println!("Doing page no. {} for {} (Deaths)", page_num, class);

        for _element in &elements {
            deaths += 1;
        }

        page_num += 1;

        let url = format!("https://te4.org/characters-vault?tag_name=&tag_level_min=&tag_level_max=&tag_dead=dead&tag_difficulty[]=36&tag_class[]={}&tag_game[]=699172&page={}", class_num, page_num);

        let resp = reqwest::blocking::get(url).unwrap();
        assert!(resp.status().is_success());

        let body = resp.text().unwrap();

        let document = Html::parse_document(&body);

        elements = document
            .select(&selector_odd)
            .chain(document.select(&selector_even))
            .map(|x| x.inner_html())
            .collect();
    }

    if elements[0].as_str() == "<td colspan=\"3\">No characters available.</td> " {
        println!("Reached end of {} deaths.", class);
    }

    Class::new(class.to_string(), wins, deaths)
}

fn class_to_num(class: &str) -> &str {
    match class {
        "adventurer" => "104",
        "alchemist" => "19",
        "annihilator" => "326744",
        "anorithil" => "20",
        "arcane blade" => "22",
        "archer" => "14",
        "archmage" => "7",
        "berserker" => "16",
        "brawler" => "56",
        "bulwark" => "80",
        "corruptor" => "34",
        "cultist of entropy" => "133921",
        "cursed" => "10",
        "demonologist" => "23297",
        "doombringer" => "23313",
        "doomed" => "29",
        "gunslinger" => "208",
        "marauder" => "71",
        "mindslayer" => "48",
        "necromancer" => "68",
        "oozemancer" => "179",
        "paradox mage" => "43",
        "possessor" => "95691",
        "psyshot" => "67509",
        "reaver" => "31",
        "rogue" => "12",
        "sawbutcher" => "67403",
        "shadowblade" => "23",
        "skirmisher" => "12400",
        "solipsist" => "102",
        "stone warden" => "70",
        "summoner" => "17",
        "sun paladin" => "27",
        "temporal warden" => "49",
        "wanderer" => "699245",
        "writhing one" => "104071",
        "wyrmic" => "4",
        _ => "0",
    }
}
