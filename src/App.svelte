<script>
    import TauriMain from "./lib/TauriMain.svelte";
    import Web from "./lib/Web.svelte";
</script>

{#if location.toString().includes("mirror")}
    <TauriMain />
{:else}
    <Web />
{/if}
