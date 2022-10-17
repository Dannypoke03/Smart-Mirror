<script lang="ts">
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";
    import type { IConfig } from "./models/config";
    import type { IQuotes } from "./models/quotes";
    import { capitalizeWords } from "./util/helpers";

    let config: IConfig;

    let quoteTags: IQuotes.Tag[] = [];

    function getConfig() {
        fetch("/api/config")
            .then(r => r.json())
            .then(data => {
                config = data;
            })
            .catch(e => {
                console.error(e);
            });
    }

    function updateConfig() {
        if (!config) return;
        config.quotes_interval = +config.quotes_interval;
        fetch("/api/config", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(config)
        })
            .then(r => r.json())
            .then(data => {
                config = data;
                setNotification("Config updated", "success");
            })
            .catch(err => {
                console.error(err);
                setNotification("Error updating config", "error");
            });
    }

    let notification = {
        title: "",
        body: "",
        icon: ""
    };

    function setNotification(msg: string, type: string, timeout: number = 5000) {
        notification.title = type;
        notification.body = msg;
        notification.icon = type === "error" ? "error" : "success";
        setTimeout(() => {
            notification.title = "";
            notification.body = "";
            notification.icon = "";
        }, timeout);
    }

    onMount(async () => {
        getConfig();
        quoteTags = await fetch("https://api.quotable.io/tags").then(r => r.json());
        setTimeout(() => {
            var elems = document.querySelectorAll("select");
            var instances = window.M.FormSelect.init(elems, {});
        }, 1000);
    });
</script>

<svelte:head>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/js/materialize.min.js"></script>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/css/materialize.min.css" />

    <link href="/fontawesome/css/fontawesome.css" rel="stylesheet" />
    <link href="/fontawesome/css/brands.css" rel="stylesheet" />
    <link href="/fontawesome/css/solid.css" rel="stylesheet" />
</svelte:head>

<div class="main">
    {#if config}
        <h1>Smart Mirror Config</h1>
        {#if notification.title}
            <div in:fade out:fade class="notification {notification.icon}">
                <p>{capitalizeWords(notification.title)} - {notification.body}</p>
            </div>
        {/if}
        <a href="/api/spotify-login" class="btn waves-effect waves-light">
            <i class="fab fa-spotify" />
            <span>Connect to Spotify</span>
        </a>
        <br />
        <br />
        <div class="input-field">
            <label class="active" for="weatherLocation">Weather Location</label>
            <input class="validate" id="weatherLocation" type="text" bind:value={config.weather_location} />
        </div>
        <br />
        <div class="input-field">
            <label class="active" for="weatherLocation">Quote Cycle Frequency (Seconds)</label>
            <input class="validate" id="weatherLocation" type="text" bind:value={config.quotes_interval} />
        </div>
        <br />
        <div class="input-field select">
            <select multiple bind:value={config.quote_tags}>
                {#each quoteTags as tag}
                    <option selected={config.quote_tags.includes(tag.name)} value={tag.name}>{capitalizeWords(tag.name)}</option>
                {/each}
            </select>
            <!-- svelte-ignore a11y-label-has-associated-control -->
            <label> Quote Tags </label>
        </div>
        <br />
        <p>
            <label>
                <input type="checkbox" bind:checked={config.weather_enabled} />
                <span> Weather Enabled </span>
            </label>
        </p>
        <p>
            <label>
                <input type="checkbox" bind:checked={config.time_enabled} />
                <span> Time Enabled </span>
            </label>
        </p>
        <p>
            <label>
                <input type="checkbox" bind:checked={config.spotify_playing_enabled} />
                <span> Spotify Now Playing Enabled </span>
            </label>
        </p>
        <p>
            <label>
                <input type="checkbox" bind:checked={config.spotify_releases_enabled} />
                <span> Spotify New Releases Enabled </span>
            </label>
        </p>
        <p>
            <label>
                <input type="checkbox" bind:checked={config.quotes_enabled} />
                <span> Quotes Enabled </span>
            </label>
        </p>
        <br />
        <button class="waves-effect waves-light btn" on:click={() => updateConfig()}> Update </button>
    {/if}
</div>

<style lang="scss">
    * {
        font-family: "Open Sans";
        color: white;
    }
    .main {
        margin: 10px;
    }

    .notification {
        padding: 5px;
        margin-bottom: 15px;
        border-radius: 5px;

        &.error {
            background-color: #f44336;
        }
        &.success {
            background-color: #4caf50;
        }
    }

    :global(.select input) {
        color: #fff;
    }
    :global(.select svg) {
        fill: #fff !important;
    }
</style>
