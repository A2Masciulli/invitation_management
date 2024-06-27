use chrono::NaiveDate;
use std::io;

use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

extern crate google_calendar3 as calendar3;
use crate::hyper::client::HttpConnector;
use crate::hyper::Response;
use crate::hyper_rustls::HttpsConnector;
use crate::oauth2::authenticator::Authenticator;
use ::hyper::body::Body;

use calendar3::api::Channel;
use calendar3::{chrono, hyper, hyper_rustls, oauth2, CalendarHub, FieldMask};
use calendar3::{Error, Result};
use std::default::Default;

use futures::executor::block_on;

fn main() {
    println!("Gestion des invitations");

    // let (name, invitation_date, play_date) = read_invitation_information();

    // //convert string to date
    // let play_date: NaiveDate = play_date.parse().expect("Failed to parse date");
    // let november_15_2024: NaiveDate = NaiveDate::from_ymd_opt(2024, 11, 15).unwrap();

    // if play_date < november_15_2024 {
    //     println!("La date de la demande d'invitation est inférieure au 15 novembre 2024");
    // } else {
    //     println!("La date de la demande d'invitation est supérieure au 15 novembre 2024");
    // }

    // // Read OAuth2 credentials from the JSON file
    // let oauth_config = read_application_secret_from_file("./src/client_secret.json");

    let secret: oauth2::ApplicationSecret = Default::default();
    let auth = block_on(await_function(secret));

    let mut hub = CalendarHub::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        auth,
    );
    r = hub.events().get(...).doit().await
    // let mut re: std::prelude::v1::Result<(Response<dyn Body>, Channel), Error>q = Channel::default();
    // let result = block_on(await_function2(hub, req));


}

fn read_invitation_information() -> (String, String, String) {
    println!(
        "
    Entrez le nom et prénom de la personne qui invite 
    Mettre des majucules et un espace entre le nom et le prénom
    (exemple Masciulli Alizée):"
    );

    let mut name: String = String::new();

    io::stdin()
        .read_line(&mut name)
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

    println!("L'invitation à été demandé le: {invitation_date} pour le: {play_date} par: {name}");

    return (name, invitation_date, play_date);
}

// Read the JSON file containing OAuth2 credentials
fn read_application_secret_from_file(path: &str) -> (String, String) {
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();

    let client_id = json.find_path(&["client_id"]).unwrap().to_string();
    let client_secret = json.find_path(&["client_secret"]).unwrap().to_string();

    println!("{:?}", client_secret);
    return (client_id, client_secret);
}

async fn await_function(
    secret: oauth2::ApplicationSecret,
) -> Authenticator<HttpsConnector<HttpConnector>> {
    let _auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();

    return _auth;
}

async fn await_function2(
    hub: CalendarHub<HttpsConnector<HttpConnector>>, req: Channel
) -> Result<(Response<dyn Body>, Channel), Error>{

    let result = hub.events().watch(req, "calendarId").doit().await;
    
    return result;
}