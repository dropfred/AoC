import sys

START = 50

def part_1(puzzle):
    code = 0
    distance = START
    for d in puzzle:
        distance = (distance + d) % 100
        if distance == 0: code += 1
    return code

def part_2(puzzle):
    code = 0
    distance = START
    for d in puzzle:
        pp, distance = distance, (distance + d) % 100
        code += abs(d) // 100
        if (distance == 0) or (pp != 0) and ((d < 0 and distance > pp) or (d > 0 and distance < pp)):
            code += 1
    return code

puzzle = tuple(
    int(dd[1:]) * (1 if dd[0] == 'R' else -1)
    for dd in sys.stdin.read().strip().splitlines()
)

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
