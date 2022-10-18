<script>
    import { onDestroy } from "svelte";
    import Quote from "./components/Quote.svelte";
    import SpotifyNewReleases from "./components/SpotifyNewReleases.svelte";
    import SpotifyPlayer from "./components/SpotifyPlayer.svelte";
    import Time from "./components/Time.svelte";
    import Weather from "./components/Weather.svelte";
    import { config } from "./stores/store";

    const updateInterval = setInterval(() => {
        fetch("/api/config")
            .then(r => r.json())
            .then(data => {
                if (JSON.stringify(data) !== JSON.stringify($config)) {
                    $config = data;
                }
            })
            .catch(e => {
                console.error(e);
            });
    }, 1000);

    onDestroy(() => {
        clearInterval(updateInterval);
    });
</script>

<link href="/fontawesome/css/fontawesome.css" rel="stylesheet" />
<link href="/fontawesome/css/brands.css" rel="stylesheet" />
<link href="/fontawesome/css/solid.css" rel="stylesheet" />

{#if $config}
    <main class="mainContainer">
        <div class="time">
            {#if $config.time_enabled}
                <Time />
            {/if}
            {#if $config.quotes_enabled}
                <Quote />
            {/if}
        </div>
        {#if $config.weather_enabled}
            <div class="weather">
                <Weather />
            </div>
        {/if}
        <div class="spotifyPlaying">
            {#if $config.spotify_releases_enabled}
                <SpotifyNewReleases />
            {/if}
            {#if $config.spotify_playing_enabled}
                <SpotifyPlayer />
            {/if}
        </div>
    </main>
{/if}

<style lang="scss">
    .mainContainer {
        box-sizing: border-box;
        position: relative;
        .time {
            position: absolute;
            top: 80px;
            left: 40px;
            display: flex;
            flex-direction: column;
            gap: 40px;
        }
        .weather {
            position: absolute;
            top: 80px;
            right: 40px;
        }
        .spotifyPlaying {
            position: absolute;
            bottom: 80px;
            left: 40px;
            display: flex;
            flex-direction: column;
            gap: 40px;
        }
    }
</style>
