import type { IConfig } from "src/lib/models/config";
import { writable } from "svelte/store";

export const config = writable<IConfig>();
