use chrono::NaiveDate;
use chrono::Weekday;
use chrono::Utc;
use std::io;
use std::collections::HashMap;

extern crate ical;

use std::io::BufReader;
use std::fs::File;

use ical::IcalParser;
use chrono::Datelike;

use polars_core::prelude::*;
use polars::prelude::*;
use polars::prelude::lit;

fn main() {
    println!("Gestion des invitations");

    let (host_name, guest_name, invitation_date, play_date) = read_invitation_information();
    let date: String =Utc::now().format("%Y-%m-%d").to_string();

    
    let play_date: NaiveDate = play_date.parse().expect("Failed to parse date");
    let invitation_date: NaiveDate = invitation_date.parse().expect("Failed to parse date");
    
    let december: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 1).unwrap();
    let past_invitations = read_csv();

    let mut answer: Option<i64> = Some(1);
    let mut reason: String = String::from("None");
    
    if play_date < december {
        reason = String::from("L'invitation est demandé pour une date avant le 1er décembre 2023"); 
        println!("{:?}", reason);
    } else {
        println!("L'invitation est demandé pour une date après le 1er décembre 2023: NEXT STEP");
        let check_inivtation_date: i64 = play_date.signed_duration_since(invitation_date).num_days();
        if check_inivtation_date <= 0 {
            reason = String::from("La demande d'invitation est faite moins de 1 jours avant la date de jeu"); 
            answer = Some(0);
            println!("{:?}", reason);
        } else {
            println!("La demande d'invitation est faite au moins 3 jours avant la date de jeu: NEXT STEP");
            let evenements_path = "src/data/calendar_ics/Evenements.ics";
            let result_evenement = check_date_event_evenements(play_date, evenements_path);
            if result_evenement == false {
                reason = String::from("L'invitation est demandée un jour d'évenement interne");
                answer = Some(0);
                println!("{:?}", reason);
            }
            else{
                println!("L'invitation n'est pas demandée un jour d'évenement interne: NEXT STEP");
                let ic_path = "src/data/calendar_ics/Interclubs.ics";
                let result_ic = check_date_event_ic(play_date, ic_path);
                if result_ic == false {
                    reason = String::from("L'invitation est demandée un jour d'IC");
                    answer = Some(0);
                    println!("{:?}", reason);
                }
                else {
                    println!("L'invitation n'est pas demandée un jour d'IC: NEXT STEP");

                    let (result_max_invitation,nb_invitations_already_accepted) = check_max_invitations(guest_name.clone(), past_invitations.clone(), 4);
                    if result_max_invitation == false {
                        reason = format!("{} a déjà été invité {} fois", guest_name, nb_invitations_already_accepted);
                        answer = Some(0);
                        println!("{:?}", reason);
                    }
                    else {
                        if play_date.weekday()==Weekday::Thu{
                            let (result_invitations_accepted, n_invitations_accepted)= check_invitations_accepted(play_date, past_invitations.clone(), 2);
                            if result_invitations_accepted == false{
                                reason = format!("Il y a déjà {} invitations pour le jeudi {}", n_invitations_accepted, play_date);
                                answer = Some(0);
                                println!("{:?}", reason);
                            }
                        }
                        else if  play_date.weekday()==Weekday::Sun{
                            let (result_invitations_accepted, n_invitations_accepted) = check_invitations_accepted(play_date, past_invitations.clone(), 2);
                            if result_invitations_accepted == false{
                                reason = format!("Il y a déjà {} invitations pour le dimanche {}", n_invitations_accepted, play_date);
                                answer = Some(0);
                                println!("{:?}", reason);
                            }
                        }
                        else {
                            reason = String::from("L'invitation est demandé sur un autre jour qu'un créneau de jeu libre");
                            answer = Some(0);
                            println!("{:?}", reason);
                            
                        }
                    }
                }
            }
        }
    }
    let demande = df!(
        "date" => [date],
        "host_name" => [host_name],
        "play_date" => [play_date.to_string()],
        "invitation_date" => [invitation_date.to_string()],
        "guest_name" => [guest_name],
        "answer" => [answer.unwrap()],
        "reason" => [reason]
    ).unwrap();
    println!("{:?}", demande);

    let mut invitations = concat(
        [past_invitations.lazy(), demande.lazy()],
        UnionArgs::default(),
    ).unwrap().collect().unwrap();

    let mut path: File = File::create("src/data/invitations_historic.csv").unwrap();

    CsvWriter::new(&mut path)
        .with_separator(b';')
        .finish(&mut invitations)
        .unwrap();

}



fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn read_invitation_information() -> (String, String, String, String) {
    println!(
        "
    Entrez les prénom et nom de la personne qui invite 
    Mettre des majucules et un espace entre le nom et le prénom
    (exemple Alizée Masciulli):"
    );

    let mut host_name: String = String::new();

    io::stdin()
        .read_line(&mut host_name)
        .expect("Failed to read line");

    println!(
        "
        Entrez les prénom et nom de la personne qui est invitée 
        Mettre des majucules et un espace entre le nom et le prénom
        (exemple Adriana Masciulli):"
        );
    
    let mut guest_name: String = String::new();

    io::stdin()
        .read_line(&mut guest_name)
        .expect("Failed to read line");

    println!(
        "
    Entrez la date de la demande d'invitation 
    (exemple 2024-08-20):"
    );

    let mut invitation_date: String = String::new();

    io::stdin()
        .read_line(&mut invitation_date)
        .expect("Failed to read line");

    println!(
        "
    Entrez la date pour laquelle l'invitation est demandée
    (exemple 2024-08-20):"
    );

    let mut play_date: String = String::new();
    io::stdin()
        .read_line(&mut play_date)
        .expect("Failed to read line");

    println!("L'invitation à été demandé le: {invitation_date} pour le: {play_date} par: {host_name} pour {guest_name}");

    return (host_name, guest_name, invitation_date, play_date);
}

fn read_calendar(path: &str) -> IcalParser<BufReader<File>> {
    let buf = BufReader::new(File::open(&path).unwrap());

    let reader = ical::IcalParser::new(buf);

    return reader;
}

fn format_calendar(calendar_reader: IcalParser<BufReader<File>>) -> HashMap<String,  HashMap<String, Option<String>>>{
    let mut all_events: HashMap<String, HashMap<String, Option<String>>> = HashMap::new();

    for calendar in calendar_reader {
        let mut event_n: i32 = 0;
        for event in calendar.unwrap().events {
            let mut map: HashMap<String, Option<String>>= HashMap::new();
            for property in event.properties {
                map.insert(property.name, property.value);
            }
            event_n += 1;
            let event_str = format!("{}{}", "event", event_n);
            all_events.insert(event_str, map.clone());
        }
    }
    return all_events;
}

fn convert_date (date_str: &str) -> Option<NaiveDate> {

    let mut date_event: Option<NaiveDate> = None;

    if date_str.len() == 8 {
        date_event = Some(NaiveDate::parse_from_str(date_str, "%Y%m%d").expect("Failed to parse date)"));
    } else {
        date_event = Some(NaiveDate::parse_from_str(date_str, "%Y%m%dT%H%M%SZ").expect("Failed to parse date)"));
    }
    return date_event;
}

fn check_date_event_evenements(play_date: NaiveDate, path: &str) -> bool{

    let calendar_reader = read_calendar(path);
    let calendar = format_calendar(calendar_reader);

    for (_, properties) in calendar.into_iter() {
        let date_start = properties.get("DTSTART").unwrap();
        let date_start = convert_date(date_start.as_deref().unwrap()).unwrap();
        if play_date == date_start {
            return false;
        }
    }
    return true;
}

fn check_date_event_ic(play_date: NaiveDate, path: &str) -> bool {

    let calendar_reader = read_calendar(path);
    let calendar = format_calendar(calendar_reader);


    for (_, properties) in calendar.into_iter() {
        let date_start = properties.get("DTSTART").unwrap();
        let date_start = convert_date(date_start.as_deref().unwrap()).unwrap();
        if play_date == date_start {
            return false;
        }
    }
    return true;
}

fn check_date_event_holidays(play_date: NaiveDate, path: &str) {

    let calendar_reader = read_calendar(path);

    for calendar in calendar_reader {
        for event in calendar.unwrap().events {
            let mut date_start_holiday: Option<NaiveDate> = None;
            let mut date_end_holiday: Option<NaiveDate> = None;
            for property in event.properties {
                // println!("{:?}", property);

                if property.name == "DTSTART" {
                    let date_str = &property.value.as_deref().unwrap();
                    date_start_holiday = convert_date(date_str);
                }
                if property.name == "DTEND" {
                    let date_str = &property.value.as_deref().unwrap();
                    date_end_holiday = convert_date(date_str);
                }
                
            }

            if play_date >= date_start_holiday.unwrap() && play_date <= date_end_holiday.unwrap() {
                println!("L'invitation est demandée sur un jour de vacances scolaires, impossible de l'accepter");
                break;
            } else {
            }
        }
    }
}

fn read_csv() -> DataFrame {

    let parse_options = CsvParseOptions::default().with_separator(b';');

    let past_invitations = CsvReadOptions::default()
    .with_parse_options(parse_options)
    .try_into_reader_with_file_path(Some("src/data/invitations_historic.csv".into()))
    .unwrap()
    .finish()
    .unwrap();
    return past_invitations;
}

fn check_max_invitations(guest_name: String,past_invitations: DataFrame, nb_invitations_max:usize)-> (bool, usize){
    
    let past_invitations_accepted = past_invitations.lazy().filter(
        col("guest_name").eq(lit(guest_name.to_string())).and(col("answer").eq(1))
    ).collect().unwrap();
    
    println!("{:?}", past_invitations_accepted); 
    let last_chance = nb_invitations_max-1;
    let nb_invitations_already_accepted = past_invitations_accepted.shape().0;
    
    if nb_invitations_already_accepted == last_chance {
        println!("{guest_name} a déjà été invité {last_chance} fois");
        println!("Attention il s'agit de la dernière invitation possible pour cette personne: NEXT STEP");
        println!("{:?}", past_invitations_accepted);
        return (true, nb_invitations_already_accepted);
    } else if nb_invitations_already_accepted >= nb_invitations_max {
        return (false, nb_invitations_already_accepted); 
    } else {
        println!("{guest_name} n'a pas atteint son quota d'invitation de {nb_invitations_max}: NEXT STEP");
        println!("{:?}", past_invitations_accepted);
    }
    return (true, nb_invitations_already_accepted); 
}

fn check_invitations_accepted(play_date: NaiveDate, past_invitations: DataFrame, nb_invitations:usize)-> (bool, usize) {

    let past_invitations_accepted = past_invitations.lazy().filter(
        col("play_date").eq(lit(play_date.to_string())).and(col("answer").eq(1))
    ).collect().unwrap();

    println!("{:?}", past_invitations_accepted);
    let n_invitations_accepted = past_invitations_accepted.shape().0;

    if n_invitations_accepted >= nb_invitations {
        println!("{:?}", past_invitations_accepted);
        return (false,n_invitations_accepted);
    } 
    println!("Il y a moins de {nb_invitations} invitations pour ce jour");
    return (true,n_invitations_accepted); 
}