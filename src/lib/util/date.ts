export function intlFormat(date: Date, format: any) {
    return new Intl.DateTimeFormat("default", format).format(date);
}

export function msToMin(ms: number) {
    const minutes = Math.floor(ms / 60000);
    const seconds = Math.floor((ms % 60000) / 1000);
    return minutes + ":" + (seconds < 10 ? "0" : "") + seconds;
}

export function dateOnly(date: Date) {
    try {
        var dd = String(date.getDate()).padStart(2, "0");
        var mm = String(date.getMonth() + 1).padStart(2, "0");
        var yyyy = date.getFullYear();

        return dd + "/" + mm + "/" + yyyy;
    } catch (error) {
        return "-";
    }
}
