export function clamp(min: number, max: number, input: number) {
    return Math.max(min, Math.min(max, input));
}

export function capitalizeWords(input: string) {
    return input.replace(/\w\S*/g, txt => txt.charAt(0).toUpperCase() + txt.substr(1).toLowerCase());
}
