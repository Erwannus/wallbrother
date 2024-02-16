use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let url = "156.67.224.173";
    let path = "/record/current.jpg";

    let mut stream = TcpStream::connect(format!("{}:80", url))
        .expect("Erreur, ne peut pas ce connecter au Host et/ou ne trouve pas le path");
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, url);

    match stream.write_all(request.as_bytes()) {
        Ok(_) => {
            println!("Requête envoyé : {}", url);
        }
        Err(e) => {
            eprintln!("Erreur lors de l'envoie de la requête sur : {}", e);
        }
    }

    // Lire la réponse du serveur
    let mut buffer = Vec::new();
    match stream.read_to_end(&mut buffer) {
        Ok(_) => {
            println!("Réponse len : {}", buffer.len());
            match std::fs::write("toto.jpg", &buffer) {
                Ok(_) => {
                    println!("Image créée.");
                }
                Err(e) => {
                    println!("Erreur lors de la création de l'image : {}", e);
                }
            }
        }
        Err(e) => {
            println!("Erreur lors de la lecture de la réponse : {}", e);
        }
    }

}
