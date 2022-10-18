<script lang="ts">
    // import { Client, getClient, ResponseType } from "@tauri-apps/api/http";
    import { onDestroy, onMount } from "svelte";
    import { IconMap, type IOpenWeather, type OpenWeatherLocations } from "../models/weather";
    import { config } from "../stores/store";
    import { dateOnly, intlFormat } from "../util/date";
    import { capitalizeWords } from "../util/helpers";

    let updateInterval: NodeJS.Timer;
    let weatherData: IOpenWeather.RootObject;
    let curWeather: IOpenWeather.List;
    let forecast: { [key: string]: IOpenWeather.List[] } = {};

    // let client: Client;

    const key = "e54ac00966ee06bcf68722c86925b326";

    $: $config, getWeatherData();

    async function getWeatherData() {
        try {
            let r = await fetch(`https://api.openweathermap.org/geo/1.0/direct?q=${$config.weather_location}&limit=10&appId=${key}`);
            let data: OpenWeatherLocations[] = await r.json();
            if (r.ok && data.length > 0) {
                let area = data[0];
                let r2 = await fetch(`https://api.openweathermap.org/data/2.5/forecast?appId=e54ac00966ee06bcf68722c86925b326&lat=${area.lat}&lon=${area.lon}&units=metric`);
                let r2Data: IOpenWeather.RootObject = await r2.json();
                if (r2.ok && r2Data?.list?.length > 0) {
                    weatherData = r2Data;

                    let now = new Date();
                    let closest = weatherData.list[0];
                    let closestDiff = Math.abs(now.getTime() - closest.dt * 1000);
                    for (let i = 1; i < weatherData.list.length; i++) {
                        let diff = Math.abs(now.getTime() - weatherData.list[i].dt * 1000);
                        if (diff < closestDiff) {
                            closest = weatherData[i];
                            closestDiff = diff;
                        }
                    }
                    curWeather = closest;
                } else {
                    console.error(r2);
                }

                let forecastDate = new Date();
                let valid = weatherData.list.filter(x => new Date(x.dt * 1000).getTime() > forecastDate.getTime());
                for (const weather of valid) {
                    if (!forecast[dateOnly(new Date(weather.dt * 1000))]) {
                        forecast[dateOnly(new Date(weather.dt * 1000))] = [];
                    }
                    forecast[dateOnly(new Date(weather.dt * 1000))].push(weather);
                }
            }
        } catch (error) {
            console.error(error);
        }
    }

    onMount(async () => {
        getWeatherData();
        updateInterval = setInterval(() => {
            getWeatherData();
        }, 10 * 60 * 1000);
    });

    onDestroy(() => {
        clearInterval(updateInterval);
    });

    function getMax(list: IOpenWeather.List[]) {
        let max = list[0].main.temp_max;
        for (let i = 1; i < list.length; i++) {
            if (list[i].main.temp_max > max) {
                max = list[i].main.temp_max;
            }
        }
        return max;
    }

    function getMin(list: IOpenWeather.List[]) {
        let min = list[0].main.temp_min;
        for (let i = 1; i < list.length; i++) {
            if (list[i].main.temp_min < min) {
                min = list[i].main.temp_min;
            }
        }
        return min;
    }

    function avgRainChange(list: IOpenWeather.List[]) {
        let total = 0;
        for (let i = 0; i < list.length; i++) {
            total += list[i].pop;
        }
        return total / list.length;
    }
</script>

{#if weatherData}
    <div class="container">
        {#if curWeather}
            <div class="current">
                <p>
                    {weatherData.city.name}
                </p>
                <div class="info">
                    <div class="icon">
                        <i class={IconMap[curWeather.weather[0].icon]} />
                    </div>
                    <div class="temps">
                        <div class="curTemp">
                            {curWeather.main.temp.toFixed(1)}°
                        </div>
                        <div class="feelsLike">
                            Feels like {curWeather.main.feels_like.toFixed(1)}°C
                        </div>
                    </div>
                    <span>
                        {capitalizeWords(curWeather.weather[0].description)}
                    </span>
                    <div>
                        <span>
                            <i class="fa-regular fa-sun" />
                            {intlFormat(new Date(weatherData.city.sunrise * 1000), {
                                hour: "numeric",
                                minute: "numeric",
                                hour12: true
                            })}
                        </span>
                        <span>
                            <i class="fa-regular fa-moon" />
                            {intlFormat(new Date(weatherData.city.sunset * 1000), {
                                hour: "numeric",
                                minute: "numeric",
                                hour12: true
                            })}
                        </span>
                    </div>
                    <div class="wind">
                        <i class="fa-solid fa-wind" />
                        {curWeather.wind.speed}m/s
                    </div>
                    <div class="rain">
                        <i class="fa-solid fa-umbrella" />
                        {(curWeather.pop * 100).toFixed(0)}% - {curWeather.rain?.["3h"] ?? 0} mm
                    </div>
                </div>
            </div>
        {/if}
        {#if forecast}
            <div class="forecast">
                <div class="forecastList">
                    {#each Object.keys(forecast) as day}
                        {#if forecast[day].length > 0}
                            <div class="forecastItem">
                                <div style="text-align: left;">
                                    {intlFormat(new Date(forecast[day][0].dt * 1000), {
                                        weekday: "short"
                                    })}
                                </div>
                                <div>
                                    <i class={IconMap[forecast[day][0].weather[0].icon]} />
                                </div>
                                <div>
                                    {getMax(forecast[day])} / {getMin(forecast[day])}°C
                                </div>
                                <div>
                                    <i class="fa-solid fa-umbrella" />
                                    {avgRainChange(forecast[day]).toFixed(0)}%
                                </div>
                            </div>
                        {/if}
                    {/each}
                </div>
            </div>
        {/if}
    </div>
{/if}

<style lang="scss">
    .container {
        text-align: right;
        display: inline-block;
        .forecastList {
            margin-top: 15px;
            display: inline-flex;
            flex-direction: column;
            gap: 10px;
            .forecastItem {
                display: grid;
                grid-template-columns: 80px 40px 160px 60px;
                :first-child {
                    font-weight: bold;
                }
            }
        }
        .current {
            display: inline-block;
            font-family: "Open Sans", sans-serif;
            p {
                font-size: 1.5rem;
                font-weight: bold;
            }
            .info {
                display: grid;
                text-align: center;
                align-items: center;
                grid-template-columns: 1fr 1fr;
                gap: 10px;
                .icon {
                    i {
                        font-size: 8rem;
                    }
                }
                > .temps {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    &:not(:first-child) {
                        justify-content: space-between;
                        font-family: "Raleway";
                    }
                    gap: 10px;
                    .curTemp {
                        font-size: 8rem;
                        line-height: 8rem;
                        font-family: "Raleway";
                    }
                }
            }
        }
    }
</style>
