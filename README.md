# Poc of aws lambda using rust
This project is an attempt to proof concept of building serverless application in Rust programming language.

## Dependencies:
Project relies on **rust** itself (which comes with **cargo**, its package manager and command tool), **cargo lambda** crate, **AWS CLI** and **AWS SAM** for setting up local environment and deployment, **docker** for building local images of app for *aws sam* to work as well as local development DB.

### Rust
Install [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html), by running this command in your terminal:
```bash
$ curl https://sh.rustup.rs -sSf | sh
```
Just follow CLI helper and install everything with default settings. After verify you have installed:
```bash
$ which rustc
/Users/victorshinkevich/.cargo/bin/rustc
$ rustc --version
rustc 1.67.1 (d5a82bbd2 2023-02-07)
$ which cargo
/Users/victorshinkevich/.cargo/bin/cargo
$ cargo --version
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


### Docker
For docker installation please follow [official website](https://docs.docker.com/desktop/install/mac-install/).
You will need 
- docker desktop
- docker compose plugin

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
/usr/local/bin/aws 
$ aws --version
aws-cli/2.10.4 Python/3.9.11 Darwin/22.3.0 exe/x86_64 prompt/off
```
**Note**: output will be different depending on your OS and CPU architecture.

### AWS SAM CLI
#### MacOS
For Mac users AWS SAM CLI is available via homebrew.
```bash
$ brew install aws/tap/aws-sam-cli
```
1. Install rust
2. Install cargo
3. Install cargo lambda
4. Install docker
5. Install aws cli
6. Install sam cli
7. Get yourself a from aws web console
8. Configure aws with profile `aws-rust-poc`

First the lambda should be built. After it is build, our options are to invoke specific functions locally, start local web server to access lambdas, run validations of the SAM template or deploy lambda to the stand.
### To build: 
```bash
make build
```
### To run invoke specific lambda locally
```
sam local invoke <LambdaName>
```
```bash
sam local invoke PingLambda
```

### Start local webserver
```bash
sam local start-api
```
http://localhost:3000/ping

### Deploy to specific stage
In order to deploy we need to make sure the SAM template is valid.
To do so we need to run validation command and pass the profile which would be used for validation of CloudFormation.
```bash
sam validate --profile sam-cli-poc --lint
```
After fixing errors and linting warnings we can start deployment. Deployment goes with passing the stand file that includes configurations.
```
sam deploy --config-file samconfig-<stand-name>.toml
```
#### Example: deploy to staging

```bash
sam deploy --config-file samconfig-staging.toml
```

#### Example: deploy to prod

```bash
sam deploy --config-file samconfig-prod.toml
```