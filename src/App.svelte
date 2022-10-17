<script>
    import TauriMain from "./lib/TauriMain.svelte";
    import Web from "./lib/Web.svelte";
</script>

{#if window.__TAURI_IPC__}
    <TauriMain />
{:else}
    <Web />
{/if}
