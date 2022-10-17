<script lang="ts">
    import { Client, getClient, ResponseType } from "@tauri-apps/api/http";
    import { onDestroy, onMount } from "svelte";
    import type { ISpotify } from "../models/spotify";
    import { config } from "../stores/store";
    import { msToMin } from "../util/date";
    import { clamp } from "../util/helpers";

    let updateInterval: NodeJS.Timer;
    let spotifyData: ISpotify.NowPlaying;
    let curTime = new Date();
    let updateTime = new Date();

    let client: Client;

    $: $config, client && getData();

    async function getData() {
        try {
            let r = await client.get<ISpotify.NowPlaying>("https://api.spotify.com/v1/me/player/currently-playing", {
                responseType: ResponseType.JSON,
                headers: {
                    Authorization: `Bearer ${$config.spotify_key}`
                }
            });
            if (r.ok) {
                spotifyData = r.data;
                updateTime = new Date();
            } else {
                console.error(r);
            }
        } catch (error) {
            console.error(error);
        }
    }

    onMount(async () => {
        client = await getClient();
        if (!$config.spotify_key) return;
        getData();
        updateInterval = setInterval(() => {
            getData();
        }, 5 * 1000);
        setInterval(() => {
            curTime = new Date();
        }, 20);
    });

    onDestroy(() => {
        clearInterval(updateInterval);
    });

    $: curProgress = clamp(0, spotifyData?.item?.duration_ms ?? 0, spotifyData ? (spotifyData.is_playing ? curTime.getTime() - updateTime.getTime() + spotifyData.progress_ms : spotifyData.progress_ms) : 0);
</script>

{#if spotifyData?.item}
    <div class="container">
        {#if spotifyData?.item?.album?.images[0]?.url}
            <div class="bgImg">
                <img src={spotifyData.item.album.images[0].url} alt="Album art" width="400" height="400" />
            </div>
        {/if}
        <div>
            <div class="album">
                <img src={spotifyData.item.album.images[0].url} alt="Album art" width="100" height="100" />
            </div>
            <div class="song">
                <div>
                    [{spotifyData.item.album.name}]
                </div>
                <div>
                    {spotifyData.item.name} - {spotifyData.item.artists.map(x => x.name).join(", ")}
                </div>
            </div>
            <div class="song">
                {msToMin(curProgress)}/{msToMin(spotifyData.item.duration_ms)}
            </div>
        </div>
        <div class="progress">
            <div class="bar" style="width: {((curProgress / spotifyData.item.duration_ms) * 100).toFixed(2)}%;" />
        </div>
    </div>
{/if}

<style lang="scss">
    .container {
        font-family: "Open Sans";
        display: inline-block;
        position: relative;
        .bgImg {
            position: absolute;
            top: -10%;
            left: -20px;
            width: calc(100% + 40px);
            height: 120%;
            opacity: 0.5;
            z-index: 1;
            overflow: hidden;
            filter: blur(6px) saturate(1.3);
            box-shadow: 0 0 5px 10px #000;
            img {
                filter: blur(6px) saturate(1.3);
                width: 100%;
                height: 100%;
                object-fit: cover;
            }
        }
        > div:not(.bgImg) {
            z-index: 5;
            display: flex;
            align-items: flex-end;
            flex-flow: row nowrap;
            // justify-content: space-between;
            gap: 10px;
            .song {
                display: flex;
                flex-direction: column;
                margin-bottom: 2px;
                :last-child {
                    font-size: 1.2em;
                    font-weight: 600;
                }
            }
            .album {
                img {
                    border-radius: 5px;
                    border: 2px solid #f6f6f6;
                }
            }
            * {
                z-index: 5;
                text-shadow: 1px 1px 1px #000;
            }
        }
        .progress {
            background-color: #444;
            width: 100%;
            min-width: 300px;
            height: 10px;
            border-radius: 8px;
            overflow: hidden;
            border: 1px solid #f6f6f6;
            .bar {
                background-color: #d00164;
                height: 20px;
                transition: width 200ms ease;
            }
        }
    }
</style>
