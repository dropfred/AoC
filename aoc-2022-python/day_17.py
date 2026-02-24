import sys
# itertools.islice is super slow compared to basic slice O_o
# from itertools import cycle, islice
from itertools import cycle

shapes = ('####', '.#.\n###\n.#.', '..#\n..#\n###', '#\n#\n#\n#', '##\n##')
shapes = tuple(
    (
        tuple(reversed(tuple(map(lambda p: int(p.translate(t), base=2), ps)))),
        max(map(len, ps))
    )
    for s in shapes if (ps := s.splitlines()) and (t := str.maketrans('.#', '01'))
)

def stack(shapes, jets, rocks, n=1):
    h = len(rocks)
    ml = lambda s, n: tuple(p << n for p in s)
    mr = lambda s, n: tuple(p >> n for p in s)
    def j(s, rs = ()):
        m, w = (ml, 0b1000000) if next(jets) == '<'  else (mr, 0b0000001)
        return ns if all((p & w == 0 for p in s)) and \
                     (ns := m(s, 1)) and \
                     all((p & r == 0 for p, r in zip(ns, rs))) \
                  else s
    for _ in range(n):
        s, w = next(shapes)
        s = ml(s, 5 - w)
        for _ in range(3):
            s = j(s)
        y = len(rocks)
        while True:
            s = j(s, rocks[y:])
            if y == 0 or any(((p & r) != 0 for p, r in zip(s, rocks[y-1:]))): break
            y -= 1
        for p, y in zip(s, range(y, y + len(s))):
            if y < len(rocks): rocks[y] |= p
            else: rocks.append(p)
    
    return len(rocks) - h

def part_1(puzzle):
    return stack(cycle(shapes), cycle(puzzle), [], 2022)

def part_2(puzzle):
    ss, js, rs = cycle(shapes), cycle(puzzle), []
    # empirically found that a "hash" of height 50 is ok on my input
    n, hh, hs = 0, 50, {}
    while True:
        n += 1
        stack(ss, js, rs)
        if len(rs) >= hh:
            h = tuple(rs[-hh:])
            if h in hs: break
            hs[h] = (n, len(rs))
    n0, h0 = hs[h]
    cn, ch = n - n0, len(rs) - h0
    q, r = divmod(1000000000000 - n0, cn)
    rh = stack(ss, js, rs, r)

    return h0 + q * ch + rh

puzzle = sys.stdin.read().strip()

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
