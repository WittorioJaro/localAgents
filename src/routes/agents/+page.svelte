<script lang="ts">
  import { ollamaManager } from '$lib/stores/ollama-store.svelte';
  import { onMount } from 'svelte';

  let isLoading = $state(false);
  let result = $state('');
  let error = $state('');
  let isServiceReady = $state(false);

  let config = $state({
    modelName: '',
    role: '',
    goal: '',
    task: '',
    backstory: ''
  });

  onMount(async () => {
    try {
      await ollamaManager.listInstalledModels();
      // Test if Python service is ready
      const response = await fetch('http://127.0.0.1:3001/docs');
      isServiceReady = response.ok;
    } catch (e) {
      console.error('Service check failed:', e);
      isServiceReady = false;
      error = 'CrewAI service is not running. Please restart the application.';
    }
  });

  async function handleSubmit() {
    isLoading = true;
    error = '';
    result = '';
    
    if (!isServiceReady) {
      error = 'CrewAI service is not running. Please restart the application.';
      isLoading = false;
      return;
    }

    try {
      result = await ollamaManager.executeCrewAITask(config);
    } catch (e) {
      console.error('Task execution failed:', e);
      error = e instanceof Error ? e.message : String(e);
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="container mx-auto p-4 max-w-2xl">
  <h1 class="text-4xl font-bold mb-8">Create Agent</h1>

  <form on:submit|preventDefault={handleSubmit} class="space-y-6">
    <div class="space-y-4">
      <div>
        <label for="model" class="block text-sm font-medium mb-2">Model</label>
        <select
          id="model"
          bind:value={config.modelName}
          class="w-full p-2 border rounded-md bg-background"
          required
        >
          <option value="">Select a model</option>
          {#each ollamaManager.installedModels.filter(m => m.status === 'ready') as model}
            <option value={model.name}>{model.name}</option>
          {/each}
        </select>
      </div>

      <div>
        <label for="role" class="block text-sm font-medium mb-2">Role</label>
        <input
          id="role"
          type="text"
          bind:value={config.role}
          placeholder="e.g., Python Developer"
          class="w-full p-2 border rounded-md"
          required
        />
      </div>

      <div>
        <label for="goal" class="block text-sm font-medium mb-2">Goal</label>
        <input
          id="goal"
          type="text"
          bind:value={config.goal}
          placeholder="e.g., Write efficient and maintainable code"
          class="w-full p-2 border rounded-md"
          required
        />
      </div>

      <div>
        <label for="task" class="block text-sm font-medium mb-2">Task</label>
        <textarea
          id="task"
          bind:value={config.task}
          placeholder="e.g., Write a function that calculates fibonacci numbers"
          class="w-full p-2 border rounded-md h-24"
          required
        />
      </div>

      <div>
        <label for="backstory" class="block text-sm font-medium mb-2">Backstory (Optional)</label>
        <textarea
          id="backstory"
          bind:value={config.backstory}
          placeholder="e.g., You are an experienced developer with 10 years of experience..."
          class="w-full p-2 border rounded-md h-24"
        />
      </div>
    </div>

    <button
      type="submit"
      class="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors disabled:opacity-50"
      disabled={isLoading || !ollamaManager.isOllamaRunning || !isServiceReady}
    >
      {#if isLoading}
        Running...
      {:else if !isServiceReady}
        Service Not Ready
      {:else if !ollamaManager.isOllamaRunning}
        Ollama Not Running
      {:else}
        Run Agent
      {/if}
    </button>
  </form>

  {#if error}
    <div class="mt-6 p-4 bg-destructive/15 text-destructive rounded-md">
      {error}
    </div>
  {/if}

  {#if result}
    <div class="mt-6 p-4 bg-muted rounded-md">
      <h2 class="text-lg font-semibold mb-2">Result:</h2>
      <pre class="whitespace-pre-wrap">{result}</pre>
    </div>
  {/if}
</div> 