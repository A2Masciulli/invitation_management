// use core::str;
use std::io;
// use calendar3::hyper::client;
// use calendar3::oauth2::storage;
use chrono::NaiveDate;
// use std::path::Path;

// use std::collections::HashMap;
use std::fs::File;
use rustc_serialize::json::Json;
use std::io::Read;

use oauth2::Client;
use oauth2::ClientId;
// use oauth2::{AuthorizationCode, CsrfToken, RedirectUrl};
// use oauth2::basic::{BasicClient, BasicTokenType};
// use oauth2::reqwest::http_client;
// use oauth2::AccessToken;
// use oauth2::reqwest::http_client;
use oauth2::ClientSecret;


fn main() {
    println!("Gestion des invitations");

    let (name, invitation_date, play_date) = read_invitation_information();
    
    //convert string to date
    let play_date: NaiveDate = play_date.parse().expect("Failed to parse date");
    let november_15_2024: NaiveDate = NaiveDate::from_ymd_opt(2024, 11, 15).unwrap();

    if play_date < november_15_2024 {
        println!("La date de la demande d'invitation est inférieure au 15 novembre 2024");
    } else {
        println!("La date de la demande d'invitation est supérieure au 15 novembre 2024");
    }

    // Read OAuth2 credentials from the JSON file
    let oauth_config = read_application_secret_from_file("./src/client_secret.json");

    
}

fn read_invitation_information() -> (String, String, String){

    println!("
    Entrez le nom et prénom de la personne qui invite 
    Mettre des majucules et un espace entre le nom et le prénom
    (exemple Masciulli Alizée):"
    );

    let mut name: String = String::new();

    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");

    println!("
    Entrez la date de la demande d'invitation 
    (exemple 2024-08-20):"
    );

    let mut invitation_date: String = String::new();

    io::stdin()
    .read_line(&mut invitation_date)
    .expect("Failed to read line");

    println!("
    Entrez la date pour laquelle l'invitation est demandée
    (exemple 2024-08-20):"
    );

    let mut play_date: String = String::new();
    io::stdin()
    .read_line(&mut play_date)
    .expect("Failed to read line");


    println!("L'invitation à été demandé le: {invitation_date} pour le: {play_date} par: {name}");

    return (name, invitation_date, play_date);

}

// Read the JSON file containing OAuth2 credentials
fn read_application_secret_from_file(path: &str) -> (String,String) {

    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json= Json::from_str(&data).unwrap();

    let client_id = json.find_path(&["client_id"]).unwrap().to_string();
    let client_secret = json.find_path(&["client_secret"]).unwrap().to_string();

    println!("{:?}",client_secret);
    return (client_id,client_secret)

}