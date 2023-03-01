# Poc of aws lambda using rust

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