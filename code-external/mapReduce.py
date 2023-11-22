import threading
from collections import defaultdict

def map_reduce(words, num_threads):
    word_counts = defaultdict(int)
    chunk_size = len(words) // num_threads

    def process_chunk(chunk):
        local_counts = defaultdict(int)
        for word in chunk:
            local_counts[word] += 1
        with lock:
            for word, count in local_counts.items():
                word_counts[word] += count

    lock = threading.Lock()
    threads = []
    for i in range(num_threads):
        chunk = words[i * chunk_size: (i + 1) * chunk_size]
        thread = threading.Thread(target=process_chunk, args=(chunk,))
        thread.start()
        threads.append(thread)

    for thread in threads:
        thread.join()

    return dict(word_counts)


if __name__ == "__main__":
    import time
    start_time = time.time()
    with open("citiesOrders.txt", "r") as file:
        words = [line.strip() for line in file]

   
    result = map_reduce(words, 4)
    elapsed_time = time.time() - start_time

    for word, count in result.items():
        print(f"{word}: {count}")

    print(f"Elapsed: {elapsed_time:.2f}s")
