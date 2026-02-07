# /// Boot Code ///
# Every Import Known To Man

import machine
import gc
import json
import random
import time

filesystem = {

}

apps = {

}

drivers = {

}

dragon = {
    "happiness": 50
}

def startup():
    try:
        with open("config.bau", "r") as f:
            data = f.read().split("\n")
            global filesystem
            global apps
            global drivers
            filesystem = json.loads(data[0])
            apps = json.loads(data[1])
            drivers = json.loads(data[2])
    except Exception:
        pass

startup()

errorcodes = [
    "ERR: A catastrophic and unrecoverable error has occured. Please contact the current owners and developers on GitHub to troubleshoot this error.", # 0
    "ERR: Empty Command", # 1
    "ERR: Item does not exist", # 2
    "ERR: Invalid or missing arguments", # 3
    "ERR: Invalid number", # 4
    "ERR: Runtime error", # 5
    "ERR: Unknown command", # 6
    "ERR: File error", # 7
    "ERR: MicroAqua syntax error" # 8
]

python3 = 0
txtedit = 0
appmake = 0
aquamar = 0
bsod = True

def errorcode(code):
    print(errorcodes[code])
    time.sleep(1)

def exit():
    with open("config.bau", "w") as f:
        f.write(json.dumps(filesystem) + "\n" + json.dumps(apps) + "\n" + json.dumps(drivers))
    raise SystemExit

# Failure handler in the event of app or driver failure

def BSOD():
    if bsod == True:
        gc.collect()
        with open("config.bau", "w") as f:
            f.write(json.dumps(filesystem) + "\n" + json.dumps(apps) + "\n" + json.dumps(drivers))
        machine.reset()

# Load drivers and kernel mods

try:
    for key in drivers:
        try:
            code = "\n".join(drivers[key])
            exec(code)
        except Exception:
            errorcode(0)
            BSOD()
except TypeError:
    pass

# /// MicroAqua language interpreter ///

collectlines = []
indent = 0

def aquapush(fline):
    global collectlines
    collectlines.append(fline)

def interpret(line):
    line = line.lower().split()
    global indent
    indent = 0
    if len(line) == 0:
        return
    
    if line[0] == "echo":
        aquapush(("    " * indent) + "print('" + " ".join(line[1:]) + "')")

    elif line[0] == "var":
        aquapush(("    " * indent) + line[1] + " = " + " ".join(line[2:]))

    elif line[0] == "qfunc":
        op = {
            "add": "+=", "sub": "-=", "mul": "*=",
            "divr": "//=", "divf": "/="
        }
        aquapush(("    " * indent) + line[2] + " " + op[line[1]] + " " + " ".join(line[3:]))

    elif line[0] == "qdef":
        templates = {"stack": "[]", "hmap": "{}"}
        aquapush(("    " * indent) + line[2] + " = " + templates[line[1]])

    elif line[0] == "loop":
        aquapush(("    " * indent) + "while True:")
        indent += 1

    elif line[0] == "for":
        aquapush(("    " * indent) + "for " + " ".join(line[1:]) + ":")
        indent += 1

    elif line[0] == "while":
        aquapush(("    " * indent) + "while " + " ".join(line[1:]) + ":")
        indent += 1

    elif line[0] == "repeat":
        aquapush(("    " * indent) + "for _ in range(" + line[1] + "):")
        indent += 1
        
    elif line[0] == "if":
        aquapush(("    " * indent) + "if " + " ".join(line[1:]) + ":")
        indent += 1
        
    elif line[0] == "elif":
        aquapush(("    " * indent) + "elif " + " ".join(line[1:]) + ":")
        indent += 1
        
    elif line[0] == "else":
        aquapush(("    " * indent) + "else:")
        indent += 1
        
    elif line[0] == "fn":
        aquapush(("    " * indent) + "def " + " ".join(line[1:]) + ":")
        indent += 1
        
    elif line[0] == "mpy":
        aquapush(("    " * indent) + " ".join(line[1:]))

    elif line[0] == "rblock":
        indent = 0

    elif line[0] == "endblock":
        indent = max(0, indent - 1)

    elif line[0] == "iblock":
        indent += 1

    elif line[0] == "none":
        aquapush("pass")

# /// Vortex Markov Chain Bot ///

words = {}

def generate(length, word):
    startingword = word
    sentence = []
    sentence.append(startingword)
    i = 1
    for _ in range(length - 1):
        if sentence[i - 1] in words:
            nextwordoptions = words[sentence[i - 1]]
            if len(nextwordoptions) == 0:
                break
            nextword = random.choice(nextwordoptions)
            sentence.append(nextword)
            i += 1
    return " ".join(sentence)

def train(inputtext):
    inputwords = inputtext.split()
    i = 0
    for word in inputwords:
        if word not in words:
            words[word] = []
        else:
            if i + 1 < len(inputwords):
                words[word].append(inputwords[i + 1])
        i += 1

# /// System ///

while True:
    try:
        cmdlet = input("BaushaOS> ").split()
        if len(cmdlet) == 0:
            errorcode(1)
            continue

        if cmdlet[0] == "echo":
            print(" ".join(cmdlet[1:]))

        elif cmdlet[0] == "python":
            print("Welcome to Python! Use //exit to exit, or execute anything here!")
            python3 = 1
            while python3 == 1:
                code = input("py> ")
                if code == "//exit":
                    python3 = 0
                else:
                    try:
                        exec(code)
                    except Exception:
                        errorcode(5)

        elif cmdlet[0] == "dir":
            if len(cmdlet) < 3:
                errorcode(3)
                continue
            if cmdlet[1] == "-create":
                filesystem[cmdlet[2]] = []
            elif cmdlet[1] == "-delete":
                try:
                    del filesystem[cmdlet[2]]
                except KeyError:
                    errorcode(2)
            elif cmdlet[1] == "-show":
                try:
                    print("\n".join(filesystem[cmdlet[2]]))
                except KeyError:
                    errorcode(2)
            else:
                errorcode(3)
            
        elif cmdlet[0] == "txtedit":
            if len(cmdlet) < 2:
                errorcode(3)
                continue
            if cmdlet[1] not in filesystem:
                errorcode(2)
                continue
            print("Welcome to Text Editor! Use //exit to exit, or type anything here!")
            txtedit = 1
            while txtedit == 1:
                newline = input("tx> ")
                if newline == "//exit":
                    txtedit = 0
                else:
                    filesystem[cmdlet[1]].append(newline)

        elif cmdlet[0] == "poweroff":
            exit()

        elif cmdlet[0] == "sapphire":
            if len(cmdlet) < 4 or cmdlet[1] != "-create":
                errorcode(3)
                continue
            if cmdlet[2] == "--new":
                drivers[cmdlet[3]] = []
            elif cmdlet[2] == "--edit":
                if cmdlet[3] not in drivers:
                    errorcode(2)
                    continue
                print("Welcome to Sapphire. Use //exit to exit.")
                print("Please know that incorrect driver code can lead to system instability or crashes, effectively bricking this OS.")
                aquamar = 1
                while aquamar == 1:
                    newline = input("sh> ")
                    if newline == "//exit":
                        aquamar = 0
                    else:
                        drivers[cmdlet[3]].append(newline)
            else:
                errorcode(3)

        elif cmdlet[0] == "balengu":
            if len(cmdlet) < 4 or cmdlet[1] != "-create":
                errorcode(3)
                continue
            if cmdlet[2] == "--new":
                apps[cmdlet[3]] = []
            elif cmdlet[2] == "--edit":
                if cmdlet[3] not in apps:
                    errorcode(2)
                    continue
                print("Welcome to Balengu App Maker! Use //exit to exit, or type your MicroPython here!")
                appmake = 1
                while appmake == 1:
                    newline = input("ba> ")
                    if newline == "//exit":
                        appmake = 0
                    else:
                        apps[cmdlet[3]].append(newline)
            else:
                errorcode(3)

        elif cmdlet[0] == "apprun":
            if len(cmdlet) < 2:
                errorcode(3)
                continue
            if cmdlet[1] not in apps:
                errorcode(2)
                continue
            try:
                code = "\n".join(apps[cmdlet[1]])
                exec(code)
            except Exception:
                errorcode(5)
                BSOD()

        elif cmdlet[0] == "aquamarine":
            if len(cmdlet) < 3:
                errorcode(3)
                continue
            if cmdlet[1] not in apps:
                errorcode(2)
                continue
            try:
                for line in apps[cmdlet[1]]:
                    interpret(line)
            except Exception:
                errorcode(8)
                collectlines = []
                continue
            apps[cmdlet[2]] = collectlines
            collectlines = []
            
        elif cmdlet[0] == "random":
            if len(cmdlet) < 2:
                errorcode(3)
                continue
            try:
                upper = int(cmdlet[1])
                if upper < 0:
                    raise ValueError
                print(random.randint(0, upper))
            except ValueError:
                errorcode(4)

        elif cmdlet[0] == "vortex":
            if len(cmdlet) < 2:
                errorcode(3)
                continue
            if cmdlet[1] == "-generate":
                if len(cmdlet) < 4:
                    errorcode(3)
                    continue
                try:
                    length = int(cmdlet[2])
                    if length < 1:
                        raise ValueError
                except ValueError:
                    errorcode(4)
                    continue
                result = generate(length, cmdlet[3])
                print("Vortex Generated Output:", result)
            elif cmdlet[1] == "-train":
                if len(cmdlet) < 3:
                    errorcode(3)
                    continue
                try:
                    with open(cmdlet[2], "r") as file:
                        data = file.read()
                        train(data)
                except Exception:
                    errorcode(7)
            else:
                errorcode(3)
        else:
            errorcode(6)
            
        dragon["happiness"] += random.randint(1, 5)
    except IndexError:
        errorcode(1)

