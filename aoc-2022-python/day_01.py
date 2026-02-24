import sys

puzzle = sys.stdin.read().strip()

solve = sorted((sum(map(int, cs.split())) for cs in '\n'.join(puzzle.splitlines()).split('\n\n')))

print('part 1:', solve[-1])
print('part 2:', sum(solve[-3:]))
