import sys
from math import prod, inf
from itertools import combinations

def part_1(puzzle):
    def distance2(a, b):
        ax, ay, az = puzzle[a]
        bx, by, bz = puzzle[b]
        dx, dy, dz = bx - ax, by - ay, bz - az
        return (dx ** 2 + dy ** 2 + dz ** 2)

    circuits = []
    def circuit(p): return next((c for c in circuits if p in c), None)

    for a, b in sorted(combinations(range(len(puzzle)), 2), key=lambda e: distance2(*e))[:1000]:
        ca, cb = circuit(a), circuit(b)
        if ca is None and cb is None: circuits.append({a, b})
        elif ca is None: cb.add(a)
        elif cb is None: ca.add(b)
        elif ca != cb: ca |= cb; circuits.remove(cb)
    
    return prod(sorted(map(len, circuits), reverse=True)[:3])

def part_2(puzzle):
    # https://en.wikipedia.org/wiki/Prim%27s_algorithm
    def prim(puzzle):
        def distance2(a, b):
            ax, ay, az = puzzle[a]
            bx, by, bz = puzzle[b]
            dx, dy, dz = bx - ax, by - ay, bz - az
            return (dx ** 2 + dy ** 2 + dz ** 2)

        js = set(range(len(puzzle)))
        es = [(None, inf)] * len(puzzle); es[0] = (None, 0)

        while js:
            j = min(js, key=lambda j: es[j][1])
            js.remove(j)
            for n in js:
                if (d := distance2(j, n)) < es[n][1]:
                    es[n] = ((j, n), d)

        return filter(lambda e: e[0], es)

    a, b = max(prim(puzzle), key=lambda e: e[1])[0]

    return (puzzle[a][0] * puzzle[b][0])

puzzle = tuple(tuple(map(int, r.split(','))) for r in sys.stdin.read().strip().splitlines())

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
