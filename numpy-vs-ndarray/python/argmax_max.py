import time
import numpy as np

def test_max(shape):
    a = np.random.rand(*shape)
    st = time.time()
    np.amax(a)
    return (time.time() - st)*1000*1000

def test_argmax(shape):
    a = np.random.rand(*shape)
    st = time.time()
    np.argmax(a)
    return (time.time() - st)*1000*1000

def test(shape):
    print(shape)
    times=[]
    for i in range(1000):
        times.append(test_max(shape))
    print(f"Average time for max: {sum(times) / len(times)}µs")

    times = [] 
    print(shape)
    for i in range(1000):
        times.append(test_argmax(shape))
    print(f"Average time for argmax: {sum(times) / len(times)}µs")

# test((1,512,512))
# test((1,128,128))
# test((1,12,128))
test((1,2,128))
# test((1,1,128))
# test((1,12))