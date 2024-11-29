use chrono::NaiveDate;
use chrono::Utc;
use chrono::Weekday;
use rusqlite::params;
use std::collections::HashMap;

extern crate ical;
use std::option::Option;

use std::fs::File;
use std::io::BufReader;

use chrono::Datelike;
use ical::IcalParser;

use rusqlite::Connection;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let member = &args[1];
    let display_name = &args[2];
    let guest_name = &args[3];
    let play_date_str = &args[4];

    let date: NaiveDate = Utc::now().date_naive();
    let date_str: String = date.format("%Y-%m-%d").to_string();

    let play_date: NaiveDate = play_date_str.parse().expect("Failed to parse date");

    let limit_date: NaiveDate = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

    let conn = Connection::open("../../asbg-bot/src/asbg_bot/data-dev.db").unwrap();

    let mut answer: Option<i64> = Some(1);
    let mut reason: String = String::from("None");

    if play_date < limit_date {
        reason = String::from("L'invitation ne peut pas être acceptée car elle est demandée pour une date avant le 1er janvier 2025.");
        answer = Some(0);
    } else {
        let check_inivtation_date: i64 = play_date.signed_duration_since(date).num_days();
        let min_jour_avant_invitation: i64 = 1;
        if check_inivtation_date <= min_jour_avant_invitation {
            reason =format!(
                "L'invitation ne peut pas être acceptée car elle est faite moins de {} jours avant la date de jeu",
                min_jour_avant_invitation
            );
            answer = Some(0);
        } else {
            let evenements_path = "src/data/calendar_ics/Evenements.ics";
            let result_evenement = check_date_event_evenements(play_date, evenements_path);
            if result_evenement == false {
                reason = String::from("L'invitation ne peut pas être acceptée car elle est demandée un jour d'évenement interne.");
                answer = Some(0);
            } else {
                let ic_path = "src/data/calendar_ics/Interclubs.ics";
                let result_ic = check_date_event_ic(play_date, ic_path);
                if result_ic == false && play_date.weekday() == Weekday::Thu {
                    reason = String::from("L'invitation ne peut pas être acceptée car car elle est demandée un jour d'IC");
                    answer = Some(0);
                } else {
                    let (result_max_invitation, nb_invitations_already_accepted) =
                        check_max_invitations(guest_name.clone(), &conn, 3);
                    if result_max_invitation == false {
                        reason = format!(
                            "{} a déjà été invité {} fois",
                            guest_name, nb_invitations_already_accepted
                        );
                        answer = Some(0);
                    } else {
                        if play_date.weekday() == Weekday::Thu {
                            let (result_invitations_accepted, n_invitations_accepted) =
                                check_invitations_accepted(play_date_str, &conn, 1);
                            if result_invitations_accepted == false {
                                reason = format!(
                                    "L'invitation ne peut pas être acceptée car il y a déjà {} invitations pour le jeudi {}",
                                    n_invitations_accepted, play_date
                                );
                                answer = Some(0);
                            }
                        } else if play_date.weekday() == Weekday::Sun {
                            let (result_invitations_accepted, n_invitations_accepted) =
                                check_invitations_accepted(play_date_str, &conn, 2);
                            if result_invitations_accepted == false {
                                reason = format!(
                                    "L'invitation ne peut pas être acceptée car il y a déjà {} invitations pour le dimanche {}",
                                    n_invitations_accepted, play_date
                                );
                                answer = Some(0);
                            }
                        } else {
                            reason = String::from("L'invitation est demandée sur un autre jour qu'un créneau de jeu libre");
                            answer = Some(0);
                        }
                    }
                }
            }
        }
    }

    insert_invitation_request(
        &date_str,
        member,
        display_name,
        guest_name,
        play_date_str,
        answer.unwrap(),
        &reason,
        &conn,
    );
}

fn read_calendar(path: &str) -> IcalParser<BufReader<File>> {
    let buf = BufReader::new(File::open(&path).unwrap());

    let reader = ical::IcalParser::new(buf);

    return reader;
}

fn format_calendar(
    calendar_reader: IcalParser<BufReader<File>>,
) -> HashMap<String, HashMap<String, Option<String>>> {
    let mut all_events: HashMap<String, HashMap<String, Option<String>>> = HashMap::new();

    for calendar in calendar_reader {
        let mut event_n: i32 = 0;
        for event in calendar.unwrap().events {
            let mut map: HashMap<String, Option<String>> = HashMap::new();
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

fn convert_date(date_str: &str) -> Option<NaiveDate> {
    let mut date_event: Option<NaiveDate> = None;

    if date_str.len() == 8 {
        date_event =
            Some(NaiveDate::parse_from_str(date_str, "%Y%m%d").expect("Failed to parse date)"));
    } else {
        date_event = Some(
            NaiveDate::parse_from_str(date_str, "%Y%m%dT%H%M%SZ").expect("Failed to parse date)"),
        );
    }
    return date_event;
}

fn check_date_event_evenements(play_date: NaiveDate, path: &str) -> bool {
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
            let summary = properties.get("SUMMARY").unwrap();
            if let Some(summary) = &summary {
                let parts: Vec<&str> = summary.split("/").collect();
                if parts.get(0).unwrap().contains("ASBG") {
                    return false;
                }
            }
        }
    }
    return true;
}

fn check_max_invitations(
    guest_name: String,
    conn: &rusqlite::Connection,
    nb_invitations_max: usize,
) -> (bool, usize) {
    let mut stmt = conn
        .prepare(
            "SELECT count(guest_name) as count 
                        FROM invitations 
                        WHERE guest_name= ?1 AND answer = ?2",
        )
        .unwrap();
    let answer_accepted = "1";
    let mut past_invitations_accepted = stmt
        .query_map(params![&guest_name, answer_accepted], |row| {
            row.get::<_, i32>(0)
        })
        .unwrap();

    let nb_invitations_already_accepted = if let Some(result) = past_invitations_accepted.next() {
        result.unwrap() as usize
    } else {
        0 // Pas de résultats trouvés
    };

    let last_chance = nb_invitations_max - 1;

    if nb_invitations_already_accepted == last_chance {
        // println!("{guest_name} a déjà été invité {last_chance} fois");
        // println!(
        //     "Attention il s'agit de la dernière invitation possible pour cette personne: NEXT STEP"
        // );
        return (true, nb_invitations_already_accepted);
    } else if nb_invitations_already_accepted >= nb_invitations_max {
        return (false, nb_invitations_already_accepted);
    } else {
        // println!("{guest_name} n'a pas atteint son quota d'invitation de {nb_invitations_max}: NEXT STEP");
    }
    return (true, nb_invitations_already_accepted);
}

fn check_invitations_accepted(
    play_date_str: &str,
    conn: &rusqlite::Connection,
    nb_invitations: usize,
) -> (bool, usize) {
    let mut stmt = conn
        .prepare(
            "SELECT count(guest_name) as count 
                    FROM invitations 
                    WHERE play_date =?1 AND answer = ?2",
        )
        .unwrap();

    let answer_accepted = "1";
    let mut past_invitations_accepted = stmt
        .query_map(params![play_date_str, answer_accepted], |row| {
            row.get::<_, i32>(0)
        })
        .unwrap();

    let nb_invitations_already_accepted = if let Some(result) = past_invitations_accepted.next() {
        result.unwrap() as usize
    } else {
        0
    };
    if nb_invitations_already_accepted >= nb_invitations {
        return (false, nb_invitations_already_accepted);
    }

    return (true, nb_invitations_already_accepted);
}

fn insert_invitation_request(
    date: &str,
    member: &str,
    display_name: &str,
    guest_name: &str,
    play_date: &str,
    answer: i64,
    reason: &str,
    conn: &rusqlite::Connection,
) {
    let res = conn.execute(
        "INSERT INTO invitations (date, member, display_name, guest_name, play_date, answer, reason) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            date,
            member,
            display_name,
            guest_name,
            play_date,
            answer,
            reason,
        ],
    );
    match res {
        Ok(_) => true,
        Err(err) => {
            println!("Erreur d'insertion : {}", err);
            false
        }
    };
}
