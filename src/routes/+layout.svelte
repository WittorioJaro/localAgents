<script lang="ts">
  import { cn } from '$lib/utils';
  import '../app.css';
  import { goto } from '$app/navigation';
  import { ollamaManager } from '$lib/stores/ollama-store.svelte';
  import "../app.css";
  
  let { children } = $props();
  let currentPath = $state('/');

  function navigate(path: string) {
    currentPath = path;
    goto(path, { replaceState: true });
  }

  // Start Ollama server when app launches
  $effect(() => {
    void ollamaManager.startServer();
  });
</script>

<div class={cn('min-h-screen bg-background font-sans antialiased')}>
  <nav class="border-b">
    <div class="container mx-auto px-4 py-3">
      <div class="flex items-center justify-between">
        <button 
          onclick={() => navigate('/')}
          class="text-xl font-semibold"
        >
          CrewAI UI
        </button>
        <div class="flex gap-4">
          <button 
            onclick={() => navigate('/')}
            class={cn(
              "px-3 py-2 rounded-md hover:bg-muted transition-colors",
              currentPath === '/' && "bg-muted"
            )}
          >
            Home
          </button>
          <button 
            onclick={() => navigate('/models')}
            class={cn(
              "px-3 py-2 rounded-md hover:bg-muted transition-colors",
              currentPath === '/models' && "bg-muted"
            )}
          >
            Models
          </button>
          <button 
            onclick={() => navigate('/agents')}
            class={cn(
              "px-3 py-2 rounded-md hover:bg-muted transition-colors",
              currentPath === '/agents' && "bg-muted"
            )}
          >
            Agents
          </button>
        </div>
      </div>
    </div>
  </nav>
  <main>
    {@render children()}
  </main>
</div>