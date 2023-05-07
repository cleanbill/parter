use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs::create_dir_all;

use std::io::BufReader;
use std::io::Write;
use std::path::Path;

use csv::Reader;
use csv::StringRecord;

pub mod static_code;

fn add_to_interface_type(filename: &str, value: &str) {
    if value.contains(" ") {
        return;
    }
    let data_file = OpenOptions::new()
        .append(true)
        .open(&filename)
        .or_else(|e| {
            println!("Error opening file {}: {}", filename, e);
            OpenOptions::new().create(true).open(&filename)
        });

    let data = vec!["\t\t", value, " : string,", "\n"].join("");
    data_file.unwrap().write(data.as_bytes()).expect("write failed");
}

fn add_to_label_type(filename: &str, value: &str) {
    if value.contains(" ") {
        return;
    }
    let data_file = OpenOptions::new()
        .append(true)
        .open(&filename)
        .or_else(|e| {
            println!("Error opening file {}: {}", filename, e);
            OpenOptions::new().create(true).open(&filename)
        });

    let data = vec!["\t\t  \"", value, "\" = \"", value, "\", \n"].join("");
    data_file.unwrap().write(data.as_bytes()).expect("write failed");
}

fn write_translation(dir: &str, country_code: &str, column_no: usize, value: &StringRecord) {
    if country_code.contains(" ") || value[0].contains(" ") {
        // println!("Rejecting {}", &value[0]);
        return;
    }
    // println!("{}, {{ {:?}: {:?} }},", country_code, &value[0], &value[column_no]);
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
    let filename = vec!["translate_", &country_code.to_lowercase(), ".ts"].join("");
    return path.join(dir).join(filename).display().to_string();
}

fn get_file_content(filename: &String) -> Reader<BufReader<File>> {
    //let filename = "translations.json.csv";
    let file = File::open(filename).unwrap();
    let buf_reader = BufReader::new(file);
    let reader = Reader::from_reader(buf_reader);
    return reader;
}

fn create_locale_type_file(dir: &str, filename: &str, locales: &StringRecord) {
    let path = Path::new(".");
    let path_filename = path.join(dir).join(filename).display().to_string();

    println!("Creating {} ", path_filename);
    let mut data_file = File::create(path_filename).expect("Unable to create file");

    let start_locale_type = "export enum Locales {\n";
    data_file.write(&start_locale_type.as_bytes()).expect("write failed");

    for country in locales.iter().enumerate() {
        let country_code = country.1;
        if !country_code.contains(" ") {
            let line = vec![
                "\t\t\"",
                &country_code.trim(),
                "\" = \"",
                &country_code.trim(),
                "\",\n"
            ].join("");
            data_file.write(&line.as_bytes()).expect("write failed");
        }
    }
    let end_locale_type = "}\n\n";
    data_file.write(&end_locale_type.as_bytes()).expect("write failed");
}

fn create_translation_type_file(dir: &str, filename: &str) -> String {
    let path = Path::new(".");
    let path_filename = path.join(dir).join(filename).display().to_string();

    println!("Creating {} ", path_filename);
    let mut data_file = File::create(path_filename).expect("Unable to create file");

    let start_translation_type = "export type Translations = {\n";
    data_file.write(&start_translation_type.as_bytes()).expect("write failed");

    return path.join(dir).join(filename).display().to_string();
}

fn create_label_type_file(dir: &str, filename: &str) -> String {
    let path = Path::new(".");
    let path_filename = path.join(dir).join(filename).display().to_string();

    println!("Creating {} ", path_filename);
    let mut data_file = File::create(path_filename).expect("Unable to create file");

    let start_translation_type = "export enum Label { \n";
    data_file.write(&start_translation_type.as_bytes()).expect("write failed");

    return path.join(dir).join(filename).display().to_string();
}

fn create_file(dir: &str, country_code: &str) {
    if country_code.contains(" ") {
        return;
    }
    let filename = generate_filename(dir, country_code);
    println!("Creating {} ", filename);
    let data = vec![
        "import { Translations } from \"./translationTypes\";\n\n",
        "export const translate",
        &country_code.to_ascii_uppercase(),
        ":Translations = {\n"
    ].join("");
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

fn finish_type_file(filename: &str) {
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
        create_file(dir, country_code);
    }

    create_locale_type_file(dir, "localeTypes.ts", &header);
    let translation_filename = create_translation_type_file(dir, "translationTypes.ts");
    let label_filename = create_label_type_file(dir, "labelTypes.ts");
    static_code::create_service_file(dir, "translationService.ts");

    // Add the translations...
    let mut row_no = 0;
    for row in reader.records() {
        // println!("{}",row.iter().len());
        for value in row.iter().enumerate() {
            let mut column_no = 0;
            let key = value.1;
            add_to_interface_type(&translation_filename, &key[0]);
            add_to_label_type(&label_filename, &key[0]);
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
    finish_type_file(&translation_filename);
    finish_type_file(&label_filename);
}