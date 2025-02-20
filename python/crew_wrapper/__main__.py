from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from contextlib import asynccontextmanager
import uvicorn
import sys
import traceback
from .core.workflow import execute_task

app = FastAPI()

class TaskRequest(BaseModel):
    model_name: str
    role: str
    goal: str
    task: str
    backstory: str = ""

class ErrorResponse(BaseModel):
    detail: str
    traceback: str | None = None

@app.post("/execute")
async def execute_crew_task(request: TaskRequest):
    try:
        result = execute_task(
            model_name=request.model_name,
            role=request.role,
            goal=request.goal,
            task_description=request.task,
            backstory=request.backstory
        )
        return {"result": result}
    except Exception as e:
        print(f"Error executing task: {str(e)}")
        traceback.print_exc()
        raise HTTPException(
            status_code=500, 
            detail=ErrorResponse(
                detail=str(e),
                traceback=traceback.format_exc()
            ).model_dump()
        )

def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 3001
    print(f"Starting CrewAI service on port {port}")
    uvicorn.run(app, host="127.0.0.1", port=port, log_level="info")

if __name__ == "__main__":
    main() 