<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { intlFormat } from "../util/date";

    let curTime: Date = new Date();
    let timerInterval: NodeJS.Timer;

    onMount(() => {
        timerInterval = setInterval(() => {
            curTime = new Date();
        }, 1000);
    });

    onDestroy(() => {
        clearInterval(timerInterval);
    });

    $: secondsTime = curTime.getSeconds() < 10 ? `0${curTime.getSeconds()}` : curTime.getSeconds();
    $: minutesTime = curTime.getMinutes() < 10 ? `0${curTime.getMinutes()}` : curTime.getMinutes();
</script>

<div class="container">
    <div class="date">
        {new Intl.DateTimeFormat("default", { weekday: "long" }).format(curTime)},
        <br />
        {new Intl.DateTimeFormat("default", { month: "long" }).format(curTime)}
        {new Intl.DateTimeFormat("default", { day: "2-digit" }).format(curTime)},
        {new Intl.DateTimeFormat("default", { year: "numeric" }).format(curTime)}
    </div>
    <div class="time">
        <div class="mainTime">
            {intlFormat(curTime, { hour: "2-digit", hour12: false })}:{minutesTime}
        </div>
        <div class="secondsTime">
            :{secondsTime}
        </div>
    </div>
</div>

<style lang="scss">
    .container {
        font-family: "Raleway";
        display: flex;
        flex-direction: column;
        .date {
            font-size: 2rem;
            line-height: 2rem;
        }
        .time {
            display: flex;
            align-items: baseline;
            font-size: 8rem;
            line-height: 8rem;
            .secondsTime {
                font-size: 4rem;
                line-height: 4rem;
            }
        }
    }
</style>
