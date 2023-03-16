# Github Actions Handler

## Author

Htet Aung Hlaing

[DreamerChaserHAH (Htet Aung) (github.com)](https://github.com/DreamerChaserHAH)

## Description

Manually doing CI/CD on VPS servers are quite painful and unrealisticly slow. I want to change that. This is a custom Github Action Handler that would be called through a simple web request with the use of an additional cms system like [Cockpit (getcockpit.com)](https://getcockpit.com/)

## Usage

![envpng](https://user-images.githubusercontent.com/109950820/225601383-b47c1bdd-b955-4f4e-b271-013c51be513b.png)

there is a sample.env file contain the 2 variables that you can adjust

### PORT

This will be the port at which the server would be running on

### CMS_API_URL

This will be api endpoint to call to obtain the necessary json

**Make sure to have a ".env" instead of a "sample.env" to make the program runnable**

and then you can execute the function by visiting http://{hostname}:{your_port}?command={command_code},

