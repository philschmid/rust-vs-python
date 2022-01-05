from starlette.applications import Starlette
from starlette.responses import JSONResponse
from starlette.routing import Route
from transformers import pipeline

classifier = pipeline("text-classification")


async def age(request):
    body = await request.json()
    res = classifier(body["inputs"])
    fake_res = [
        {"label": "NEGATIVE", "score": 0.0001},
        {"label": "POSITIVE", "score": 0.9999},
    ]
    return JSONResponse(res)


async def some_startup_task():
    pass


app = Starlette(
    debug=False,
    routes=[
        Route("/age", age, methods=["POST"]),
    ],
)
