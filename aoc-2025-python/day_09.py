import sys
from itertools import combinations, chain, pairwise

def size(a, b):
    ax, ay = a
    bx, by = b
    dx, dy = abs(bx - ax) + 1, abs(by - ay) + 1
    return (dx * dy)

def part_1(puzzle):
    return max(size(a, b) for a, b in combinations(puzzle, 2))

def part_2(puzzle):
    def valid(a, b):
        xa, ya = a
        xb, yb = b
        if xa > xb: xa, xb = xb, xa
        if ya > yb: ya, yb = yb, ya
        def valid(i, j):
            xi, yi = i
            xj, yj = j
            if xi > xj: xi, xj = xj, xi
            if yi > yj: yi, yj = yj, yi
            return xj <= xa or xi >= xb or yj <= ya or yi >= yb
        for i, j in pairwise(chain(puzzle, puzzle[:1])):
            if not valid(i, j): return False
        return True

    area = 0
    for a, b in combinations(puzzle, 2):
        if (s := size(a, b)) > area and valid(a, b):
            area = s
    return area

puzzle = tuple(tuple(map(int, r.split(','))) for r in sys.stdin.read().strip().splitlines())

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
