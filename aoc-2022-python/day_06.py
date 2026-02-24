import sys

def solve(puzzle, n):
    return next((i + n for i in range(len(puzzle) - n) if len(set(puzzle[i:i+n])) == n), None)

puzzle = sys.stdin.read().strip()

print('part 1: ', solve(puzzle, 4))
print('part 2: ', solve(puzzle, 14))
