import os
import shutil

os.system("wasm-pack build --target web");

os.remove("pkg/.gitignore")