# Deployment IaC

This project deploys the AWS infrastructure using CDK. It is designed to be as cheap as possible
running small lambdas with the FE just a hosted S3 bucket. No API gateway as the goal is to only use free tier services.

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `cdk deploy`      deploy this stack to your default AWS account/region
* `cdk diff`        compare deployed stack with current state
* `cdk synth`       emits the synthesized CloudFormation template
