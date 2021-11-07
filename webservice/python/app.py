from starlette.applications import Starlette
from starlette.responses import JSONResponse
from starlette.routing import Route


async def age(request):
    body = await request.json()
    born = 2021 - body["age"]
    res = {"message": f"{body['name']} was born in {born}", "born": born}
    return JSONResponse(res)


app = Starlette(
    debug=False,
    routes=[
        Route("/age", age, methods=["POST"]),
    ],
)
