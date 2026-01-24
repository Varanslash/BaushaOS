import random

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

train("This is a sample text. This text is for training the Markov chain bot. The bot generates text based on the input text.")
print(generate(100, "This"))