import json


def handler(event, context):
    print(event)
    body = json.loads(event["body"])

    born = 2021 - body["age"]
    res = {"message": f"{body['name']} was born in {born}", "born": born}
    print(res)
    return {"statusCode": 200, "body": json.dumps(res)}


# handler({"body": json.dumps({"name": "philipp", "age": 26})}, "")
