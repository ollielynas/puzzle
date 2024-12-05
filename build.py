import os

import htmlmin

os.system("cls")

os.system("cargo run ")

book = "error loading book"

with open("book.txt", "r", encoding="utf8", errors="ignore") as f:
    lines = f.readlines()
    book = "".join([("<pre><code>"+"\n".join(lines[i:i+50])+"</code></pre>") for i in range(0, len(lines), 50)]);

html = ""

with open("index-template.html", "r", encoding="utf8") as f:
    html = f.read().replace("CONTENT_GOES_HERE", book)

try:
    html=htmlmin.minify(html, remove_comments=True, remove_empty_space=True)
except:
    print("failed to min")
with open("index.html", "w", encoding="utf-8") as f:
    if html!="":
        f.write(html);


os.system("wasm-pack build --target web")

os.remove("pkg/.gitignore")
