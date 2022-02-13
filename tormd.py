import os

for file in os.listdir("./data"):
    print(file)
    if file.endswith(".md") :
        new_name = file.rstrip(".md")
        src = "./data/"+file
        dist = "./data/"+new_name+".rmd"
        os.rename(src, dist)
        print("rename", src, "to", dist)
    elif file.endswith(".markdown"):
        new_name = file.rstrip(".markdown")
        src = "./data/"+file
        dist = "./data/"+new_name+".rmd"
        os.rename(src, dist)
        print("rename", src, "to", dist)

