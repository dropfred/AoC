import sys
import json
from functools import cmp_to_key

def compare(a, b):
    if type(a) is int and type(b) is int: return a < b
    if type(a) is list and type(b) is list:
        for aa, bb in zip(a, b):
            if compare(aa, bb): return True
            elif compare(bb, aa): return False
        return len(a) < len(b)
    return compare(
        a if type(a) is list else [a],
        b if type(b) is list else [b],
    )

def sort(a, b):
    if type(a) is int and type(b) is int: return -1
    if type(a) is list and type(b) is list:
        for aa, bb in zip(a, b):
            if compare(aa, bb): return -1
            elif compare(bb, aa): return 1
        return -1 if len(a) < len(b) else 0
    return sort(
        a if type(a) is list else [a],
        b if type(b) is list else [b],
    )

def part_1(puzzle):
    return sum(i + 1 for i, ab in enumerate(puzzle) if compare(*ab))

def part_2(puzzle):
    divs = [[[2]], [[6]]]
    ps = sorted((p for ab in puzzle + [divs] for p in ab), key=cmp_to_key(sort))
    return (ps.index(divs[0]) + 1) * (ps.index(divs[1]) + 1)

puzzle = [
    [json.loads(xs) for lst in p.splitlines() if (xs := lst.strip())]
    for p in '\n'.join(sys.stdin.read().strip().splitlines()).split('\n\n')
]

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
