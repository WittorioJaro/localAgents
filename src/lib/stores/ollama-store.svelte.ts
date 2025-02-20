import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { OllamaModel } from '$lib/types';

export interface OllamaModelStatus {
  name: string;
  status: 'downloading' | 'ready' | 'error';
  progress?: number;
  error?: string;
  downloaded?: string;
  total?: string;
  speed?: string;
  eta?: string;
}

export interface CrewAIConfig {
  modelName: string;
  task: string;
  role: string;
  goal: string;
  backstory?: string;
}

export class OllamaManager {
  availableModels = $state<string[]>([
    'llama3.2',
    'qwen:0.5b',
  ]);
  
  installedModels = $state<OllamaModelStatus[]>([]);
  isOllamaRunning = $state<boolean>(false);
  isStartingServer = $state<boolean>(false);
  startupError = $state<string | null>(null);

  constructor() {
    this.setupProgressListener();
    this.setupModelsListener();
  }

  private async setupModelsListener() {
    await listen<string[]>('models-updated', (event) => {
      this.installedModels = event.payload.map(name => ({
        name,
        status: 'ready'
      }));
    });
  }

  private async setupProgressListener() {
    await listen<{
      percent: number;
      downloaded: string;
      total: string;
      speed?: string;
      eta?: string;
    }>('download-progress', (event) => {
      const { percent, downloaded, total, speed, eta } = event.payload;
      const downloadingModel = this.installedModels.find(m => m.status === 'downloading');
      if (downloadingModel) {
        this.updateModelStatus(
          downloadingModel.name,
          'downloading',
          undefined,
          percent,
          downloaded,
          total,
          speed,
          eta
        );
      }
    });
  }

  async startServer() {
    if (this.isStartingServer) return;
    
    this.isStartingServer = true;
    this.startupError = null;
    
    try {
      await invoke('start_ollama_server');
      const status = await this.checkOllamaStatus();
      if (!status) {
        throw new Error('Server started but status check failed');
      }
    } catch (error) {
      console.error('Failed to start Ollama server:', error);
      this.isOllamaRunning = false;
      this.startupError = error instanceof Error ? error.message : 'Failed to start Ollama server';
      throw error;
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

    console.log(`Starting download of model: ${modelName}`);

    // Add model to installedModels with downloading status
    this.installedModels = [
      ...this.installedModels.filter(m => m.name !== modelName),
      { name: modelName, status: 'downloading', progress: 0 }
    ];

    try {
      await invoke('download_ollama_model', { modelName });
      console.log(`Successfully downloaded model: ${modelName}`);
      this.updateModelStatus(modelName, 'ready');
      await this.listInstalledModels(); // Refresh the list
    } catch (error) {
      console.error(`Failed to download model ${modelName}:`, error);
      // Don't treat manifest writing as an error
      if (error instanceof Error && error.message.includes('writing manifest')) {
        this.updateModelStatus(modelName, 'ready');
        await this.listInstalledModels();
      } else {
        this.updateModelStatus(
          modelName, 
          'error', 
          error instanceof Error ? error.message : String(error)
        );
      }
    }
  }

  async listInstalledModels() {
    if (!this.isOllamaRunning) {
      await this.startServer();
    }

    try {
      const models = await invoke<string[]>('list_ollama_models');
      console.log('Installed models:', models);
      this.installedModels = models.map(name => ({
        name,
        status: 'ready'
      }));
    } catch (error) {
      console.error('Failed to list installed models:', error);
    }
  }

  async deleteModel(modelName: string) {
    try {
      await invoke('delete_ollama_model', { modelName });
      console.log(`Successfully deleted model: ${modelName}`);
      // Remove from installed models
      this.installedModels = this.installedModels.filter(m => m.name !== modelName);
    } catch (error) {
      console.error(`Failed to delete model ${modelName}:`, error);
      this.updateModelStatus(
        modelName,
        'error',
        error instanceof Error ? error.message : String(error)
      );
    }
  }

  private updateModelStatus(
    modelName: string,
    status: OllamaModelStatus['status'],
    error?: string,
    progress?: number,
    downloaded?: string,
    total?: string,
    speed?: string,
    eta?: string
  ) {
    console.log(`Updating model ${modelName} status:`, { 
      status, error, progress, downloaded, total, speed, eta 
    });
    this.installedModels = this.installedModels.map(model =>
      model.name === modelName
        ? { ...model, status, error, progress, downloaded, total, speed, eta }
        : model
    );
  }

  async executeCrewAITask(config: CrewAIConfig): Promise<string> {
    if (!this.isOllamaRunning) {
      await this.startServer();
    }

    const installedModel = this.installedModels.find(m => m.name === config.modelName);
    if (!installedModel) {
      throw new Error(`Model ${config.modelName} is not installed`);
    }

    console.log('Executing CrewAI task:', config);
    
    try {
      const result = await invoke<string>('execute_crewai_task', { config });
      console.log('CrewAI task result:', result);
      return result;
    } catch (error) {
      console.error('Failed to execute CrewAI task:', error);
      throw error;
    }
  }
}

export const ollamaManager = new OllamaManager(); 