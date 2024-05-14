// use core::str;
use std::io;
// use calendar3::hyper::client;
// use calendar3::oauth2::storage;
use chrono::NaiveDate;
// use std::path::Path;

// use std::collections::HashMap;
// use serde_json::{Value};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;


// use oauth2::Client;
// use oauth2::ClientId;
// use oauth2::{AuthorizationCode, CsrfToken, RedirectUrl};
// use oauth2::basic::{BasicClient, BasicTokenType};
// use oauth2::reqwest::http_client;
// use oauth2::AccessToken;
// use oauth2::reqwest::http_client;
// use oauth2::ClientSecret;


struct OAuthConfig {
    client_id: String,
    project_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_secret: String,
    redirect_uris: Vec<String>,
}

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

    // // Define OAuth2 parameters
    // let client_id = ClientId::new(oauth_config.client_id);
    // let client_secret = ClientSecret::new(oauth_config.client_secret);
    // let auth_url = "https://accounts.google.com/o/oauth2/auth".to_string();
    // let token_url = "https://accounts.google.com/o/oauth2/token".to_string();
    // let redirect_url =
    //     RedirectUrl::new("http://localhost:8080/oauth2/callback".to_string()).unwrap();

    // // Set up client
    // let client = Client::new();

    // // Construct OAuth2 authorization URL
    // let (authorize_url, csrf_state) =
    //     AuthorizationCode::new(&auth_url, &client_id, &redirect_url).add_scope(
    //         "https://www.googleapis.com/auth/calendar",
    //     ).url();

    // // Print URL and have user visit it to obtain authorization code

    // // Once user is redirected back with authorization code
    // let code = AuthorizationCode::new("AUTHORIZATION_CODE".to_string());
    // let token_result = code
    //     .exchange_token(&client, &token_url, &client_id, &client_secret)
    //     .unwrap();

    // // Use the access token to make requests to the Calendar API
    // let access_token = token_result.access_token().secret();
    // let response = client
    //     .get("https://www.googleapis.com/calendar/v3/calendars/calendarId/events")
    //     .bearer_auth(access_token)
    //     .send()
    //     .await
    //     .unwrap();

    // // Charger les informations d'authentification depuis le fichier JSON téléchargé depuis la Console Google Cloud
    // let path: &Path = Path::new("./src/client_secret.json");
    // let result = read_application_secret(path);
    // // Extract the secret or handle the error
    // let secret = match result {
    //     Ok(secret) => secret,
    //     Err(err) => {
    //         // Handle the error
    //         eprintln!("Error: {}", err);
    //         // Return or panic, depending on your use case
    //         return;
    //     }
    // };
    // // Define OAuth2 parameters
    // let client_id = ClientId::new(secret.client_id);

    // println!("Secret: {:?}", client_id);

    // let auth = Authenticator::new(
    //     &secret, 
    //     DefaultAuthenticatorDelegate,
    //     client::Client::new(),
    //     yup_oauth2::MemoryStorage::new(),
    //     flow_type::InstalledFlow,

    // );

    // // Créer une instance du hub Calendar
    // let hub = CalendarHub::new(auth);

    // // Définir la date spécifique pour laquelle vous voulez récupérer les événements
    // let date = "2024-05-15";

    // // Récupérer la liste des événements de votre agenda pour la date spécifique
    // let result = hub.events().list("primary")
    //     .time_min(&format!("{}T00:00:00Z", date))
    //     .time_max(&format!("{}T23:59:59Z", date))
    //     .doit();

    // match result {
    //     Err(e) => println!("Erreur lors de la récupération des événements : {}", e),
    //     Ok(response) => {
    //         println!("Liste des événements pour le {} :", date);
    //         for event in response.items.unwrap_or_default() {
    //             println!("{} - {}", event.summary.unwrap_or_default(), event.start.unwrap_or_default().date_time.unwrap_or_default());
    //         }
    //     }
    // }

    // Ok(())
    // println!("Erreur lors de la récupération des événements : {}", result)
    
    
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
fn read_application_secret_from_file(path: &str) -> OAuthConfig {

    let file = File::open(&path);
    let reader = BufReader::new(file);

    // Analyser le JSON en tant que Value
    let json_value: OAuthConfig = serde_json::from_reader(reader);

    // // Convertir le Value en HashMap<String, Value>
    // let map = json_value.as_object().unwrap();

    // // Créer une HashMap<String, String> à partir de la HashMap<String, Value>
    // let mut string_map = HashMap::new();
    // for (key, value) in map {
    //     string_map.insert(key.clone(), value.to_string());
    // }

    // Afficher le dictionnaire
    println!("{:?}", json_value);
    return json_value

}