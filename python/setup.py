from setuptools import setup, find_packages

setup(
    name="crew_wrapper",
    version="0.1.0",
    packages=find_packages(),
    install_requires=[
        "crewai>=0.1.0",
        "requests>=2.31.0",
        "python-dotenv>=1.0.0",
        "fastapi>=0.109.0",
        "uvicorn>=0.27.0",
        "langchain-community>=0.0.10",
        "pydantic>=2.0.0"
    ],
) 