import type { Agent, Workflow } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

export class AgentManager {
  agents = $state<Agent[]>([]);
  activeWorkflow = $state<Workflow | null>(null);
  
  async createAgent(agent: Agent) {
    const response = await invoke<Agent>('create_agent', { agent });
    this.agents = [...this.agents, response];
    return response;
  }
  
  async executeWorkflow(workflow: Workflow) {
    this.activeWorkflow = workflow;
    return await invoke('execute_workflow', { workflow });
  }
}

export const agentManager = new AgentManager(); 