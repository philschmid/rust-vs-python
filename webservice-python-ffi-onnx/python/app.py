from starlette.applications import Starlette
from starlette.responses import JSONResponse
from starlette.routing import Route
from transformers import AutoTokenizer
import time
import onnxruntime as ort
import numpy as np

# classifier = pipeline("text-classification", device=0)
ort_session = ort.InferenceSession("pt_model.onnx", providers=["CPUExecutionProvider"])
tokenizer = AutoTokenizer.from_pretrained("distilbert-base-uncased-finetuned-sst-2-english")


def softmax(_outputs):
    maxes = np.max(_outputs, axis=-1, keepdims=True)
    shifted_exp = np.exp(_outputs - maxes)
    return shifted_exp / shifted_exp.sum(axis=-1, keepdims=True)


async def age(request):
    body = await request.json()
    start_time = time.time()
    onnx_inputs = tokenizer(body["inputs"], return_tensors="np")
    onnx_outputs = ort_session.run(None, onnx_inputs.data)[0]
    scores = softmax(onnx_outputs)
    res = {"label": scores.argmax().item(), "score": scores.max().item()}
    return JSONResponse(res)


async def some_startup_task():
    pass


app = Starlette(
    debug=False,
    routes=[
        Route("/age", age, methods=["POST"]),
    ],
)
