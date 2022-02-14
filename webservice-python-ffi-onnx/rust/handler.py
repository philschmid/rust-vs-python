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


def handle(input):
    onnx_inputs = tokenizer(input, return_tensors="np")
    onnx_outputs = ort_session.run(None, onnx_inputs.data)[0]
    scores = softmax(onnx_outputs)
    res = {"label": f"{scores.argmax().item()}", "score": scores.max().item()}
    return res
