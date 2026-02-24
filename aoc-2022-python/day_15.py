import sys
import re

def part_1(puzzle):
    def empty(pairs, at_t):
        def merge(rs):
            m = []
            if rs:
                rs = sorted(rs)
                bm, em = rs[0]
                for i in range(1, len(rs)):
                    b, e = rs[i]
                    if b > em:
                        m.append((bm, em))
                        bm, em = b, e
                    elif e > em:
                        em = e
                m.append((bm, em))
            return m

        empty = []
        for (sx, sy), (bx, by) in pairs:
            d = abs(sx - bx) + abs(sy - by)
            dy = abs(sy - at_t)
            if dy <= d:
                i = abs(dy - d)
                be, ee = sx - i, sx + i
                if by == at_t:
                    if bx == be: be += 1
                    else       : ee -= 1
                if be <= ee: empty.append((be, ee))

        return merge(empty)

    return sum((e - b) + 1 for b, e in empty(puzzle, 2000000))

def part_2(puzzle):
    return 'TODO'

puzzle = [
    (p[:2], p[2:])
    for r in sys.stdin.read().strip().splitlines()
    for p in [tuple(int(c) for c in re.findall("-?\d+", r))]
]

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
