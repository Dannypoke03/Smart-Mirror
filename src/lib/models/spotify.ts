export namespace ISpotify {
    export interface ExternalUrls {
        spotify: string;
    }

    export interface Artist {
        external_urls: ExternalUrls;
        href: string;
        id: string;
        name: string;
        type: string;
        uri: string;
    }

    export interface ExternalUrls2 {
        spotify: string;
    }

    export interface Image {
        height: number;
        url: string;
        width: number;
    }

    export interface Album {
        album_type: string;
        artists: Artist[];
        available_markets: string[];
        external_urls: ExternalUrls2;
        href: string;
        id: string;
        images: Image[];
        name: string;
        release_date: string;
        release_date_precision: string;
        total_tracks: number;
        type: string;
        uri: string;
    }

    export interface ExternalUrls3 {
        spotify: string;
    }

    export interface Artist2 {
        external_urls: ExternalUrls3;
        href: string;
        id: string;
        name: string;
        type: string;
        uri: string;
    }

    export interface ExternalIds {
        isrc: string;
    }

    export interface ExternalUrls4 {
        spotify: string;
    }

    export interface ArtistItem {
        album: Album;
        artists: Artist2[];
        available_markets: string[];
        disc_number: number;
        duration_ms: number;
        explicit: boolean;
        external_ids: ExternalIds;
        external_urls: ExternalUrls4;
        href: string;
        id: string;
        is_local: boolean;
        name: string;
        popularity: number;
        preview_url: string;
        track_number: number;
        type: string;
        uri: string;
    }

    export interface Disallows {
        resuming: boolean;
        toggling_repeat_context: boolean;
        toggling_repeat_track: boolean;
        toggling_shuffle: boolean;
    }

    export interface Actions {
        disallows: Disallows;
    }

    export interface NowPlaying {
        timestamp: number;
        context?: any;
        progress_ms: number;
        item: ArtistItem;
        currently_playing_type: string;
        actions: Actions;
        is_playing: boolean;
    }

    export interface ExternalUrls {
        spotify: string;
    }

    export interface Followers {
        href?: any;
        total: number;
    }

    export interface Image {
        height: number;
        url: string;
        width: number;
    }

    export interface ArtistItem {
        external_urls: ExternalUrls;
        followers: Followers;
        genres: string[];
        href: string;
        id: string;
        images: Image[];
        name: string;
        popularity: number;
        type: string;
        uri: string;
    }

    export interface Cursors {
        after: string;
    }

    export interface Artists {
        items: ArtistItem[];
        next: string;
        total: number;
        cursors: Cursors;
        limit: number;
        href: string;
    }

    export interface FollowedArtists {
        artists: Artists;
    }

    export interface ExternalUrls {
        spotify: string;
    }

    export interface Artist {
        external_urls: ExternalUrls;
        href: string;
        id: string;
        name: string;
        type: string;
        uri: string;
    }

    export interface ExternalUrls2 {
        spotify: string;
    }

    export interface Image {
        height: number;
        url: string;
        width: number;
    }

    export interface AlbumItem {
        album_group: string;
        album_type: string;
        artists: Artist[];
        available_markets: string[];
        external_urls: ExternalUrls2;
        href: string;
        id: string;
        images: Image[];
        name: string;
        release_date: string;
        release_date_precision: string;
        total_tracks: number;
        type: string;
        uri: string;
    }

    export interface ArtistAlbums {
        href: string;
        items: AlbumItem[];
        limit: number;
        next: string;
        offset: number;
        previous?: any;
        total: number;
    }
}
