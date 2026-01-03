import sys

def part_1(puzzle):
    total = 0
    for b, e in puzzle:
        for i in range(b, e + 1):
            sn = str(i)
            ss = len(sn)
            if (ss & 1) != 0: continue
            ss //= 2
            if sn[:ss] == sn[ss:]: total += i
    return total

def part_2(puzzle):
    total = 0
    for b, e in puzzle:
        for n in range(b, e + 1):
            sn = str(n)
            ss = len(sn)
            for s in range(1, ss // 2 + 1):
                if ss % s != 0: continue
                if sn[:s] * (ss // s) == sn: total += n; break
    return total

puzzle = [tuple(map(int, be.split('-'))) for be in sys.stdin.read().strip().split(',')]

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
