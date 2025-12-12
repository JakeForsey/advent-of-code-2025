from collections import defaultdict
from functools import lru_cache

with open("input/11.txt", "r") as f:
    data = f.read()

edges = defaultdict(list)
for line in data.splitlines():
    src, dsts = line.split(": ")
    for dst in dsts.split(" "):
        edges[src].append(dst)

start = "you"
end = "out"
paths = []
queue = [[start]]
while queue:
    path = queue.pop()
    if path[-1] == end:
        paths.append(path)
        continue
    for dst in edges[path[-1]]:
        new_path = path[:]
        new_path.append(dst)
        queue.append(new_path)

print(f"part1: {len(paths)}")

@lru_cache
def expand(pos, seen_dac, seen_fft) -> int:
    if pos == "out":
        return 1 if seen_dac and seen_fft else 0
    elif pos == "dac":
        seen_dac = True
    elif pos == "fft":
        seen_fft = True
    
    total = 0
    for dst in edges[pos]:
        total += expand(dst, seen_dac, seen_fft)
    
    return total

print(f"part2: {expand("svr", False, False)}")
