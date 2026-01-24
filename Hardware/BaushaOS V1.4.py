# /// Boot Code ///
# pyright: reportMissingImports=false, reportMissingModuleSource=false
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
    "ERR: Item does not exist" # 2
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

# /// Aquamarine Language Compiler ///



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

        if cmdlet[0] == "echo":
            print(" ".join(cmdlet[1:]))

        elif cmdlet[0] == "python":
            print("Welcome to Python! Use /exit to exit, or execute anything here!")
            python3 = 1
            while python3 == 1:
                code = input("py> ")
                if code == "//exit":
                    python3 = 0
                else:
                    exec(code)

        elif cmdlet[0] == "dir":
            if cmdlet[1] == "-create":
                filesystem[cmdlet[2]] = []
            elif cmdlet[1] == "-delete":
                del filesystem[cmdlet[2]]
            elif cmdlet[1] == "-show":
                print("\n".join(filesystem[cmdlet[2]]))
            
        elif cmdlet[0] == "txtedit" and 1 < len(cmdlet):
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
            if cmdlet[1] == "-create":
                if cmdlet[2] == "--new":
                    drivers[cmdlet[3]] = []
                elif cmdlet[2] == "--edit":
                    print("Welcome to Sapphire. Use //exit to exit.")
                    print("Please know that incorrect driver code can lead to system instability or crashes, effectively bricking this OS.")
                    aquamar = 1
                    while aquamar == 1:
                        try:
                            newline = input("aq> ")
                            if newline == "//exit":
                                aquamar = 0
                            else:
                                drivers[cmdlet[3]].append(newline)
                        except KeyError:
                            errorcode(2)

        elif cmdlet[0] == "balengu":
            if cmdlet[1] == "-create":
                if cmdlet[2] == "--new":
                    apps[cmdlet[3]] = []
                elif cmdlet[2] == "--edit":
                    print("Welcome to Balengu App Maker! Use //exit to exit, or type your MicroPython here!")
                    appmake = 1
                    while appmake == 1:
                        try:
                            newline = input("ba> ")
                            if newline == "//exit":
                                appmake = 0
                            else:
                                apps[cmdlet[3]].append(newline)
                        except KeyError:
                            errorcode(2)

        elif cmdlet[0] == "apprun":
            try:
                code = "\n".join(apps[cmdlet[1]])
                exec(code)
            except Exception:
                errorcode(0)
                BSOD()
            
        elif cmdlet[0] == "random":
            print(random.randint(0, int(cmdlet[1])))

        elif cmdlet[0] == "vortex":
            if cmdlet[1] == "-generate":
                result = generate(int(cmdlet[2]))
                print("Vortex Generated Output:", result)
            
        dragon["happiness"] += random.randint(1, 5)
    except IndexError:
        errorcode(1)