# merge three sorted arrays

from random import randrange

def gen_arrays():
    return (
            sorted([randrange(1, 10) for _ in range(10)]),
            sorted([randrange(1, 10) for _ in range(10)]),
            sorted([randrange(1, 10) for _ in range(10)])
            )

arrays = gen_arrays()

def merge_three_sorted(arrays):
    transposed = zip(*arrays)
    for i in transposed:
        # take the minimum
        # and add to a heap
        # and then keep adding them in?
        print(i)

merge_three_sorted(arrays)
