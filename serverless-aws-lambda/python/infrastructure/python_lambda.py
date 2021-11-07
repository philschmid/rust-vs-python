from aws_cdk import core as cdk
import os
import subprocess
import shutil

# For consistency with other languages, `cdk` is the preferred import name for
# the CDK's core module.  The following line also imports it as `core` for use
# with examples from the CDK Developer's Guide, which are in the process of
# being updated to use `cdk`.  You may delete this import if you don't need it.
from aws_cdk import (
    aws_lambda as lambda_,
    aws_apigateway as _apigw,
)


class PythonLambdaStack(cdk.Stack):
    def __init__(
        self, scope: cdk.Construct, construct_id: str, target_architecture=lambda_.Architecture.X86_64, **kwargs
    ) -> None:
        super().__init__(scope, construct_id, **kwargs)

        ##############################
        #       Lambda Function      #
        ##############################

        lambda_handler_path = os.path.join(os.getcwd(), "src")

        # create function
        lambda_fn = lambda_.Function(
            self,
            "python-function",
            code=lambda_.Code.from_asset(lambda_handler_path),
            handler="handler.handler",
            timeout=cdk.Duration.seconds(60),
            memory_size=128,
            runtime=lambda_.Runtime.PYTHON_3_8,
        )

        api = _apigw.LambdaRestApi(self, "api-gw", proxy=True, handler=lambda_fn)
