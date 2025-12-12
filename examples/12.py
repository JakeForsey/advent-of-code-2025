
with open("input/12.txt") as f:
    data = f.read()

*present_strs, regions_str = data.split("\n\n")

presents = {}
for present_str in present_strs:
    idx, *row_strs = present_str.split("\n")
    idx = int(idx.replace(":", ""))
    coords = []
    for y, row_str in enumerate(row_strs):
        for x, c in enumerate(row_str):
            if c == "#":
                coords.append((x, y))
    presents[idx] = coords

count = 0
for region_str in regions_str.splitlines():
    shape, counts = region_str.split(": ")
    w, h = shape.split("x")
    w, h = int(w), int(h)
    area = w * h
    counts = list(map(int, counts.split(" ")))

    w3 = w - (w % 3)
    h3 = h - (h % 3)
    area3 = w3 * h3
    if sum(counts) * 9 <= area3:
        count += 1

print(f"part1: {count}")
