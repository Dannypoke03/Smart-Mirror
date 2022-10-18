<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import type { ISpotify } from "../models/spotify";
    import { config } from "../stores/store";
    import { dateOnly } from "../util/date";
    import { capitalizeWords } from "../util/helpers";

    let updateInterval: NodeJS.Timer;
    let followedArtits: ISpotify.ArtistItem[] = [];
    let albums: ISpotify.AlbumItem[] = [];
    let albumsLoaded = false;
    let requesting = false;

    $: $config, getData();

    async function getData() {
        if (requesting) return;
        requesting = true;
        followedArtits = [];
        try {
            let r = await fetch("https://api.spotify.com/v1/me/following?type=artist&limit=50", {
                headers: {
                    Authorization: `Bearer ${$config.spotify_key}`
                }
            });
            if (r.ok) {
                let data: ISpotify.FollowedArtists = await r.json();
                followedArtits = [...followedArtits, ...data.artists.items];
                if (data.artists.next) {
                    let next = data.artists.next;
                    while (next) {
                        let r = await fetch(next, {
                            headers: {
                                Authorization: `Bearer ${$config.spotify_key}`
                            }
                        });
                        let data: ISpotify.FollowedArtists = await r.json();
                        if (r.ok) {
                            followedArtits = [...followedArtits, ...data.artists.items];
                            next = data.artists.next;
                        } else {
                            console.error(r);
                            next = null;
                        }
                    }
                }
            } else {
                console.error(r);
            }
        } catch (error) {
            console.error(error);
        }

        albums = [];
        let promises: Promise<void>[] = [];
        for (const artist of followedArtits) {
            let promise = fetch(`https://api.spotify.com/v1/artists/${artist.id}/albums`, {
                headers: {
                    Authorization: `Bearer ${$config.spotify_key}`
                }
            })
                .then(async r => {
                    if (r.ok) {
                        let data: ISpotify.ArtistAlbums = await r.json();
                        albums = [...albums, ...data.items];
                    } else {
                        console.error(r);
                    }
                })
                .catch(error => {
                    console.error(error);
                });

            promises.push(promise);
        }
        await Promise.all(promises);
        albums.sort((a, b) => {
            return new Date(b.release_date).getTime() - new Date(a.release_date).getTime();
        });
        albumsLoaded = true;
        albums = albums;
        requesting = false;
    }

    onMount(async () => {
        if (!$config.spotify_key) return;
        getData();
        updateInterval = setInterval(() => {
            getData();
        }, 10 * 60 * 1000);
    });

    onDestroy(() => {
        clearInterval(updateInterval);
    });
</script>

{#if albums.length > 0 && albumsLoaded}
    <div class="container">
        <div class="albums">
            {#each albums.slice(0, 5) as album}
                <div class="album">
                    <img src={album.images[0].url} width="75" height="75" alt="" />
                    <div class="info">
                        <div class="type">{capitalizeWords(album.album_type)}</div>
                        <div class="name">{album.name}</div>
                        <div class="artist">{album.artists.map(x => x.name).join(", ")} - {dateOnly(new Date(album.release_date))}</div>
                        {#if album.total_tracks > 1}
                            <div class="tracks">{album.total_tracks} tracks</div>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
    </div>
{/if}

<style lang="scss">
    .albums {
        display: flex;
        flex-direction: column-reverse;
        gap: 10px;
        font-family: "Open Sans";
        .album {
            display: flex;
            align-items: center;
            gap: 10px;
            img {
                border-radius: 8px;
                height: 75px;
                width: auto;
            }
            .info {
                display: flex;
                flex-direction: column;
                > div {
                    max-width: 300px;
                    overflow: hidden;
                    white-space: nowrap;
                }
                .type {
                    font-size: 0.8em;
                    color: #999;
                    font-style: italic;
                }
                .name {
                    text-overflow: ellipsis;
                    font-weight: bold;
                }
                .artist {
                    font-size: 0.8em;
                    color: #999;
                }
                .tracks {
                    font-size: 0.7em;
                    line-height: 0.8rem;
                    color: #999;
                }
            }
        }
    }
</style>
