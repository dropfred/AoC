import sys
from functools import reduce, cache
from operator import xor, sub
from itertools import combinations
from math import inf

def presses(buttons):
    s = len(buttons)
    return (
        (bs, reduce(xor, (buttons[b] for b in bs), 0))
        for n in range(s + 1)
        for bs in combinations(range(s), n)
    )

def part_1(puzzle):
    def solve(machine):
        pattern, buttons, _ = machine
        return next((len(bs) for bs, p in presses(buttons) if p == pattern), inf)

    return sum(solve(m) for m in puzzle)

# https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory
def part_2(puzzle):
    def solve(machine):
        _, buttons, joltage = machine
        wires = tuple(tuple(1 if (1 << i) & ws else 0 for i in range(len(joltage))) for ws in buttons)
        zero = (0,) * len(joltage)
        patterns = {}
        for bs, p in presses(buttons):
            if p not in patterns: patterns[p] = []
            patterns[p].append(bs)
        step = lambda j, bs: tuple(map(
            lambda j: j // 2,
            reduce(lambda j, b: map(sub, j, wires[b]), bs, j)
        ))

        @cache
        def solve(joltage):
            if joltage == zero: return 0
            elif any(j < 0 for j in joltage): return inf
            else:
                p = sum((b & 1) << i for i, b in enumerate(joltage))
                if p == 0:
                    return solve(step(joltage, ())) * 2
                else:
                    return min(
                        (len(bs) + solve(step(joltage, bs)) * 2 for bs in patterns[p]),
                        default=inf
                    ) if p in patterns else inf

        return solve(joltage)

    return sum(solve(m) for m in puzzle)

puzzle = tuple(
    (
        int(''.join(map(lambda c: '1' if c == '#' else '0', reversed(p[1:-1]))), base=2),
        tuple(sum((1 << int(w) for w in ws)) for ws in (b[1:-1].split(',') for b in bs)),
        tuple(int(j) for j in j[1:-1].split(','))
    )
    for p, *bs, j in (r.split() for r in sys.stdin.read().strip().splitlines())
)

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
