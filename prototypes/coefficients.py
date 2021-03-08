import math
import random

def bound_sqrt(x):
    candidate = math.sqrt(x)
    while candidate > 2:
        candidate /= 2
    return candidate

numbers = random.sample(list(map(bound_sqrt, range(500))), 12)
print(numbers)