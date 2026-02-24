import sys

puzzle = [tuple([tuple(map(int, mm.split('-'))) for mm in ab]) for ab in (r.strip().split(',') for r in sys.stdin.read().strip().splitlines())]

print('part 1:', len(list(filter(lambda ab: ((ab[0][0] >= ab[1][0]) and (ab[0][1] <= ab[1][1])) or ((ab[1][0] >= ab[0][0]) and (ab[1][1] <= ab[0][1])), puzzle))))
print('part 2:', len(list(filter(lambda ab: (ab[0][0] <= ab[1][1]) and (ab[0][1] >= ab[1][0]), puzzle))))
