import sys
import re
from itertools import permutations, combinations
import math

START = 'AA'
TIME = 30
LEARN = 4

def part_1(puzzle):
    # compute actual paths as well for drawing purpose
    def solve(puzzle, time):
        # distances between every pairs (floyd-warshall)
        # https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
        distances = {(v, n): 0 if v == n else 1 if n in vs else math.inf for n in puzzle for v, (_, vs) in puzzle.items()}
        for k, i, j in permutations(puzzle, 3): distances[i, j] = min(distances[i, j], distances[i, k] + distances[k, j])

        # skip null valves
        valves = [(v, r) for v, (r, _) in puzzle.items() if r > 0]

        paths = [(START, time, 0, [])]
        while paths:
            v, t, p, vs = paths.pop()
            d = len(paths)
            for nv, nr in valves:
                nt = t - (distances[v, nv] + 1)
                if (nt > 0) and (nv not in vs):
                    paths.append((nv, nt, p + nt * nr, vs + [nv]))
            if len(paths) == d: yield((p, vs))

    return max(solve(puzzle, TIME))[0]

def part_2(puzzle):
    distances = {(v, n): 0 if v == n else 1 if n in vs else math.inf for n in puzzle for v, (_, vs) in puzzle.items()}
    for k, i, j in permutations(puzzle, 3): distances[i, j] = min(distances[i, j], distances[i, k] + distances[k, j])
    # names as flags for easier sets operations (and presumably more efficient)
    valves = [(v, r, 1 << i) for i, (v, r) in enumerate((v, r) for v, (r, _) in puzzle.items() if r > 0)]
    best = {}
    paths = [(START, TIME - LEARN, 0, 0)]
    while paths:
        v, t, p, vs = paths.pop()
        for nv, nr, nf in valves:
            if ((nt := t - distances[v, nv] - 1) > 0) and ((vs & nf) == 0):
                paths.append((nv, nt, p + nt * nr, vs | nf))
        best[vs] = max(best.get(vs, 0), p)
    return max(mp + ep for (mvs, mp), (evs, ep) in combinations(best.items(), 2) if (mvs & evs) == 0)

pattern = re.compile(r'Valve (?P<name>[A-Z]{2}) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<valves>[A-Z]{2}(, [A-Z]{2})*)')
puzzle = {
    m.group('name'): (int(m.group('rate')), tuple(m.group('valves').split(', ')))
    for r in sys.stdin.read().strip().splitlines() if (m := pattern.match(r))
}

# import os
# with open('16.dot', 'w') as f:
#     f.write('digraph {\n')
#     f.write('  rankdir="LR"\n')
#     f.write('  node[shape="circle" width=0.75 fixedsize=true shape="circle"]\n')
#     f.write('  AA [shape="doublecircle" style="filled" fillcolor="red"]\n')
#     for n, (r, _) in puzzle.items():
#         f.write(f'  {n} [label="{n} ({r})"]\n')
#     for n, (_, ns) in puzzle.items():
#         f.write(f'  {n} -> {{{" ".join(ns)}}}\n')
#     f.write('}')
# os.system('dot -Tsvg 16.dot >16.svg')

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
