import random

text = ""
with open("src/bear.txt", "r", encoding="utf8", errors="ignore") as f:
    text = f.read()
    

para = []

for p in text.split("\n\n"):
    if len(p) > 150 and len(p) < 240:
        para.append(p.replace("\n", "; "))


with open("src/book_para.txt", "w", encoding="utf8") as f:
    f.write("\n".join(para))

print(random.choice(para))
print("\n\n")
print(random.choice(para))
print("\n\n")
print(random.choice(para))
print("\n\n")
print(random.choice(para))
print("\n\n")