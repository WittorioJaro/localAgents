import { invoke } from '@tauri-apps/api/core';
import type { OllamaModel } from '$lib/types';

export interface OllamaModelStatus {
  name: string;
  status: 'downloading' | 'ready' | 'error';
  progress?: number;
  error?: string;
}

export class OllamaManager {
  availableModels = $state<string[]>([
    'mistral:3.2-q4_K_M',
    'llama2:3.2-q4_K_M',
    'codellama:3.2-q4_K_M'
  ]);
  
  installedModels = $state<OllamaModelStatus[]>([]);
  isOllamaRunning = $state<boolean>(false);
  isStartingServer = $state<boolean>(false);

  async startServer() {
    if (this.isStartingServer) return;
    
    this.isStartingServer = true;
    try {
      await invoke('start_ollama_server');
      // Wait for server to start
      await new Promise(resolve => setTimeout(resolve, 2000));
      await this.checkOllamaStatus();
    } catch (error) {
      console.error('Failed to start Ollama server:', error);
      this.isOllamaRunning = false;
    } finally {
      this.isStartingServer = false;
    }
  }

  async checkOllamaStatus() {
    try {
      const status = await invoke<boolean>('check_ollama_status');
      this.isOllamaRunning = status;
      if (!status && !this.isStartingServer) {
        await this.startServer();
      }
      return status;
    } catch (error) {
      console.error('Failed to check Ollama status:', error);
      this.isOllamaRunning = false;
      return false;
    }
  }

  async downloadModel(modelName: string) {
    if (!this.isOllamaRunning) {
      await this.startServer();
    }

    // Add model to installedModels with downloading status
    this.installedModels = [
      ...this.installedModels,
      { name: modelName, status: 'downloading', progress: 0 }
    ];

    try {
      await invoke('download_ollama_model', { modelName });
      // Update model status to ready
      this.updateModelStatus(modelName, 'ready');
    } catch (error) {
      console.error(`Failed to download model ${modelName}:`, error);
      this.updateModelStatus(modelName, 'error', error as string);
    }
  }

  async listInstalledModels() {
    if (!this.isOllamaRunning) {
      await this.startServer();
    }

    try {
      const models = await invoke<string[]>('list_ollama_models');
      this.installedModels = models.map(name => ({
        name,
        status: 'ready'
      }));
    } catch (error) {
      console.error('Failed to list installed models:', error);
    }
  }

  private updateModelStatus(
    modelName: string,
    status: OllamaModelStatus['status'],
    error?: string,
    progress?: number
  ) {
    this.installedModels = this.installedModels.map(model =>
      model.name === modelName
        ? { ...model, status, error, progress }
        : model
    );
  }
}

export const ollamaManager = new OllamaManager(); 