export interface IConfig {
    spotify_key: string;
    spotify_playing_enabled: boolean;
    spotify_releases_enabled: boolean;
    time_enabled: boolean;
    weather_enabled: boolean;
    weather_location: string;
    quotes_enabled: boolean;
    quotes_interval: number;
    quote_tags: string[];
}
