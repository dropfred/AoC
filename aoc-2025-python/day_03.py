import sys

def solve(puzzle, size):
    def solve(bs):
        j, p, m = 0, 0, len(bs) - size + 1
        for n in range(size):
            for b in range(p + 1, m + n):
                if bs[p] == '9': break
                if bs[b] > bs[p]: p = b
            j = j * 10 + int(bs[p])
            p += 1
        return j

    return sum(map(solve, puzzle))

puzzle = tuple(s.strip() for s in sys.stdin.read().strip().splitlines())

print('part 1:', solve(puzzle, 2))
print('part 2:', solve(puzzle, 12))
