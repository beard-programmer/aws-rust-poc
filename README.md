# Poc of aws lambda using rust
This project is an attempt to proof concept of building serverless application in Rust programming language.

Clone this project
```bash
git clone git@github.com:runahr/aws-rust-poc.git
```

And then proceed with dependencies.

## Dependencies:
Project relies on **rust** itself (which comes with **cargo**, its package manager and command tool), **cargo lambda** crate, **AWS CLI** and **AWS SAM** for setting up local environment and deployment, **docker** for building local images of app for *aws sam* to work as well as local development DB.

### Rust
Install [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html), by running this command in your terminal:
```bash
curl https://sh.rustup.rs -sSf | sh
```
Just follow CLI helper and install everything with default settings. After verify you have installed:
```
which rustc
rustc --version
which cargo
cargo --version
```
### Cargo lambda
Next you need to install [cargo lambda](https://www.cargo-lambda.info/guide/what-is-cargo-lambda.html).
Here is command for installation via [homebrew](https://brew.sh/) package manager.
```bash
brew tap cargo-lambda/cargo-lambda
brew install cargo-lambda
```
Alternative ways to install cargo lambda can be found [here](https://www.cargo-lambda.info/guide/installation.html#with-nix).

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