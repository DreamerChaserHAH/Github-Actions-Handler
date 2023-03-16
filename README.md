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

This will be api endpoint to call to obtain the necessary json.

If you look carefully, you will see that I provided the link in the form of

```
http://link.com?api={uniqueidentifier}  
```

the constant here is **uniqueidentifier**

make sure it is included in the .env file or the program will not run properly!

The api can be path based if you change it into this format

```
http://link.com/{uniqueidentifier}
```

**Make sure to have a ".env" instead of a "sample.env" to make the program runnable**

and then you can execute the function by visiting 
http://{hostname}:{port}?command={command_code}

#### hostname

If you are testing on your local computer it would be your

```
localhost
```

or if you are testing it on your VPS server, it would be your

```
{VPS IP}
```

you can also use your domain name if you have connected it to your VPS

#### port

The port at which the server is running on

#### command_code

Unique Identifier of the data that you are trying to obtain. This code will be appended directly into the **uniqueidentifier** field in the env file, so make sure the resulting api would make sense to the server!

### Return Values

There should be two values in the json that returned

1. ```
   execute_path
   ```

2. ```
   command
   ```

#### Execute Path

It would be the path where the executable is located

#### Command

It would be the command arguments that should be processed



## Crates Used

[runtime-format - crates.io: Rust Package Registry](https://crates.io/crates/runtime-format)

[actix-web - crates.io: Rust Package Registry](https://crates.io/crates/actix-web)


