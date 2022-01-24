from robyn import Robyn, jsonify
from transformers import pipeline
import time
import json

classifier = pipeline("text-classification", device=-1)
# classifier = pipeline("text-classification", device=0)
app = Robyn(__file__)


@app.post("/age")
async def age(request):
    start_time = time.time()
    body = json.loads(bytearray(request["body"]).decode("utf-8"))
    print(body["inputs"])
    print(classifier)
    res = classifier(body["inputs"])
    print(res)
    # print(f"Time taken: {(time.time() - start_time) * 1000}")
    # return jsonify(res)


# async def some_startup_task():
#     pass


app.start(port=8080, url="0.0.0.0")  # url is optional, defaults to 127.0.0.1
