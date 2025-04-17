# Project Automation Tool

## Overview
This script automates the scaffolding of new projects by setting up directories, initializing git repositories, and preparing various technology stacks according to the user's specifications. It supports multiple programming languages and frameworks, including Python, Node.js, Java, Ruby, Go, PHP, Swift, and Rust, as well as popular front-end frameworks like React, Angular, Vue, Ember, and Svelte. The script also supports database initialization for SQLite, PostgreSQL, and MySQL.

## Features
- **Project Directory Creation**: Automatically creates a new project directory at the specified location.
- **Technology Stack Initialization**: Sets up project components based on selected technologies such as Django, Flask, FastAPI, Rust, and more.
- **Database Setup**: Supports SQLite, PostgreSQL, and MySQL with options to configure via Docker containers.
- **Git Integration**: Initializes a git repository, adds all files, and makes an initial commit.
- **Environment Setup**: Configures Python virtual environments using Pyenv and handles dependency installations.

## Usage
To use the script, run it and follow the on-screen prompts to specify the project name, location, components, and technologies you want to include in your project.

### Commands
- `Project name:` Enter the name of your project.
- `Base path:` Specify the path where the project should be created; defaults to the current working directory.
- `Component:` Specify different parts of the project like frontend, backend, etc., and choose the technology for each component.
- `Add DB:` Choose a database from SQLite, PostgreSQL, and MySQL or skip this step by leaving it blank.

## Customization
The script uses dictionaries to manage executable commands and gitignore templates which can be customized to include additional technologies or to change existing configurations.

## Requirements
- Python 3
- Access to terminal or command prompt
- Necessary permissions to install software and run scripts on your system

Make sure all dependencies like Pyenv, Docker, and the executables for your chosen technologies are installed before running the script to ensure everything works smoothly.

## Contributing
Contributions to enhance the script or expand its functionality are welcome. Please feel free to fork the repository, make changes, and submit a pull request.

## License
This script is released under an open-source license.
