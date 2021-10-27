import os
import boto3
import json

client = boto3.client("sagemaker-runtime")


def handler(event, context):

    body = json.loads(event["body"])

    return {"statusCode": 200, "body": json.loads(response["Body"].read())}
