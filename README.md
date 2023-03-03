# Poc of aws lambda using rust
This project is an attempt to proof concept of building serverless application in Rust programming language.

## Dependencies:
- [Rust](https://www.rust-lang.org/) (*v1.67.1*)
- [Cargo](https://doc.rust-lang.org/cargo/) (*v1.67.1*)
- [Cargo lambda](https://www.cargo-lambda.info/) (*v0.17.1*)
- [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-welcome.html) (*v2.10.4*)
- [AWS SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/what-is-sam.html) (*v1.75.0*)
- [Docker](https://www.docker.com/) (*v20.10.22*)

### Rust
Install Rust, by running this command in your terminal:
```bash
$ curl https://sh.rustup.rs -sSf | sh
```
Just follow CLI helper and install everything with default settings. It will install both Rust language and Cargo tool.
Verify installation:
```bash
$ which rustc
```
```
/Users/victorshinkevich/.cargo/bin/rustc
```
```bash
$ rustc --version
```
```
rustc 1.67.1 (d5a82bbd2 2023-02-07)
```
```bash
$ which cargo
```
```
/Users/victorshinkevich/.cargo/bin/cargo
```
```bash
$ cargo --version
```
```
cargo 1.67.1 (8ecd4f20a 2023-01-10)
```
### Cargo lambda
Next you need to install [cargo lambda](https://www.cargo-lambda.info/guide/what-is-cargo-lambda.html).
Here is command for installation via [homebrew](https://brew.sh/) package manager.
```bash
$ brew tap cargo-lambda/cargo-lambda
$ brew install cargo-lambda
```
Alternative ways to install cargo lambda can be found [here](https://www.cargo-lambda.info/guide/installation.html#with-nix).

### AWS
You need to have both [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) and [AWS SAM](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html).
### AWS CLI
#### MacOS
AWS CLI installer takes care of arm/x64 differences, just download and run installer.
```bash
$ curl "https://awscli.amazonaws.com/AWSCLIV2.pkg" -o "AWSCLIV2.pkg"
$ sudo installer -pkg AWSCLIV2.pkg -target /
```
Verify installation, it should be something like:
```bash
$ which aws
```
```
/usr/local/bin/aws 
```
```bash
$ aws --version
```
```
aws-cli/2.10.4 Python/3.9.11 Darwin/22.3.0 exe/x86_64 prompt/off
```
**Note**: output will be different depending on your OS and CPU architecture.

#### Configure AWS CLI
Log into aws web console, open IAM and create yourself a pair of keys: public `access key ID` and `secret access key`. Detailed instruction is found [here](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-prereqs.html).
Having those keys, configure yourself an aws profile. In scripts and configs we are using named profile `aws-rust-poc` Run the command to create it:
```bash
$ aws configure --profile aws-rust-poc
```
Enter `Access Key ID`, `AWS Secret Access Key`, `Default region name` and `Default output format`
You can verify profile successfully added by printing it in terminal:
```bash
$ cat ~/.aws/credentials
```
```
[aws-rust-poc]
aws_access_key_id = <YOUR_PUBLIC_ACCESS_KEY_ID>
aws_secret_access_key = <YOUR_PRIVATE_SECRET_ACCESS_KEY>
```

Now AWS CLI is configured, and we can proceed with installation of AWS SAM CLI.

### AWS SAM CLI
#### MacOS
For Mac users AWS SAM CLI is available via homebrew.
```bash
$ brew install aws/tap/aws-sam-cli
```
Verify installation.
```bash
$ which sam
```
```
/opt/homebrew/bin/sam
```
```bash
$ sam --version
```
```
SAM CLI, version 1.75.0
```

### Docker
For docker installation please follow [official website](https://docs.docker.com/desktop/install/mac-install/).
You will need
- docker desktop
- docker compose plugin

And we are done with dependencies!

## Building project
Before running or deploying a project we need to build it, meaning compile the code.
It can be done running make command:
```bash
$ make build
```
```
Finished release [optimized] target(s) in 53.96s
```

## Local environment and development
### Running DB
Prepare an .env file for docker compose running DB:
```bash
$ cp .env.dist .env
```
Run the docker compose in console in daemon mode:
```bash
$ docker compose up -d
```
List containers:
```bash
$ docker compose ps
```
To shut down container
```bash
$ docker compose down #
```
To shut down container and remove volumes - persisted databases and tables.
```bash
$ docker compose down -v
```

### Importing DB dump
Next you need to import dump from development/stating environment.
1. Download dump from bastion or ask SRE for it.
2. After receiving dump copy it to the container:
    ```bash
    $ docker cp <path_to_dump>/<dump-name>.sql aws-rust-poc-mysql_db-1:/
    ```
3. You will need a root password for mysql for import. It can be fetched from logs when it was generated during first `docker compose up -d` command.
    ```bash
   $ docker logs aws-rust-poc-mysql_db-1 2>&1 | grep GENERATED
   GENERATED ROOT PASSWORD: 2CB0A6A3eWgQHtJiVtRAMCd5r/XheLE1
   ```
   Copy this password to clilboard.
4. Open bash inside container and import DB using mysql client:
    ```bash
   $ docker exec -it aws-rust-poc-mysql_db-1 /bin/bash
   # Replace <ROOT_PASSWORD> with password you acquired from logs on the previous step.
   $ mysql -uroot -p<ROOT_PASSWORD> zuka < <name-of-dump>.sql
    ```
5. While you there, you could verify that import was successfull:
    ```bash
   $ mysql -uzuka -pzuka # Connect to db via mysql client.
    ```
   ```sql
   SHOW DATABASES;
   ```
   You should see `zuka` there.
    ```sql
   USE ZUKA;
   SHOW TABLES;
   ```
   You should see a list of tables.
### Running application
Serverless application consists of API gateway, which are http endpoinds and lambda functions that are being invoked (triggered) by API gateway and DB that is being used by functions.
SAM gives us ability to do both in local environment - invoking lambdas and running HTTP server that acts as API gateway.
For both we need to set up local env variables.
#### Set up local env variables file
```bash
$ cp .env.json.dist .env.sam.local.json
```
#### Invoke single lambda
General command looks like this
```
$ sam local invoke <LambdaName> --host localhost --port 9001 --env-vars .env.sam.local.json
```
List of lambdas can be found in `template.yaml` under `Resources` section with `**Lambda` name pattern.
For instance
```bash
$ sam local invoke PingLambda --env-vars .env.sam.local.json
$ sam local invoke PingDbLambda --env-vars .env.sam.local.json
```

#### Set up local server
```bash
$ sam local start-api --host localhost --port 9001 --env-vars .env.sam.local.json
```
This will start server on localhost listening 9001 port.
Routing from http server (gateway) to lambdas could be found in `template.yaml` under `Resources` -> `<LambdaName>` -> `Properties` -> `Events`.
For instance:
```bash
$ curl http://localhost:9001/ping
$ curl http://localhost:9001/ping-db
```


## Deployment
For deployment of the changes currently we use SAM CLI.
The flow looks like this:
1. Compile application
2. Validate SAM template
3. Deploy to required stage.

### Compile
```bash
make build
```
### Validate template
```bash
$ sam validate --profile aws-rust-poc --lint
```
If there are some errors or suggestions please fix them.
### Deploy
```
$ sam deploy --config-file samconfig-<stage-name>.toml
```
#### Staging
```bash
$ sam deploy --config-file samconfig-staging.toml
```
#### Prod
```bash
$ sam deploy --config-file samconfig-prod.toml
```