import sys

def merge(ranges):
    rs = []
    for b, e in sorted(ranges):
        if (len(rs) == 0) or (b > rs[-1][1]): rs.append((b, e))
        else: rs[-1] = (rs[-1][0], max(rs[-1][1], e))
    return rs

fresh, ids = '\n'.join(sys.stdin.read().strip().splitlines()).split('\n\n')
fresh = merge(tuple(map(int, f.split('-'))) for f in fresh.splitlines())
ids = list(map(int, ids.splitlines()))

print('part 1:', sum(1 if i >= b and i <= e else 0 for i in ids for b, e in fresh))
print('part 2:', sum(e - b + 1 for b, e in fresh))
