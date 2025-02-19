<script lang="ts">
  import { ollamaManager } from '$lib/stores/ollama-store.svelte';
  import { onMount } from 'svelte';

  onMount(async () => {
    await ollamaManager.checkOllamaStatus();
    if (ollamaManager.isOllamaRunning) {
      await ollamaManager.listInstalledModels();
    }
  });

  async function downloadModel(modelName: string) {
    await ollamaManager.downloadModel(modelName);
  }
</script>

<div class="container mx-auto p-4">
  <h1 class="text-4xl font-bold mb-8">Model Management</h1>

  {#if !ollamaManager.isOllamaRunning}
    <div class="bg-destructive/15 text-destructive p-4 rounded-lg mb-4">
      <h2 class="text-lg font-semibold">Ollama is not running</h2>
      <p>Please make sure Ollama is installed and running on your system.</p>
    </div>
  {:else}
    <div class="grid gap-6">
      <div class="bg-card p-6 rounded-lg shadow-sm">
        <h2 class="text-2xl font-semibold mb-4">Available Models</h2>
        <div class="grid gap-4">
          {#each ollamaManager.availableModels as model}
            {@const installedModel = ollamaManager.installedModels.find(m => m.name === model)}
            <div class="flex items-center justify-between p-4 bg-muted rounded-lg">
              <div>
                <h3 class="font-medium">{model}</h3>
                {#if installedModel}
                  <p class="text-sm text-muted-foreground">
                    Status: {installedModel.status}
                    {#if installedModel.progress !== undefined}
                      ({installedModel.progress}%)
                    {/if}
                  </p>
                {/if}
              </div>
              <button
                class="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors"
                disabled={installedModel?.status === 'downloading'}
                on:click={() => downloadModel(model)}
              >
                {#if installedModel?.status === 'ready'}
                  Installed
                {:else if installedModel?.status === 'downloading'}
                  Downloading...
                {:else}
                  Download
                {/if}
              </button>
            </div>
          {/each}
        </div>
      </div>

      {#if ollamaManager.installedModels.length > 0}
        <div class="bg-card p-6 rounded-lg shadow-sm">
          <h2 class="text-2xl font-semibold mb-4">Installed Models</h2>
          <div class="grid gap-4">
            {#each ollamaManager.installedModels.filter(m => m.status === 'ready') as model}
              <div class="flex items-center justify-between p-4 bg-muted rounded-lg">
                <h3 class="font-medium">{model.name}</h3>
                <span class="text-sm text-muted-foreground">Ready</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>