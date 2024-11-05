import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import path from 'path';
import { CfnOutput, aws_lambda } from 'aws-cdk-lib';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const webServer = new aws_lambda.Function(this, "web_server", {
      memorySize: 128,
      runtime: aws_lambda.Runtime.PROVIDED_AL2,
      handler: "invalid",
      functionName: "rust-web-server",
      architecture: aws_lambda.Architecture.ARM_64,
      code: aws_lambda.Code.fromAsset(path.join(__dirname, "..", "..", "target/lambda/backend/bootstrap.zip")),
    });
    const fnUrl = webServer.addFunctionUrl({
      authType: aws_lambda.FunctionUrlAuthType.NONE,
    });

    new CfnOutput(this, 'TheUrl', {
      // The .url attributes will return the unique Function URL
      value: fnUrl.url,
    });
  }
}
