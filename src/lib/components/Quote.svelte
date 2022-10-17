<script lang="ts">
    import { Client, getClient, ResponseType } from "@tauri-apps/api/http";
    import { onDestroy, onMount } from "svelte";
    import type { IQuotes } from "../models/quotes";
    import { config } from "../stores/store";

    let updateInterval: NodeJS.Timer;
    let quote: IQuotes.RandomQuote = null;
    let client: Client;

    $: $config, client && init();

    async function getData() {
        try {
            let r = await client.get<IQuotes.RandomQuote>(`https://api.quotable.io/random${$config.quote_tags.length > 0 ? "?tags" + $config.quote_tags.join(",") : ""}`, {
                responseType: ResponseType.JSON,
                headers: {
                    Authorization: `Bearer ${$config.spotify_key}`
                }
            });
            if (r.ok) {
                quote = r.data;
            } else {
                console.error(r);
            }
        } catch (error) {
            console.error(error);
        }
    }

    onMount(async () => {
        client = await getClient();
        getData();
        init();
    });

    function init() {
        clearInterval(updateInterval);
        updateInterval = setInterval(() => {
            getData();
        }, $config.quotes_interval * 1000);
    }

    onDestroy(() => {
        clearInterval(updateInterval);
    });
</script>

{#if quote?.content}
    <div class="quote">
        <p>{quote?.content}</p>
        <p>- {quote?.author}</p>
    </div>
{/if}

<style lang="scss">
    .quote {
        max-width: 400px;
        font-family: "Open Sans", sans-serif;
        > p {
            margin: 0px;
            margin-bottom: 5px;
            &:last-child {
                font-family: "Raleway", sans-serif;
            }
        }
    }
</style>
