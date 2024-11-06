<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let brewList = new Array;

    invoke("brew_list")
    // @ts-ignore
        .then((list) => (brewList = list));

    function brewInstall(packageName: string) {
        invoke("brew_install_formula", { package_name: packageName })
            .then(() => brewList.push(packageName))
            .catch((error) => alert(error))
    }
</script>

<main class="container">
    <div style="text-align: center;">
        <label>
            Search<br />
            <input>
        </label>
    </div>
    {#if brewList.length === 0}
        <h3>Nothing</h3>
    {:else}
        {#each brewList as installedPackage}
            <h3>{installedPackage}</h3>
        {/each}
    {/if}
</main>

<style>
    main {
        color: white;
    }
</style>
