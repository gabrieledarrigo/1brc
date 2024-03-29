use std::{
    fs::File,
    io::{BufWriter, Write},
    time::Instant,
};

use rand::Rng;

use crate::weather_station::WeatherStation;

const MEASUREMENT_FILE: &str = "./measurements.txt";

fn get_stations() -> Vec<WeatherStation> {
    vec![
        WeatherStation::new(String::from("Abha"), 18.0),
        WeatherStation::new(String::from("Abidjan"), 26.0),
        WeatherStation::new(String::from("Abéché"), 29.4),
        WeatherStation::new(String::from("Accra"), 26.4),
        WeatherStation::new(String::from("Addis Ababa"), 16.0),
        WeatherStation::new(String::from("Adelaide"), 17.3),
        WeatherStation::new(String::from("Aden"), 29.1),
        WeatherStation::new(String::from("Ahvaz"), 25.4),
        WeatherStation::new(String::from("Albuquerque"), 14.0),
        WeatherStation::new(String::from("Alexandra"), 11.0),
        WeatherStation::new(String::from("Alexandria"), 20.0),
        WeatherStation::new(String::from("Algiers"), 18.2),
        WeatherStation::new(String::from("Alice Springs"), 21.0),
        WeatherStation::new(String::from("Almaty"), 10.0),
        WeatherStation::new(String::from("Amsterdam"), 10.2),
        WeatherStation::new(String::from("Anadyr"), -6.9),
        WeatherStation::new(String::from("Anchorage"), 2.8),
        WeatherStation::new(String::from("Andorra la Vella"), 9.8),
        WeatherStation::new(String::from("Ankara"), 12.0),
        WeatherStation::new(String::from("Antananarivo"), 17.9),
        WeatherStation::new(String::from("Antsiranana"), 25.2),
        WeatherStation::new(String::from("Arkhangelsk"), 1.3),
        WeatherStation::new(String::from("Ashgabat"), 17.1),
        WeatherStation::new(String::from("Asmara"), 15.6),
        WeatherStation::new(String::from("Assab"), 30.5),
        WeatherStation::new(String::from("Astana"), 3.5),
        WeatherStation::new(String::from("Athens"), 19.2),
        WeatherStation::new(String::from("Atlanta"), 17.0),
        WeatherStation::new(String::from("Auckland"), 15.2),
        WeatherStation::new(String::from("Austin"), 20.7),
        WeatherStation::new(String::from("Baghdad"), 22.77),
        WeatherStation::new(String::from("Baguio"), 19.5),
        WeatherStation::new(String::from("Baku"), 15.1),
        WeatherStation::new(String::from("Baltimore"), 13.1),
        WeatherStation::new(String::from("Bamako"), 27.8),
        WeatherStation::new(String::from("Bangkok"), 28.6),
        WeatherStation::new(String::from("Bangui"), 26.0),
        WeatherStation::new(String::from("Banjul"), 26.0),
        WeatherStation::new(String::from("Barcelona"), 18.2),
        WeatherStation::new(String::from("Bata"), 25.1),
        WeatherStation::new(String::from("Batumi"), 14.0),
        WeatherStation::new(String::from("Beijing"), 12.9),
        WeatherStation::new(String::from("Beirut"), 20.9),
        WeatherStation::new(String::from("Belgrade"), 12.5),
        WeatherStation::new(String::from("Belize City"), 26.7),
        WeatherStation::new(String::from("Benghazi"), 19.9),
        WeatherStation::new(String::from("Bergen"), 7.7),
        WeatherStation::new(String::from("Berlin"), 10.3),
        WeatherStation::new(String::from("Bilbao"), 14.7),
        WeatherStation::new(String::from("Birao"), 26.5),
        WeatherStation::new(String::from("Bishkek"), 11.3),
        WeatherStation::new(String::from("Bissau"), 27.0),
        WeatherStation::new(String::from("Blantyre"), 22.2),
        WeatherStation::new(String::from("Bloemfontein"), 15.6),
        WeatherStation::new(String::from("Boise"), 11.4),
        WeatherStation::new(String::from("Bordeaux"), 14.2),
        WeatherStation::new(String::from("Bosaso"), 30.0),
        WeatherStation::new(String::from("Boston"), 10.9),
        WeatherStation::new(String::from("Bouaké"), 26.0),
        WeatherStation::new(String::from("Bratislava"), 10.5),
        WeatherStation::new(String::from("Brazzaville"), 25.0),
        WeatherStation::new(String::from("Bridgetown"), 27.0),
        WeatherStation::new(String::from("Brisbane"), 21.4),
        WeatherStation::new(String::from("Brussels"), 10.5),
        WeatherStation::new(String::from("Bucharest"), 10.8),
        WeatherStation::new(String::from("Budapest"), 11.3),
        WeatherStation::new(String::from("Bujumbura"), 23.8),
        WeatherStation::new(String::from("Bulawayo"), 18.9),
        WeatherStation::new(String::from("Burnie"), 13.1),
        WeatherStation::new(String::from("Busan"), 15.0),
        WeatherStation::new(String::from("Cabo San Lucas"), 23.9),
        WeatherStation::new(String::from("Cairns"), 25.0),
        WeatherStation::new(String::from("Cairo"), 21.4),
        WeatherStation::new(String::from("Calgary"), 4.4),
        WeatherStation::new(String::from("Canberra"), 13.1),
        WeatherStation::new(String::from("Cape Town"), 16.2),
        WeatherStation::new(String::from("Changsha"), 17.4),
        WeatherStation::new(String::from("Charlotte"), 16.1),
        WeatherStation::new(String::from("Chiang Mai"), 25.8),
        WeatherStation::new(String::from("Chicago"), 9.8),
        WeatherStation::new(String::from("Chihuahua"), 18.6),
        WeatherStation::new(String::from("Chișinău"), 10.2),
        WeatherStation::new(String::from("Chittagong"), 25.9),
        WeatherStation::new(String::from("Chongqing"), 18.6),
        WeatherStation::new(String::from("Christchurch"), 12.2),
        WeatherStation::new(String::from("City of San Marino"), 11.8),
        WeatherStation::new(String::from("Colombo"), 27.4),
        WeatherStation::new(String::from("Columbus"), 11.7),
        WeatherStation::new(String::from("Conakry"), 26.4),
        WeatherStation::new(String::from("Copenhagen"), 9.1),
        WeatherStation::new(String::from("Cotonou"), 27.2),
        WeatherStation::new(String::from("Cracow"), 9.3),
        WeatherStation::new(String::from("Da Lat"), 17.9),
        WeatherStation::new(String::from("Da Nang"), 25.8),
        WeatherStation::new(String::from("Dakar"), 24.0),
        WeatherStation::new(String::from("Dallas"), 19.0),
        WeatherStation::new(String::from("Damascus"), 17.0),
        WeatherStation::new(String::from("Dampier"), 26.4),
        WeatherStation::new(String::from("Dar es Salaam"), 25.8),
        WeatherStation::new(String::from("Darwin"), 27.6),
        WeatherStation::new(String::from("Denpasar"), 23.7),
        WeatherStation::new(String::from("Denver"), 10.4),
        WeatherStation::new(String::from("Detroit"), 10.0),
        WeatherStation::new(String::from("Dhaka"), 25.9),
        WeatherStation::new(String::from("Dikson"), -11.1),
        WeatherStation::new(String::from("Dili"), 26.6),
        WeatherStation::new(String::from("Djibouti"), 29.9),
        WeatherStation::new(String::from("Dodoma"), 22.7),
        WeatherStation::new(String::from("Dolisie"), 24.0),
        WeatherStation::new(String::from("Douala"), 26.7),
        WeatherStation::new(String::from("Dubai"), 26.9),
        WeatherStation::new(String::from("Dublin"), 9.8),
        WeatherStation::new(String::from("Dunedin"), 11.1),
        WeatherStation::new(String::from("Durban"), 20.6),
        WeatherStation::new(String::from("Dushanbe"), 14.7),
        WeatherStation::new(String::from("Edinburgh"), 9.3),
        WeatherStation::new(String::from("Edmonton"), 4.2),
        WeatherStation::new(String::from("El Paso"), 18.1),
        WeatherStation::new(String::from("Entebbe"), 21.0),
        WeatherStation::new(String::from("Erbil"), 19.5),
        WeatherStation::new(String::from("Erzurum"), 5.1),
        WeatherStation::new(String::from("Fairbanks"), -2.3),
        WeatherStation::new(String::from("Fianarantsoa"), 17.9),
        WeatherStation::new(String::from("Flores,  Petén"), 26.4),
        WeatherStation::new(String::from("Frankfurt"), 10.6),
        WeatherStation::new(String::from("Fresno"), 17.9),
        WeatherStation::new(String::from("Fukuoka"), 17.0),
        WeatherStation::new(String::from("Gabès"), 19.5),
        WeatherStation::new(String::from("Gaborone"), 21.0),
        WeatherStation::new(String::from("Gagnoa"), 26.0),
        WeatherStation::new(String::from("Gangtok"), 15.2),
        WeatherStation::new(String::from("Garissa"), 29.3),
        WeatherStation::new(String::from("Garoua"), 28.3),
        WeatherStation::new(String::from("George Town"), 27.9),
        WeatherStation::new(String::from("Ghanzi"), 21.4),
        WeatherStation::new(String::from("Gjoa Haven"), -14.4),
        WeatherStation::new(String::from("Guadalajara"), 20.9),
        WeatherStation::new(String::from("Guangzhou"), 22.4),
        WeatherStation::new(String::from("Guatemala City"), 20.4),
        WeatherStation::new(String::from("Halifax"), 7.5),
        WeatherStation::new(String::from("Hamburg"), 9.7),
        WeatherStation::new(String::from("Hamilton"), 13.8),
        WeatherStation::new(String::from("Hanga Roa"), 20.5),
        WeatherStation::new(String::from("Hanoi"), 23.6),
        WeatherStation::new(String::from("Harare"), 18.4),
        WeatherStation::new(String::from("Harbin"), 5.0),
        WeatherStation::new(String::from("Hargeisa"), 21.7),
        WeatherStation::new(String::from("Hat Yai"), 27.0),
        WeatherStation::new(String::from("Havana"), 25.2),
        WeatherStation::new(String::from("Helsinki"), 5.9),
        WeatherStation::new(String::from("Heraklion"), 18.9),
        WeatherStation::new(String::from("Hiroshima"), 16.3),
        WeatherStation::new(String::from("Ho Chi Minh City"), 27.4),
        WeatherStation::new(String::from("Hobart"), 12.7),
        WeatherStation::new(String::from("Hong Kong"), 23.3),
        WeatherStation::new(String::from("Honiara"), 26.5),
        WeatherStation::new(String::from("Honolulu"), 25.4),
        WeatherStation::new(String::from("Houston"), 20.8),
        WeatherStation::new(String::from("Ifrane"), 11.4),
        WeatherStation::new(String::from("Indianapolis"), 11.8),
        WeatherStation::new(String::from("Iqaluit"), -9.3),
        WeatherStation::new(String::from("Irkutsk"), 1.0),
        WeatherStation::new(String::from("Istanbul"), 13.9),
        WeatherStation::new(String::from("İzmir"), 17.9),
        WeatherStation::new(String::from("Jacksonville"), 20.3),
        WeatherStation::new(String::from("Jakarta"), 26.7),
        WeatherStation::new(String::from("Jayapura"), 27.0),
        WeatherStation::new(String::from("Jerusalem"), 18.3),
        WeatherStation::new(String::from("Johannesburg"), 15.5),
        WeatherStation::new(String::from("Jos"), 22.8),
        WeatherStation::new(String::from("Juba"), 27.8),
        WeatherStation::new(String::from("Kabul"), 12.1),
        WeatherStation::new(String::from("Kampala"), 20.0),
        WeatherStation::new(String::from("Kandi"), 27.7),
        WeatherStation::new(String::from("Kankan"), 26.5),
        WeatherStation::new(String::from("Kano"), 26.4),
        WeatherStation::new(String::from("Kansas City"), 12.5),
        WeatherStation::new(String::from("Karachi"), 26.0),
        WeatherStation::new(String::from("Karonga"), 24.4),
        WeatherStation::new(String::from("Kathmandu"), 18.3),
        WeatherStation::new(String::from("Khartoum"), 29.9),
        WeatherStation::new(String::from("Kingston"), 27.4),
        WeatherStation::new(String::from("Kinshasa"), 25.3),
        WeatherStation::new(String::from("Kolkata"), 26.7),
        WeatherStation::new(String::from("Kuala Lumpur"), 27.3),
        WeatherStation::new(String::from("Kumasi"), 26.0),
        WeatherStation::new(String::from("Kunming"), 15.7),
        WeatherStation::new(String::from("Kuopio"), 3.4),
        WeatherStation::new(String::from("Kuwait City"), 25.7),
        WeatherStation::new(String::from("Kyiv"), 8.4),
        WeatherStation::new(String::from("Kyoto"), 15.8),
        WeatherStation::new(String::from("La Ceiba"), 26.2),
        WeatherStation::new(String::from("La Paz"), 23.7),
        WeatherStation::new(String::from("Lagos"), 26.8),
        WeatherStation::new(String::from("Lahore"), 24.3),
        WeatherStation::new(String::from("Lake Havasu City"), 23.7),
        WeatherStation::new(String::from("Lake Tekapo"), 8.7),
        WeatherStation::new(String::from("Las Palmas de Gran Canaria"), 21.2),
        WeatherStation::new(String::from("Las Vegas"), 20.3),
        WeatherStation::new(String::from("Launceston"), 13.1),
        WeatherStation::new(String::from("Lhasa"), 7.6),
        WeatherStation::new(String::from("Libreville"), 25.9),
        WeatherStation::new(String::from("Lisbon"), 17.5),
        WeatherStation::new(String::from("Livingstone"), 21.8),
        WeatherStation::new(String::from("Ljubljana"), 10.9),
        WeatherStation::new(String::from("Lodwar"), 29.3),
        WeatherStation::new(String::from("Lomé"), 26.9),
        WeatherStation::new(String::from("London"), 11.3),
        WeatherStation::new(String::from("Los Angeles"), 18.6),
        WeatherStation::new(String::from("Louisville"), 13.9),
        WeatherStation::new(String::from("Luanda"), 25.8),
        WeatherStation::new(String::from("Lubumbashi"), 20.8),
        WeatherStation::new(String::from("Lusaka"), 19.9),
        WeatherStation::new(String::from("Luxembourg City"), 9.3),
        WeatherStation::new(String::from("Lviv"), 7.8),
        WeatherStation::new(String::from("Lyon"), 12.5),
        WeatherStation::new(String::from("Madrid"), 15.0),
        WeatherStation::new(String::from("Mahajanga"), 26.3),
        WeatherStation::new(String::from("Makassar"), 26.7),
        WeatherStation::new(String::from("Makurdi"), 26.0),
        WeatherStation::new(String::from("Malabo"), 26.3),
        WeatherStation::new(String::from("Malé"), 28.0),
        WeatherStation::new(String::from("Managua"), 27.3),
        WeatherStation::new(String::from("Manama"), 26.5),
        WeatherStation::new(String::from("Mandalay"), 28.0),
        WeatherStation::new(String::from("Mango"), 28.1),
        WeatherStation::new(String::from("Manila"), 28.4),
        WeatherStation::new(String::from("Maputo"), 22.8),
        WeatherStation::new(String::from("Marrakesh"), 19.6),
        WeatherStation::new(String::from("Marseille"), 15.8),
        WeatherStation::new(String::from("Maun"), 22.4),
        WeatherStation::new(String::from("Medan"), 26.5),
        WeatherStation::new(String::from("Mek'ele"), 22.7),
        WeatherStation::new(String::from("Melbourne"), 15.1),
        WeatherStation::new(String::from("Memphis"), 17.2),
        WeatherStation::new(String::from("Mexicali"), 23.1),
        WeatherStation::new(String::from("Mexico City"), 17.5),
        WeatherStation::new(String::from("Miami"), 24.9),
        WeatherStation::new(String::from("Milan"), 13.0),
        WeatherStation::new(String::from("Milwaukee"), 8.9),
        WeatherStation::new(String::from("Minneapolis"), 7.8),
        WeatherStation::new(String::from("Minsk"), 6.7),
        WeatherStation::new(String::from("Mogadishu"), 27.1),
        WeatherStation::new(String::from("Mombasa"), 26.3),
        WeatherStation::new(String::from("Monaco"), 16.4),
        WeatherStation::new(String::from("Moncton"), 6.1),
        WeatherStation::new(String::from("Monterrey"), 22.3),
        WeatherStation::new(String::from("Montreal"), 6.8),
        WeatherStation::new(String::from("Moscow"), 5.8),
        WeatherStation::new(String::from("Mumbai"), 27.1),
        WeatherStation::new(String::from("Murmansk"), 0.6),
        WeatherStation::new(String::from("Muscat"), 28.0),
        WeatherStation::new(String::from("Mzuzu"), 17.7),
        WeatherStation::new(String::from("N'Djamena"), 28.3),
        WeatherStation::new(String::from("Naha"), 23.1),
        WeatherStation::new(String::from("Nairobi"), 17.8),
        WeatherStation::new(String::from("Nakhon Ratchasima"), 27.3),
        WeatherStation::new(String::from("Napier"), 14.6),
        WeatherStation::new(String::from("Napoli"), 15.9),
        WeatherStation::new(String::from("Nashville"), 15.4),
        WeatherStation::new(String::from("Nassau"), 24.6),
        WeatherStation::new(String::from("Ndola"), 20.3),
        WeatherStation::new(String::from("New Delhi"), 25.0),
        WeatherStation::new(String::from("New Orleans"), 20.7),
        WeatherStation::new(String::from("New York City"), 12.9),
        WeatherStation::new(String::from("Ngaoundéré"), 22.0),
        WeatherStation::new(String::from("Niamey"), 29.3),
        WeatherStation::new(String::from("Nicosia"), 19.7),
        WeatherStation::new(String::from("Niigata"), 13.9),
        WeatherStation::new(String::from("Nouadhibou"), 21.3),
        WeatherStation::new(String::from("Nouakchott"), 25.7),
        WeatherStation::new(String::from("Novosibirsk"), 1.7),
        WeatherStation::new(String::from("Nuuk"), -1.4),
        WeatherStation::new(String::from("Odesa"), 10.7),
        WeatherStation::new(String::from("Odienné"), 26.0),
        WeatherStation::new(String::from("Oklahoma City"), 15.9),
        WeatherStation::new(String::from("Omaha"), 10.6),
        WeatherStation::new(String::from("Oranjestad"), 28.1),
        WeatherStation::new(String::from("Oslo"), 5.7),
        WeatherStation::new(String::from("Ottawa"), 6.6),
        WeatherStation::new(String::from("Ouagadougou"), 28.3),
        WeatherStation::new(String::from("Ouahigouya"), 28.6),
        WeatherStation::new(String::from("Ouarzazate"), 18.9),
        WeatherStation::new(String::from("Oulu"), 2.7),
        WeatherStation::new(String::from("Palembang"), 27.3),
        WeatherStation::new(String::from("Palermo"), 18.5),
        WeatherStation::new(String::from("Palm Springs"), 24.5),
        WeatherStation::new(String::from("Palmerston North"), 13.2),
        WeatherStation::new(String::from("Panama City"), 28.0),
        WeatherStation::new(String::from("Parakou"), 26.8),
        WeatherStation::new(String::from("Paris"), 12.3),
        WeatherStation::new(String::from("Perth"), 18.7),
        WeatherStation::new(String::from("Petropavlovsk-Kamchatsky"), 1.9),
        WeatherStation::new(String::from("Philadelphia"), 13.2),
        WeatherStation::new(String::from("Phnom Penh"), 28.3),
        WeatherStation::new(String::from("Phoenix"), 23.9),
        WeatherStation::new(String::from("Pittsburgh"), 10.8),
        WeatherStation::new(String::from("Podgorica"), 15.3),
        WeatherStation::new(String::from("Pointe-Noire"), 26.1),
        WeatherStation::new(String::from("Pontianak"), 27.7),
        WeatherStation::new(String::from("Port Moresby"), 26.9),
        WeatherStation::new(String::from("Port Sudan"), 28.4),
        WeatherStation::new(String::from("Port Vila"), 24.3),
        WeatherStation::new(String::from("Port-Gentil"), 26.0),
        WeatherStation::new(String::from("Portland (OR)"), 12.4),
        WeatherStation::new(String::from("Porto"), 15.7),
        WeatherStation::new(String::from("Prague"), 8.4),
        WeatherStation::new(String::from("Praia"), 24.4),
        WeatherStation::new(String::from("Pretoria"), 18.2),
        WeatherStation::new(String::from("Pyongyang"), 10.8),
        WeatherStation::new(String::from("Rabat"), 17.2),
        WeatherStation::new(String::from("Rangpur"), 24.4),
        WeatherStation::new(String::from("Reggane"), 28.3),
        WeatherStation::new(String::from("Reykjavík"), 4.3),
        WeatherStation::new(String::from("Riga"), 6.2),
        WeatherStation::new(String::from("Riyadh"), 26.0),
        WeatherStation::new(String::from("Rome"), 15.2),
        WeatherStation::new(String::from("Roseau"), 26.2),
        WeatherStation::new(String::from("Rostov-on-Don"), 9.9),
        WeatherStation::new(String::from("Sacramento"), 16.3),
        WeatherStation::new(String::from("Saint Petersburg"), 5.8),
        WeatherStation::new(String::from("Saint-Pierre"), 5.7),
        WeatherStation::new(String::from("Salt Lake City"), 11.6),
        WeatherStation::new(String::from("San Antonio"), 20.8),
        WeatherStation::new(String::from("San Diego"), 17.8),
        WeatherStation::new(String::from("San Francisco"), 14.6),
        WeatherStation::new(String::from("San Jose"), 16.4),
        WeatherStation::new(String::from("San José"), 22.6),
        WeatherStation::new(String::from("San Juan"), 27.2),
        WeatherStation::new(String::from("San Salvador"), 23.1),
        WeatherStation::new(String::from("Sana'a"), 20.0),
        WeatherStation::new(String::from("Santo Domingo"), 25.9),
        WeatherStation::new(String::from("Sapporo"), 8.9),
        WeatherStation::new(String::from("Sarajevo"), 10.1),
        WeatherStation::new(String::from("Saskatoon"), 3.3),
        WeatherStation::new(String::from("Seattle"), 11.3),
        WeatherStation::new(String::from("Ségou"), 28.0),
        WeatherStation::new(String::from("Seoul"), 12.5),
        WeatherStation::new(String::from("Seville"), 19.2),
        WeatherStation::new(String::from("Shanghai"), 16.7),
        WeatherStation::new(String::from("Singapore"), 27.0),
        WeatherStation::new(String::from("Skopje"), 12.4),
        WeatherStation::new(String::from("Sochi"), 14.2),
        WeatherStation::new(String::from("Sofia"), 10.6),
        WeatherStation::new(String::from("Sokoto"), 28.0),
        WeatherStation::new(String::from("Split"), 16.1),
        WeatherStation::new(String::from("St. John's"), 5.0),
        WeatherStation::new(String::from("St. Louis"), 13.9),
        WeatherStation::new(String::from("Stockholm"), 6.6),
        WeatherStation::new(String::from("Surabaya"), 27.1),
        WeatherStation::new(String::from("Suva"), 25.6),
        WeatherStation::new(String::from("Suwałki"), 7.2),
        WeatherStation::new(String::from("Sydney"), 17.7),
        WeatherStation::new(String::from("Tabora"), 23.0),
        WeatherStation::new(String::from("Tabriz"), 12.6),
        WeatherStation::new(String::from("Taipei"), 23.0),
        WeatherStation::new(String::from("Tallinn"), 6.4),
        WeatherStation::new(String::from("Tamale"), 27.9),
        WeatherStation::new(String::from("Tamanrasset"), 21.7),
        WeatherStation::new(String::from("Tampa"), 22.9),
        WeatherStation::new(String::from("Tashkent"), 14.8),
        WeatherStation::new(String::from("Tauranga"), 14.8),
        WeatherStation::new(String::from("Tbilisi"), 12.9),
        WeatherStation::new(String::from("Tegucigalpa"), 21.7),
        WeatherStation::new(String::from("Tehran"), 17.0),
        WeatherStation::new(String::from("Tel Aviv"), 20.0),
        WeatherStation::new(String::from("Thessaloniki"), 16.0),
        WeatherStation::new(String::from("Thiès"), 24.0),
        WeatherStation::new(String::from("Tijuana"), 17.8),
        WeatherStation::new(String::from("Timbuktu"), 28.0),
        WeatherStation::new(String::from("Tirana"), 15.2),
        WeatherStation::new(String::from("Toamasina"), 23.4),
        WeatherStation::new(String::from("Tokyo"), 15.4),
        WeatherStation::new(String::from("Toliara"), 24.1),
        WeatherStation::new(String::from("Toluca"), 12.4),
        WeatherStation::new(String::from("Toronto"), 9.4),
        WeatherStation::new(String::from("Tripoli"), 20.0),
        WeatherStation::new(String::from("Tromsø"), 2.9),
        WeatherStation::new(String::from("Tucson"), 20.9),
        WeatherStation::new(String::from("Tunis"), 18.4),
        WeatherStation::new(String::from("Ulaanbaatar"), -0.4),
        WeatherStation::new(String::from("Upington"), 20.4),
        WeatherStation::new(String::from("Ürümqi"), 7.4),
        WeatherStation::new(String::from("Vaduz"), 10.1),
        WeatherStation::new(String::from("Valencia"), 18.3),
        WeatherStation::new(String::from("Valletta"), 18.8),
        WeatherStation::new(String::from("Vancouver"), 10.4),
        WeatherStation::new(String::from("Veracruz"), 25.4),
        WeatherStation::new(String::from("Vienna"), 10.4),
        WeatherStation::new(String::from("Vientiane"), 25.9),
        WeatherStation::new(String::from("Villahermosa"), 27.1),
        WeatherStation::new(String::from("Vilnius"), 6.0),
        WeatherStation::new(String::from("Virginia Beach"), 15.8),
        WeatherStation::new(String::from("Vladivostok"), 4.9),
        WeatherStation::new(String::from("Warsaw"), 8.5),
        WeatherStation::new(String::from("Washington, D.C."), 14.6),
        WeatherStation::new(String::from("Wau"), 27.8),
        WeatherStation::new(String::from("Wellington"), 12.9),
        WeatherStation::new(String::from("Whitehorse"), -0.1),
        WeatherStation::new(String::from("Wichita"), 13.9),
        WeatherStation::new(String::from("Willemstad"), 28.0),
        WeatherStation::new(String::from("Winnipeg"), 3.0),
        WeatherStation::new(String::from("Wrocław"), 9.6),
        WeatherStation::new(String::from("Xi'an"), 14.1),
        WeatherStation::new(String::from("Yakutsk"), -8.8),
        WeatherStation::new(String::from("Yangon"), 27.5),
        WeatherStation::new(String::from("Yaoundé"), 23.8),
        WeatherStation::new(String::from("Yellowknife"), -4.3),
        WeatherStation::new(String::from("Yerevan"), 12.4),
        WeatherStation::new(String::from("Yinchuan"), 9.0),
        WeatherStation::new(String::from("Zagreb"), 10.7),
        WeatherStation::new(String::from("Zanzibar City"), 26.0),
        WeatherStation::new(String::from("Zürich"), 9.3),
    ]
}

/// Creates a specified number of weather measurements and writes them to a file.
///
/// # Arguments
///
/// * `number_of_measurements` - The total number of measurements to generate.
///
/// # Panics
///
/// Panics if it fails to create or write to the measurement file.
///
/// # Examples
///
/// ```
/// // Generate and write 100 measurements to the default file.
/// create_measurements(100);
/// ```
pub fn create_measurements(number_of_measurements: i32) {
    let start = Instant::now();

    println!(
        "Creating {} measurements into {}",
        number_of_measurements, MEASUREMENT_FILE
    );

    let mut rng = rand::thread_rng();
    let stations = get_stations();

    let file = File::create(MEASUREMENT_FILE).expect("Cannot create file");
    let mut buffer = BufWriter::new(file);

    for i in 0..number_of_measurements {
        if i > 0 && i % 50_000_000 == 0 {
            println!(
                "Wrote {} measurements after {} seconds",
                i,
                start.elapsed().as_secs()
            );
        }

        let index = rng.gen_range(0..stations.len());
        let station = &stations[index];
        let line = format!("{};{}\n", station.id(), station.measurement());

        buffer
            .write(line.as_bytes())
            .expect("Cannot write to file!");
    }

    println!(
        "{} created with {} measurementes",
        MEASUREMENT_FILE, number_of_measurements
    );
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};

    use super::*;

    #[test]
    fn test_create_measurements() {
        create_measurements(10);

        let file = File::open(MEASUREMENT_FILE).unwrap();
        let input = BufReader::new(file);

        let mut lines = 0;

        for _ in input.lines() {
            lines += 1;
        }

        assert_eq!(lines, 10);
    }
}
