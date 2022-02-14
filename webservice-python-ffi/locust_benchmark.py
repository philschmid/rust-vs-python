from locust import HttpUser, task


class LoadTest(HttpUser):
    @task
    def predict(self):
        self.client.post("/age", json={"inputs": "I love you. I like you. I am your friend."})
