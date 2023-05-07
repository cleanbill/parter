use std::path::Path;
use std::fs::File;
use std::io::Write;


pub fn create_service_file(dir: &str, filename: &str) {
    let path = Path::new(".");
    let path_filename = path.join(dir).join(filename).display().to_string();

    println!("Creating {} ", path_filename);
    let data =
        "
import { translateUK } from './translate_uk';
import { Translations } from './translationTypes';
import { Label } from './labelTypes';
import { Locales } from './localeTypes';

export const getTranslationsFromString = async (
    localString = 'UK',
): Promise<Translations> => {
    const ukKey = localString as keyof typeof Locales;
    if (!ukKey) {
    console.error('The (' + localString + ') is no more');
    return Promise.reject();
    }
    const locale = Locales[ukKey];
    if (!locale) {
    console.error(
        'The (' + localString + ') key (' + ukKey + ') brings back nothing',
    );
    return Promise.reject();
    }
    return await getTranslationsFromLocale(locale);
};

export const getTranslationsFromLocale = async (
    local: Locales,
): Promise<Translations> => {
    try {
    const translations = await import('./translate_' + local.toLowerCase());
    return translations == undefined ? translateUK : translations;
    } catch (error) {
    console.error('Cannot find ', './translate_' + local.toLowerCase(), error);
    return translateUK;
    }
};

export const getLocaleFromString = (
    localString = 'UK',
): Locales => {
    const ukKey = localString as keyof typeof Locales;
    if (!ukKey) {
    console.error('The (' + localString + ') is no more');
    return Locales.UK;
    }
    const locale = Locales[ukKey];
    if (!locale) {
    console.error(
        'The (' + localString + ') key (' + ukKey + ') brings back nothing'
    );
    return Locales.UK;
    }
    return locale;
};

export const populateLabelsFromString = async (
    localString = 'UK',
    labels: Partial<Translations>,
) => {
    const locale = getLocaleFromString(localString);
    return populateLabels(locale, labels);
};

export const populateLabels = async (
    local: Locales,
    labels: Partial<Translations>,
) => {
    const localeTranslation = await getTranslationsFromLocale(local);
    Object.keys(labels).forEach((label: string) => {
    const result = localeTranslation[label];
    const tranlastion = result == null ? translateUK[label] : result;

    labels[label] = tranlastion;
    });
    return labels;
};

const exampleClientUsage = async () => {
    type labelsPageType = Pick<
    Translations,
    Label.serviceOf | Label.serviceOrderFound | Label.serviceOrdersDate
    >;
    const labelsForThisPage: labelsPageType = {
    serviceOf: '',
    serviceOrderFound: '',
    serviceOrdersDate: ''
    };
    await populateLabelsFromString('NO',labelsForThisPage);
    console.log(labelsForThisPage.serviceOf,' etc...... ');
};                        
";
    let mut data_file = File::create(path_filename).expect("Unable to create file");
    data_file.write(&data.as_bytes()).expect("write failed");
}