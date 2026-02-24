import sys
from copy import deepcopy
from itertools import pairwise, chain

SOURCE = 500

def solve(puzzle):
    #      o       ^
    #     ooo      |
    #    ooooo     n  n^2
    #      :       |
    # ..ooooooo..  v

    paths = list(puzzle)
    box = (
        (min(min(x for x, _ in p) for p in paths), min(min(y for _, y in p) for p in paths)),
        (max(max(x for x, _ in p) for p in paths), max(max(y for _, y in p) for p in paths))
    )
    ox = box[0][0]

    grid = [list('.'  * (box[1][0] - box[0][0] + 1)) for _ in range(box[1][1] + 1)]
    for line in paths:
        for a, b in pairwise(line):
            ax, ay = a
            bx, by = b
            if ax > bx: ax, bx = bx, ax
            if ay > by: ay, by = by, ay
            if ax == bx:
                for y in range(ay, by + 1):
                    grid[y][ax - ox] = '#'
            else:
                for x in range(ax, bx + 1):
                    grid[ay][x - ox] = '#'
    w, h = len(grid[0]), len(grid)

    def solve():
        n = 0
        while True:
            x, y = SOURCE - ox, 0
            if grid[y][x] == 'o': break
            while True:
                ny = y + 1
                if ny == h: return n
                if grid[ny][x] == '.': y += 1
                elif x == 0: return n
                elif grid[ny][x - 1] == '.':
                        x -= 1
                        y += 1
                elif x + 1 == w: return n
                elif grid[ny][x + 1] == '.':
                        x += 1
                        y += 1
                else:
                    grid[y][x] = 'o'
                    break
            n += 1

        return n
    
    n = solve()

    return n

def part_1(puzzle):
     return solve(puzzle)

def part_2(puzzle):
    h = max(max(y for _, y in p) for p in puzzle) + 2
    f = (SOURCE - h, h), (SOURCE + h, h)
    return solve(chain(puzzle, (f,)))

puzzle = tuple(
    tuple((int(x), int(y)) for x, y in (p.split(',') for p in r.strip().split(' -> ')))
    for r in sys.stdin.read().strip().splitlines()
)

print(part_1(puzzle))
print(part_2(puzzle))
