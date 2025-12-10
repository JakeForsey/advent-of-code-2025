from collections import defaultdict

from z3 import Int, Optimize, sat # pip install z3-solver


with open("input/10.txt", "r") as f:
    lines = f.readlines()

total = 0
for line in lines:
    _, *buttons, target_counts = line.strip().split(" ")
    buttons = [tuple(map(int, button[1:-1].split(","))) for button in buttons]
    target_counts = tuple(map(int, target_counts[1:-1].split(",")))
    
    solver = Optimize()
    button_variables = [Int(i) for i in range(len(buttons))]
    for button_variable in button_variables:
        solver.add(button_variable >= 0)

    data = defaultdict(list)
    for button_variable, button in zip(button_variables, buttons):
        for machine_idx in button:
            data[machine_idx].append(button_variable)

    for machine_idx, target_count in enumerate(target_counts):
        solver.add(sum(data[machine_idx]) == target_count)

    solver.minimize(sum(button_variables))
    if solver.check() == sat:
        model = solver.model()
        cost = model.eval(sum(button_variables)).as_long()
        total += cost

print(f"part2: {total}")
