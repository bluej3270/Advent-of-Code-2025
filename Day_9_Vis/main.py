from pathlib import Path
import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle

# Load polygon points and save to array
path = Path("input.txt")
points = []

with open(path, "r") as f:
    for line in f:
        line = line.strip()
        if line:
            x, y = map(int, line.split(","))
            points.append((x, y))

xs = [p[0] for p in points]
ys = [p[1] for p in points]

# Rectangle corners (Change manually)
x1, y1 = 5639, 68743
x2, y2 = 94532, 50249

# Normalize rectangle coordinates
left = min(x1, x2)
bottom = min(y1, y2)
width = abs(x2 - x1)
height = abs(y2 - y1)

# Plot
fig, ax = plt.subplots(figsize=(10, 10))

# Original bounded area
ax.fill(xs, ys, alpha=0.4)
ax.plot(xs, ys, linewidth=1)

# Overlay rectangle
rect = Rectangle((left, bottom), width, height, fill=False, linewidth=3)
ax.add_patch(rect)

# Mark rectangle corners
ax.scatter([x1, x2], [y1, y2], s=40)

ax.set_aspect('equal', adjustable='box')
ax.set_title("AoC 2025 Day 9 Part 2 Area with Overlay Rectangle")
ax.set_xlabel("X")
ax.set_ylabel("Y")

# Save graph
output_path = "aoc_overlay_rectangle.png"
plt.savefig(output_path, bbox_inches="tight", dpi=300)
plt.close(fig)

print(f"Saved overlay visualization to: {output_path}")

