import os

import htmlmin
import minify_html

HEIGHT = 52;

os.system("cls")

os.system("cargo run -q")

book = "error loading book"

with open("book.txt", "r", encoding="utf8", errors="ignore") as f:
    lines = f.readlines()
    book = "".join([("<pre><code>"+"".join(lines[i:i+HEIGHT])+"</code></pre>") for i in range(0, len(lines), HEIGHT)]);

html = ""

css = ""

with open("main.css", "r", encoding="utf8") as f:
    css=f.read()
    
with open("index-template.html", "r", encoding="utf8") as f:
    html = f.read().replace("CONTENT_GOES_HERE", book)
    html = html.replace("<link rel='stylesheet' type='text/css' href='main.css'>", "")
    html = html.replace("/* CSS_HERE */", css)



try:
    # html=htmlmin.minify(html, remove_comments=True, remove_empty_space=True)
    html=minify_html.minify(html, minify_css=True, minify_js=True)
except:
    print("failed to min")


with open("index.html", "w", encoding="utf-8") as f:
    if html!="":
        f.write(html);


os.system("wasm-pack --quiet build --target web")

os.remove("pkg/.gitignore")
