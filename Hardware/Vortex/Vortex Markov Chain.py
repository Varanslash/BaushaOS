# /// Vortex Markov Chain Bot ///

greeting = ["Hello", "Hi", "Greetings", "Salutations", "Howdy", "Hey"]
farewell = ["Goodbye", "Farewell", "Bye"]
pronouns = ["I", "you", "we", "they", "he", "she", "it", "everyone", "someone", "nobody", "anybody", "her", "him", "them", "us", "myself", "yourself", "ourselves", "themselves"]
punctuation = [",", ";", ":", "...", "-", "—", "\\", "/", "\"", "'", "(", ")", "[", "]", "{", "}", "<", ">", "@", "#", "$", "%", "^", "&", "*"]
commonpunctuation = [".", "!", "?"]
names = []
greetingextend = ["man", "there", "friend", "mate", "pal", "buddy", "chum", "amigo", "compadre", "confidant"]
farewellextend = ["man", "mate", "amigo"]
nouns = ["dog", "cat", "car", "house", "computer", "phone", "tree", "river", "mountain", "ocean", "book", "song", "movie", "game", "city", "country", "friend", "family", "job", "school"]

wordcheck = greeting + farewell + pronouns + punctuation + commonpunctuation + greetingextend + farewellextend + names

def generate(length):
    startingword = random.choice(wordcheck)
    words = []
    words.append(startingword)
    i = 1
    for _ in range(length - 1):
        if words[i - 1] in greeting:
            pickedlist = random.choice(greetingextend * 2 + farewellextend)

        elif words[i - 1] in farewell:
            pickedlist = random.choice(farewellextend * 2 + greetingextend)

        elif words[i - 1] in greetingextend or words[i - 1] in farewellextend:
            pickedlist = random.choice(commonpunctuation * 4 + pronouns)

        elif words[i - 1] in pronouns:
            pickedlist = random.choice(nouns + punctuation)

        else:
            pickedlist = wordcheck
        word = random.choice(pickedlist)
        words.append(word)
        i += 1
    return " ".join(words)

def train(inputtext):
    inputwords = inputtext.split()
    for word in inputwords:
        if word.istitle() and word not in wordcheck:
            names.append(word)
        else:
            continue
