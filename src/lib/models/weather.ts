// export namespace IWeather {
//     export interface Notice {
//         copyright: string;
//         copyright_url: string;
//         disclaimer_url: string;
//         feedback_url: string;
//     }

//     export interface Header {
//         refresh_message: string;
//         ID: string;
//         main_ID: string;
//         name: string;
//         state_time_zone: string;
//         time_zone: string;
//         product_name: string;
//         state: string;
//     }

//     export interface Datum {
//         sort_order: number;
//         wmo: number;
//         name: string;
//         history_product: string;
//         local_date_time: string;
//         local_date_time_full: string;
//         aifstime_utc: string;
//         lat: number;
//         lon: number;
//         apparent_t: number;
//         cloud: string;
//         cloud_base_m?: number;
//         cloud_oktas?: number;
//         cloud_type_id?: any;
//         cloud_type: string;
//         delta_t: number;
//         gust_kmh: number;
//         gust_kt: number;
//         air_temp: number;
//         dewpt: number;
//         press: number;
//         press_qnh: number;
//         press_msl: number;
//         press_tend: string;
//         rain_trace: string;
//         rel_hum: number;
//         sea_state: string;
//         swell_dir_worded: string;
//         swell_height?: any;
//         swell_period?: any;
//         vis_km: string;
//         weather: string;
//         wind_dir: string;
//         wind_spd_kmh: number;
//         wind_spd_kt: number;
//     }

//     export interface Observations {
//         notice: Notice[];
//         header: Header[];
//         data: Datum[];
//     }

//     export interface Response {
//         observations: Observations;
//     }

//     export interface OpenWeather {
//         name: string;
//         local_names: { [key: string]: string };
//         lat: number;
//         lon: number;
//         country: string;
//         state: string;
//     }
// }

export interface OpenWeatherLocations {
    name: string;
    local_names: { [key: string]: string };
    lat: number;
    lon: number;
    country: string;
    state: string;
}

export namespace IOpenWeather {
    export interface Main {
        temp: number;
        feels_like: number;
        temp_min: number;
        temp_max: number;
        pressure: number;
        sea_level: number;
        grnd_level: number;
        humidity: number;
        temp_kf: number;
    }

    export interface Weather {
        id: number;
        main: string;
        description: string;
        icon: string;
    }

    export interface Clouds {
        all: number;
    }

    export interface Wind {
        speed: number;
        deg: number;
        gust: number;
    }

    export interface Sys {
        pod: string;
    }

    export interface Rain {
        "3h": number;
    }

    export interface List {
        dt: number;
        main: Main;
        weather: Weather[];
        clouds: Clouds;
        wind: Wind;
        visibility: number;
        pop: number;
        sys: Sys;
        dt_txt: string;
        rain: Rain;
    }

    export interface Coord {
        lat: number;
        lon: number;
    }

    export interface City {
        id: number;
        name: string;
        coord: Coord;
        country: string;
        population: number;
        timezone: number;
        sunrise: number;
        sunset: number;
    }

    export interface RootObject {
        cod: string;
        message: number;
        cnt: number;
        list: List[];
        city: City;
    }
}

export const IconMap = {
    "01d": "fa-regular fa-sun", // "clear sky"
    "02d": "fa-regular fa-cloud", // "few clouds"
    "03d": "fa-regular fa-cloud-sun", // "scattered clouds"
    "04d": "fa-regular fa-cloud", // "broken clouds"
    "09d": "fa-regular fa-cloud-rain", // "shower rain"
    "10d": "fa-regular fa-cloud-showers-heavy", // "rain"
    "11d": "fa-regular fa-cloud-bolt", // "thunderstorm"
    "13d": "fa-regular fa-snowflake", // "snow"
    "50d": "fa-regular fa-cloud-rain", // "mist"
    "01n": "fa-regular fa-moon", // "clear sky night"
    "02n": "fa-regular fa-cloud-moon", // "few clouds night"
    "03n": "fa-regular fa-cloud-moon", // "scattered clouds night"
    "04n": "fa-regular fa-cloud-moon", // "broken clouds night"
    "09n": "fa-regular fa-cloud-moon-rain", // "shower rain night"
    "10n": "fa-regular fa-cloud-moon-rain", // "rain night"
    "11n": "fa-regular fa-cloud-bolt", // "thunderstorm night"
    "13n": "fa-regular fa-snowflake", // "snow night"
    "50n": "fa-regular fa-cloud-moon" // "mist night"
};
