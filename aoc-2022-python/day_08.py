import sys

def part_1(puzzle):
    w, h = len(puzzle[0]), len(puzzle)
    vs = 0
    for y in range(1, h - 1):
        for x in range(1, w - 1):
            v = False
            for xx in range(0, x):
                if puzzle[y][xx] >= puzzle[y][x]: break
            else: v = True
            if not v:
                for xx in range(x + 1, w):
                    if puzzle[y][xx] >= puzzle[y][x]: break
                else: v = True
            if not v:
                for yy in range(0, y):
                    if puzzle[yy][x] >= puzzle[y][x]: break
                else: v = True
            if not v:
                for yy in range(y + 1, h):
                    if puzzle[yy][x] >= puzzle[y][x]: break
                else: v = True
            if v:
                vs += 1
    return vs + (w + h - 2) * 2

def part_2(puzzle):
    w, h = len(puzzle[0]), len(puzzle)
    mv = 0
    for y in range(1, h - 1):
        for x in range(1, w - 1):
            v = 1
            for xx in range(x - 1, -1, -1):
                if puzzle[y][xx] >= puzzle[y][x]: break
            v *= x - xx
            for xx in range(x + 1, w):
                if puzzle[y][xx] >= puzzle[y][x]: break
            v *= xx - x
            for yy in range(y - 1, -1, -1):
                if puzzle[yy][x] >= puzzle[y][x]: break
            v *= y - yy
            for yy in range(y + 1, h):
                if puzzle[yy][x] >= puzzle[y][x]: break
            v *= yy - y
            if v > mv: mv = v
    return mv

puzzle = [[*r] for r in sys.stdin.read().strip().splitlines()]

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))

