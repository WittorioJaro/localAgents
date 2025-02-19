// Core Types
export interface Agent {
  id: string;
  name: string;
  role: string;
  goal: string;
  backstory: string;
  tools: Tool[];
  model: OllamaModel;
}

export interface OllamaModel {
  name: string;
  parameters: {
    temperature: number;
    maxTokens: number;
    topP: number;
  };
}

export interface Tool {
  name: string;
  description: string;
  parameters: ToolParameter[];
}

export interface ToolParameter {
  name: string;
  type: string;
  description: string;
  required: boolean;
}

export interface Workflow {
  id: string;
  name: string;
  agents: Agent[];
  tasks: Task[];
  connections: Connection[];
}

export interface Task {
  id: string;
  name: string;
  description: string;
  agent: Agent;
}

export interface Connection {
  id: string;
  source: string;
  target: string;
  type: ConnectionType;
}

export type ConnectionType = 'success' | 'failure' | 'always'; 