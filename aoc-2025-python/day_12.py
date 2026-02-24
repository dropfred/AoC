import sys

def part_1(puzzle):
    return sum((w // 3) * (h // 3) >= sum(ss) for (w, h), ss in (
        (tuple(map(int, a.split('x'))), list(map(int, ss.split())))
        for a, ss in (r.split(': ') for r in puzzle[-1].splitlines())
    ))

puzzle = '\n'.join(sys.stdin.read().strip().splitlines()).split('\n\n')

print('part 1:', part_1(puzzle))
