# CrewAI Tauri UI Implementation Plan

## Overview

This document outlines the step-by-step implementation plan for creating a Tauri v2 + SvelteKit UI wrapper for the CrewAI Python package with Ollama integration. The plan is structured to allow incremental development and testing at each stage.

## Project Structure

```
src/
  lib/
    components/
      ui/               # Shadcn components
      agents/           # Agent-specific components
      workflows/        # Workflow management components
    server/
      python/          # Python integration layer
      ollama/          # Ollama integration layer
    types/             # TypeScript types
    stores/            # State management
    utils/             # Utility functions
  routes/              # SvelteKit routes
src-tauri/
  src/                 # Rust backend code
  sidecars/           # Python and Ollama binaries
python/
  crew_wrapper/       # Python package for CrewAI integration
```

## Phase 1: Project Setup and Basic Structure

### 1.1 Initial Setup

- [x] Create new Tauri + SvelteKit project
- [x] Configure TypeScript
- [x] Set up Shadcn UI
- [x] Configure project structure
- [x] Set up development environment

### 1.2 Basic Configuration

- [x] Set up basic routing
- [x] Create initial type definitions
- [x] Set up state management structure

## Phase 2: Core Backend Implementation

### 2.1 Python Integration

- [x] Create Python wrapper package structure
- [ ] Download crewai python binary
- [ ] Connect crewai to downloaded ollama models
- [ ] Implement basic IPC structure
- [ ] Set up error handling
- [ ] Create logging system

### 2.2 Tauri Backend

- [ ] Implement command handlers
- [ ] Set up file system operations
- [ ] Configure system tray
- [ ] Implement security measures

### 2.3 Ollama Integration

- [ ] Download ollama binary to allow model downlaods
- [ ] Implement model downloads in frontend. Include ollama 3.2 3b for now
- [ ] Implement model management page, to choose between downloaded models

## Phase 3: Frontend Development

### 3.1 Basic UI Components

- [x] Create layout components
- [ ] Implement navigation
- [x] Set up theme system
- [ ] Create basic forms

### 3.2 Agent Management UI

- [ ] Create agent configuration interface
- [ ] Implement agent list view
- [ ] Add agent creation flow
- [ ] Create agent editing interface

### 3.3 Workflow Management

- [ ] Create workflow builder interface
- [ ] Implement workflow visualization
- [ ] Add workflow execution controls
- [ ] Create workflow monitoring view

## Phase 4: Integration and Testing

### 4.1 Backend Integration

- [ ] Connect Python wrapper to UI
- [ ] Implement real-time updates
- [ ] Set up error handling
- [ ] Create recovery mechanisms

### 4.2 Testing

- [ ] Write unit tests
- [ ] Implement integration tests
- [ ] Create end-to-end tests
- [ ] Set up CI/CD pipeline

## Phase 5: Polish and Optimization

### 5.1 Performance

- [ ] Optimize Python communication
- [ ] Improve UI performance
- [ ] Implement caching
- [ ] Optimize memory usage

### 5.2 User Experience

- [ ] Add loading states
- [ ] Improve error messages
- [ ] Create help documentation
- [ ] Add keyboard shortcuts

## Technical Details

### TypeScript Types

```typescript
// Core Types
interface Agent {
  id: string;
  name: string;
  role: string;
  goal: string;
  backstory: string;
  tools: Tool[];
  model: OllamaModel;
}

interface OllamaModel {
  name: string;
  parameters: {
    temperature: number;
    maxTokens: number;
    topP: number;
  };
}

interface Tool {
  name: string;
  description: string;
  parameters: ToolParameter[];
}

interface Workflow {
  id: string;
  name: string;
  agents: Agent[];
  tasks: Task[];
  connections: Connection[];
}
```

### Tauri Commands

```rust
#[tauri::command]
async fn init_python_environment(app: tauri::AppHandle) -> Result<(), String> {
    // Python sidecar initialization
}

#[tauri::command]
async fn create_agent(agent: Agent) -> Result<Agent, String> {
    // Agent creation logic
}

#[tauri::command]
async fn execute_workflow(workflow: Workflow) -> Result<WorkflowExecution, String> {
    // Workflow execution logic
}
```

### State Management

```typescript
class AgentManager {
  agents = $state<Agent[]>([]);
  activeWorkflow = $state<Workflow | null>(null);

  async createAgent(agent: Agent) {
    const response = await invoke("create_agent", { agent });
    this.agents = [...this.agents, response];
  }

  async executeWorkflow(workflow: Workflow) {
    this.activeWorkflow = workflow;
    return await invoke("execute_workflow", { workflow });
  }
}
```

### Security Configuration

```typescript
const ALLOWED_MODELS = ["llama2", "mistral", "codellama"] as const;
type AllowedModel = (typeof ALLOWED_MODELS)[number];

interface SecurityConfig {
  allowedModels: AllowedModel[];
  maxConcurrentAgents: number;
  timeoutMs: number;
}
```

## Implementation Notes

1. Each phase should be completed and tested before moving to the next
2. Regular commits should be made with clear messages
3. Documentation should be updated as features are implemented
4. Security considerations should be reviewed at each phase
5. Performance testing should be done incrementally

## Getting Started

To begin implementation:

1. Clone the repository
2. Install dependencies:
   ```bash
   npm install
   ```
3. Set up development environment:
   ```bash
   npm run tauri dev
   ```
4. Follow the phases in order, checking off tasks as completed

## Next Steps

1. Configure Tauri for Python sidecar integration
2. Implement basic IPC structure
3. Set up Ollama connection
4. Create basic agent management UI

Remember to:

- Test each component thoroughly before moving on
- Document all APIs and interfaces
- Keep security in mind throughout development
- Maintain type safety across the application
