import sys

SCORE = {
    'A X': 4, 
    'A Y': 8, 
    'A Z': 3, 
    'B X': 1, 
    'B Y': 5, 
    'B Z': 9, 
    'C X': 7, 
    'C Y': 2, 
    'C Z': 6
}

MATCH = {
    'X': -1,
    'Y': 0,
    'Z': 1
}

puzzle = sys.stdin.read().strip().splitlines()

print('part 1:', sum((SCORE[r] for r in puzzle)))
print('part 2:', sum(((MATCH[r[-1]] + 1) * 3 + (('ABC'.find(r[0]) + MATCH[r[-1]]) % 3 + 1) for r in puzzle)))
