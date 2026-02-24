import sys

def part_1(puzzle):
    X, c, s = 1, 1, 0
    def cycle():
        nonlocal c, s
        if (c + 20) % 40 == 0: s += c * X
        c += 1
    for op in puzzle:
        if c > 220: break
        cycle()
        if op[0] == 'addx':
            cycle()
            X += int(op[1])
    return s

def part_2(puzzle):
    X, c, p = 1, 1, 0
    line = []
    def cycle():
        nonlocal c
        x = (c - 1) % 40
        print('X' if X - 1 <= x <= X + 1 else '    ' if c % 5 == 0 else ' ', end='\n' if c % 40 == 0 else '')
        c += 1
    for op in puzzle:
        cycle()
        if op[0] == 'addx':
            cycle()
            X += int(op[1])

puzzle = [r.split() for r in sys.stdin.read().strip().splitlines()]

print('part 1:', part_1(puzzle))
print('part 2:'); part_2(puzzle)
