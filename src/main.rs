use std::fs::File;
use std::fs::OpenOptions;

use std::io::BufReader;
use csv::Reader;
use csv::StringRecord;

use std::io::Write;


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


fn write_translation(country_code: &str, column_no: usize, value: &StringRecord) {
    if country_code.contains(" ") || value[0].contains(" ") {
        // println!("Rejecting {}", &value[0]);
        return;
    }
    //   println!("{}, {{ {:?}: {:?} }},", country_code, &value[0], &value[column_no]);
    let filename = generate_filename(country_code);
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

fn generate_filename(country_code: &str) -> String {
    return vec!["translate_", country_code, ".ts"].join("");
}

fn get_file_content() -> Reader<BufReader<File>> {
    let filename = "translations.json.csv";
    let file = File::open(filename).unwrap();
    let buf_reader = BufReader::new(file);
    let reader = Reader::from_reader(buf_reader);
    return reader;
}

fn create_interface_file(filename:&str) {
    println!("Creating {} ", filename);
    let data = "export type Translations = {\n"; // should be an object based on interface
    // Also need to create an interface file and a file to take a local and return
    // the correct interface.
    let mut data_file = File::create(filename).expect("Unable to create file");
   
    let map = "export const translationMap = new Map<string,Translations>();\n\n";
    data_file.write(&map.as_bytes()).expect("write failed");
    data_file.write(&data.as_bytes()).expect("write failed");

}


fn create_file(country_code: &str) {
    if country_code.contains(" ") {
        return;
    }
    let filename = generate_filename(country_code);
    println!("Creating {} ", filename);
    let data = vec!["export const translate",&country_code.to_ascii_uppercase(),":Translations = {\n"].join(""); 
    let mut data_file = File::create(filename).expect("Unable to create file");
    data_file.write(&data.as_bytes()).expect("write failed");
}

fn finish_file(country_code: &str) {
    if country_code.contains(" ") {
        return;
    }
    let filename = generate_filename(country_code);
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
    let mut reader = get_file_content();
    let mut header = StringRecord::new();

    // Get the headers
    for field in reader.headers() {
        header = field.to_owned();
    }

    // Create the files
    for country in header.iter().enumerate() {
        let country_code = country.1;
        create_file(country_code);
    }
    let interface_filename = "translationTypes.ts";
    create_interface_file(interface_filename);

    // Add the translations...
    let mut row_no = 0;
    for row in reader.records() {
        for value in row.iter().enumerate() {
            let mut column_no = 0;
            let key = value.1;
            add_to_interface_type(interface_filename, &key[0]);
            for country in header.iter().enumerate() {
                let country_code = country.1;
                write_translation(country_code, column_no, key);
                column_no = column_no + 1;
            }
        }
        row_no = row_no + 1;
    }

    // Finish the files
    for country in header.iter().enumerate() {
        let country_code = country.1;
        finish_file(country_code);
    }
    finish_interface_file(interface_filename);
}