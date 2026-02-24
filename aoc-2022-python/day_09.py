import sys

def solve(puzzle, n):
    rope = [[0, 0] for _ in range(n)]
    path = {(0, 0)}

    for d, s in (r.split() for r in puzzle):
        for i in range(int(s)):
            if   d == 'L': rope[0][0] -= 1
            elif d == 'R': rope[0][0] += 1
            elif d == 'U': rope[0][1] += 1
            elif d == 'D': rope[0][1] -= 1
            for i in range(1, len(rope)):
                dx = abs(rope[i - 1][0] - rope[i][0]) 
                dy = abs(rope[i - 1][1] - rope[i][1]) 
                dxy = (dx + dy) > 2
                if dxy or dx > 1:
                    rope[i][0] += 1 if rope[i - 1][0] > rope[i][0] else -1 if rope[i - 1][0] < rope[i][0] else 0
                if dxy or dy > 1:
                    rope[i][1] += 1 if rope[i - 1][1] > rope[i][1] else -1 if rope[i - 1][1] < rope[i][1] else 0
            path.add(tuple(rope[-1]))

    return len(path)

puzzle = sys.stdin.read().strip().splitlines()

print('part 1:', solve(puzzle, 2))
print('part 2:', solve(puzzle, 10))