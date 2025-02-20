from crewai import Agent
from langchain_community.llms import Ollama

def create_agent(model_name: str, role: str, goal: str, backstory: str = "") -> Agent:
    """Create a CrewAI agent with Ollama LLM."""
    return Agent(
        role=role,
        goal=goal,
        backstory=backstory,
        llm=Ollama(model=model_name)
    ) 