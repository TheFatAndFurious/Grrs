#![allow(unused)]

use clap::Parser;
//use std::io::BufReader;
//use std::fs::File;

//recherche un pattern dans un fichier et affiche les lignes ou il est contenu
#[derive(Parser)]
//definit une nouvelle structure avec 2 champs pour recevoir des donnees
//PathBuf est similaire a une String mais pour les chemins d'acces aux fichiers
struct Cli {
    //le pattern recherche
    pattern: String,
    //le chemin d'acces du fichier dans lequel chercher
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();    
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
