import sys
from functools import cache

def part_1(puzzle):
    splits = 0
    beams = {puzzle[0].index('S')}
    for r in puzzle[1:]:
        bs = set()
        for b in beams:
            if r[b] == '^':
                splits += 1
                if b > 0: bs.add(b - 1)
                if b < len(r) - 1: bs.add(b + 1)
            else:
                bs.add(b)
        beams = bs
    return splits

def part_2(puzzle):
    w, h = len(puzzle[0]) - 1, len(puzzle) - 1

    @cache
    def part_2(r, c):
        if r == h: return 1
        ts = 0
        if puzzle[r][c] == '^':
            if c > 0: ts += part_2(r + 1, c - 1)
            if c < w: ts += part_2(r + 1, c + 1)
        else:
            ts = part_2(r + 1, c)
        return  ts
        
    return part_2(1, puzzle[0].index('S'))

puzzle = [r.strip() for r in sys.stdin.read().strip().splitlines()]

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
