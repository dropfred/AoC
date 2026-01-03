import sys
from math import prod

# don't care about part 2 right-to-left order (add and mul are commutative)
def solve(puzzle, dir):
    def numbers(p):
        pos, _, size = p
        return map(int, (r[pos:pos + size] for r in puzzle[:-1]) if dir == 'H' else (''.join(r[pos + i] for r in puzzle[:-1]) for i in range(size)))
    ps = [p for p in enumerate(puzzle[-1]) if p[1] != ' ']
    ps = [(z[0][0], z[0][1], (z[1][0] - 1) - z[0][0]) for z in zip(ps, ps[1:])] + [(ps[-1][0], ps[-1][1], len(puzzle[-1]) - ps[-1][0])]
    return sum((sum if p[1] == '+' else prod)(numbers(p)) for p in ps)

puzzle = list(sys.stdin.read().splitlines())

print('part 1:', solve(puzzle, 'H'))
print('part 2:', solve(puzzle, 'V'))
