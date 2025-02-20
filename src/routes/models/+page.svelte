<script lang="ts">
  import { ollamaManager } from '$lib/stores/ollama-store.svelte';
  import { onMount } from 'svelte';

  let isInitializing = $state(true);

  onMount(async () => {
    try {
      await ollamaManager.checkOllamaStatus();
      if (ollamaManager.isOllamaRunning) {
        await ollamaManager.listInstalledModels();
      }
    } catch (error) {
      console.error('Failed to initialize Ollama:', error);
    } finally {
      isInitializing = false;
    }
  });

  async function downloadModel(modelName: string) {
    await ollamaManager.downloadModel(modelName);
  }

  async function retryStartServer() {
    try {
      await ollamaManager.startServer();
      await ollamaManager.listInstalledModels();
    } catch (error) {
      console.error('Failed to start server:', error);
    }
  }
</script>

<div class="container mx-auto p-4">
  <h1 class="text-4xl font-bold mb-8">Model Management</h1>

  {#if isInitializing}
    <div class="flex items-center justify-center p-8">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      <span class="ml-3">Initializing Ollama...</span>
    </div>
  {:else if !ollamaManager.isOllamaRunning}
    <div class="bg-destructive/15 text-destructive p-4 rounded-lg mb-4">
      <h2 class="text-lg font-semibold">Ollama is not running</h2>
      <p class="mb-4">
        {#if ollamaManager.startupError}
          Error: {ollamaManager.startupError}
        {:else}
          Please make sure Ollama is installed and running on your system.
        {/if}
      </p>
      <button
        class="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors"
        on:click={retryStartServer}
        disabled={ollamaManager.isStartingServer}
      >
        {#if ollamaManager.isStartingServer}
          Starting Server...
        {:else}
          Retry Starting Server
        {/if}
      </button>
    </div>
  {:else}
    <div class="grid gap-6">
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
                  {#if installedModel.error}
                    <span class="text-destructive">Error: {installedModel.error}</span>
                  {/if}
                </p>
              {/if}
            </div>
            <button
              class="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors disabled:opacity-50"
              disabled={installedModel?.status === 'downloading'}
              on:click={() => downloadModel(model)}
            >
              {#if installedModel?.status === 'ready'}
                Installed
              {:else if installedModel?.status === 'downloading'}
                <div class="flex flex-col items-start gap-1">
                  <div>Downloading... {installedModel.progress?.toFixed(1)}%</div>
                  {#if installedModel.downloaded}
                    <div class="text-xs text-muted-foreground">
                      {installedModel.downloaded} / {installedModel.total}
                      {#if installedModel.speed}
                        • {installedModel.speed}
                      {/if}
                      {#if installedModel.eta}
                        • {installedModel.eta}
                      {/if}
                    </div>
                  {/if}
                </div>
              {:else if installedModel?.status === 'error'}
                <div class="flex flex-col items-start gap-1">
                  <div>Error</div>
                  {#if installedModel.error}
                    <div class="text-xs text-destructive">{installedModel.error}</div>
                  {/if}
                </div>
              {:else}
                Download
              {/if}
            </button>
          </div>
        {/each}
      </div>

      {#if ollamaManager.installedModels.length > 0}
        <div class="bg-card p-6 rounded-lg shadow-sm">
          <h2 class="text-2xl font-semibold mb-4">Installed Models</h2>
          <div class="grid gap-4">
            {#each ollamaManager.installedModels.filter(m => m.status === 'ready') as model}
              <div class="flex items-center justify-between p-4 bg-muted rounded-lg">
                <h3 class="font-medium">{model.name}</h3>
                <div class="flex gap-2 items-center">
                  <span class="text-sm text-muted-foreground">Ready</span>
                  <button
                    class="px-3 py-1 bg-destructive text-destructive-foreground rounded-md hover:bg-destructive/90 transition-colors"
                    on:click={() => ollamaManager.deleteModel(model.name)}
                  >
                    Delete
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>