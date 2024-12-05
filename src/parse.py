
text = ""
with open("src/output.txt", "r") as f:
    text = f.read()
text=text.replace("|", "\n")
text2 = ""
for t in text.split("\n"):
    if " " not in t and t.isalpha():
        text2 += t+"\n"
with open("src/output.txt", "w") as f:
    f.write(text2)
