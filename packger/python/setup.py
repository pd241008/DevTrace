from setuptools import setup, find_packages

setup(
    name="devtrace",
    version="0.1.1",
    author="pd241008",
    description="DevTrace - Distributed Developer Observability Engine (Python Wrapper)",
    long_description=open("README.md").read() if open("README.md") else "",
    long_description_content_type="text/markdown",
    packages=find_packages(),
    python_requires=">=3.7",
    entry_points={
        "console_scripts": [
            "devtrace=devtrace.__main__:main",
        ],
    },
)
