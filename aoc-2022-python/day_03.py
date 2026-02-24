import sys

def part_1(puzzle):
    def priority(r):
        h = len(r) // 2
        c = tuple(set(r[:h]) & set(r[h:]))[0]
        return ord(c) - ord('a') + 1 if c.islower() else ord(c) - ord('A') + 27
    return sum(map(priority, puzzle))

def part_2(puzzle):
    def priority(r):
        c = tuple(set(puzzle[r]) & set(puzzle[r + 1]) & set(puzzle[r + 2]))[0]
        return ord(c) - ord('a') + 1 if c.islower() else ord(c) - ord('A') + 27
    return sum(map(priority, range(0, len(puzzle), 3)))

puzzle = sys.stdin.read().strip().splitlines()

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
