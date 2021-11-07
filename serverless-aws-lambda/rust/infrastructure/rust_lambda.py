from aws_cdk import core as cdk
import os
import subprocess
import shutil

# For consistency with other languages, `cdk` is the preferred import name for
# the CDK's core module.  The following line also imports it as `core` for use
# with examples from the CDK Developer's Guide, which are in the process of
# being updated to use `cdk`.  You may delete this import if you don't need it.
from aws_cdk import (
    aws_iam as iam,
    aws_sagemaker as sagemaker,
    aws_lambda as lambda_,
    aws_apigateway as _apigw,
)
from aws_cdk.aws_logs import RetentionDays


class RustLambdaStack(cdk.Stack):
    def __init__(
        self, scope: cdk.Construct, construct_id: str, target_architecture=lambda_.Architecture.X86_64, **kwargs
    ) -> None:
        super().__init__(scope, construct_id, **kwargs)

        # target architecture
        target = "x86_64-unknown-linux-musl"

        # commands for building binary
        build_command = f"cargo build --release --target {target}"
        build_directory = os.path.join(os.getcwd(), "target", target, "release")
        # path to build rust binary
        lambda_handler_path = os.path.join(os.getcwd(), "target", target, "release", "lambda")

        # executing build
        output = subprocess.run(build_command.split(" "), capture_output=True)
        if "error:" in output.stderr.decode():
            raise ValueError(output.stderr.decode())
        os.makedirs(f"target/{target}/release/lambda", exist_ok=True)

        shutil.copy(os.path.join(build_directory, "bootstrap"), lambda_handler_path)
        ##############################
        #       Lambda Function      #
        ##############################

        # create function
        lambda_fn = lambda_.Function(
            self,
            "rust-function",
            code=lambda_.Code.from_asset(lambda_handler_path),
            timeout=cdk.Duration.seconds(60),
            runtime=lambda_.Runtime.PROVIDED_AL2,
            handler="not.required",
            memory_size=128,
        )

        api = _apigw.LambdaRestApi(self, "api-gw", proxy=True, handler=lambda_fn)
