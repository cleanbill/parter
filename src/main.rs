use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs::create_dir_all;

use std::io::BufReader;
use std::io::Write;
use std::path::Path;

use csv::Reader;
use csv::StringRecord;

fn add_to_interface_type(filename: &str, value: &str) {
    if value.contains(" "){
        return;
    }
    let data_file = OpenOptions::new()
        .append(true)
        .open(&filename)
        .or_else(|e| {
            println!("Error opening file {}: {}", filename, e);
            OpenOptions::new().create(true).open(&filename)
        });

    let data = vec!["\t\t", value, " : string,","\n"].join("");
    data_file.unwrap().write(data.as_bytes()).expect("write failed");
}

fn write_translation(dir: &str,country_code: &str, column_no: usize, value: &StringRecord) {
    if country_code.contains(" ") || value[0].contains(" ") {
        // println!("Rejecting {}", &value[0]);
        return;
    }
    //   println!("{}, {{ {:?}: {:?} }},", country_code, &value[0], &value[column_no]);
    let filename = generate_filename(dir, country_code);
    let data_file = OpenOptions::new()
        .append(true)
        .open(&filename)
        .or_else(|e| {
            println!("Error opening file {}: {}", filename, e);
            OpenOptions::new().create(true).open(&filename)
        });

    let data = vec!["\t\t\"", &value[0], "\" : \"", &value[column_no], "\",\n"].join("");
    data_file.unwrap().write(data.as_bytes()).expect("write failed");
}

fn generate_filename(dir: &str, country_code: &str) -> String {
    let path = Path::new(".");
    let filename = vec!["translate_", country_code, ".ts"].join("");
    return path.join(dir).join(filename).display().to_string();
}

fn get_file_content(filename: &String) -> Reader<BufReader<File>> {
    //let filename = "translations.json.csv";
    let file = File::open(filename).unwrap();
    let buf_reader = BufReader::new(file);
    let reader = Reader::from_reader(buf_reader);
    return reader;
}

fn create_interface_file(dir: &str, filename:&str) -> String {
    let path = Path::new(".");
    let path_filename = path.join(dir).join(filename).display().to_string();

    println!("Creating {} ", path_filename);
    let data = "export type Translations = {\n";
    let mut data_file = File::create(path_filename).expect("Unable to create file");
   
    let map = "export const translationMap = new Map<string,Translations>();\n\n";
    data_file.write(&map.as_bytes()).expect("write failed");
    data_file.write(&data.as_bytes()).expect("write failed");
    
    return path.join(dir).join(filename).display().to_string();
}

fn create_service_file(dir: &str, filename:&str) {
    let path = Path::new(".");
    let path_filename = path.join(dir).join(filename).display().to_string();

    println!("Creating {} ", path_filename);
    let data = "
import { translateUK } from \"./translate_UK\";
import { Translations } from \"./translationTypes\";

export const getTranslations = async (local:string): Promise<Translations> => {
    const translations = await import(\"./translate_\"+local.toUpperCase())
    return translations == undefined? translateUK: translations;
}";
    let mut data_file = File::create(path_filename).expect("Unable to create file");
    data_file.write(&data.as_bytes()).expect("write failed");
}

fn create_file(dir: &str, country_code: &str) {
    if country_code.contains(" ") {
        return;
    }
    let filename = generate_filename(dir, country_code);
    println!("Creating {} ", filename);
    let data = vec!["import { Translations } from \"./translationTypes\";\n\n","export const translate",&country_code.to_ascii_uppercase(),":Translations = {\n"].join(""); 
    let mut data_file = File::create(filename).expect("Unable to create file");
    data_file.write(&data.as_bytes()).expect("write failed");
}

fn finish_file(dir: &str, country_code: &str) {
    if country_code.contains(" ") {
        return;
    }
    let filename = generate_filename(dir, country_code);
    let data = "}\n";
    let mut data_file = OpenOptions::new().append(true).open(&filename).unwrap();
    data_file.write(&data.as_bytes()).expect("write failed");
}

fn finish_interface_file(filename: &str) {
    let data = "}\n";
    let mut data_file = OpenOptions::new().append(true).open(&filename).unwrap();
    data_file.write(&data.as_bytes()).expect("write failed");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reader = get_file_content(&args[1]);
    let mut header = StringRecord::new();

    // Get the headers
    for field in reader.headers() {
        header = field.to_owned();
    }

    let dir = "translations";
    create_dir_all(dir).expect("write failed");

    // Create the files
    for country in header.iter().enumerate() {
        let country_code = country.1;
        create_file(dir,country_code);
    }
    let interface_filename = create_interface_file(dir, "translationTypes.ts");
    create_service_file(dir, "translationService.ts");

    // Add the translations...
    let mut row_no = 0;
    for row in reader.records() {
        for value in row.iter().enumerate() {
            let mut column_no = 0;
            let key = value.1;
            add_to_interface_type(&interface_filename, &key[0]);
            for country in header.iter().enumerate() {
                let country_code = country.1;
                write_translation(dir, country_code, column_no, key);
                column_no = column_no + 1;
            }
        }
        row_no = row_no + 1;
    }

    // Finish the files
    for country in header.iter().enumerate() {
        let country_code = country.1;
        finish_file(dir, country_code);
    }
    finish_interface_file(&interface_filename);
}