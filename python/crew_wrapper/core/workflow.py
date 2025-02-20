from crewai import Task, Crew, Process
from .agent import create_agent

def execute_task(model_name: str, role: str, goal: str, task_description: str, backstory: str = "") -> str:
    """Execute a single task with one agent and return the result."""
    # Create agent with Ollama model
    agent = create_agent(model_name, role, goal, backstory)
    
    # Create task
    task = Task(
        description=task_description,
        agent=agent
    )
    
    # Create and execute crew
    crew = Crew(
        agents=[agent],
        tasks=[task],
        process=Process.sequential
    )
    
    return crew.kickoff() 