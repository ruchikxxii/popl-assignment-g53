import threading
import time
def merge_sort(arr):
    if len(arr) > 1:
        mid = len(arr) // 2
        left_half = arr[:mid]
        right_half = arr[mid:]
 
        if len(left_half)>100 :
            # Create two threads for sorting each half
            left_thread = threading.Thread(target=merge_sort, args=(left_half,))
            right_thread = threading.Thread(target=merge_sort, args=(right_half,))
    
            # Start both threads
            left_thread.start()
            right_thread.start()
    
            # Wait for both threads to finish
            left_thread.join()
            right_thread.join()
        else :
            merge_sort(left_half)
            merge_sort(right_half)
 
        # Merge the sorted halves
        i = j = k = 0
 
        while i < len(left_half) and j < len(right_half):
            if left_half[i] < right_half[j]:
                arr[k] = left_half[i]
                i += 1
            else:
                arr[k] = right_half[j]
                j += 1
            k += 1
 
        while i < len(left_half):
            arr[k] = left_half[i]
            i += 1
            k += 1
 
        while j < len(right_half):
            arr[k] = right_half[j]
            j += 1
            k += 1
 
def multithreaded_merge_sort(arr):
    if len(arr) <= 1:
        return arr
 
    # Create a thread for the merge_sort function
    sorting_thread = threading.Thread(target=merge_sort, args=(arr,))
 
    # Start the sorting thread
    sorting_thread.start()
 
    # Wait for the sorting thread to finish
    sorting_thread.join()
 
if __name__ == '__main__':
    arr = []
    with open('data.txt', 'r') as f:
        arr = [int(line.strip()) for line in f]
 
    start_time = time.time()
    multithreaded_merge_sort(arr)
    end_time = time.time()
    elapsed_time_ms = (end_time - start_time) * 1000  # convert to milliseconds
    # print("Sorted array:", arr)
    print("Time taken for sorting: {} ms".format(elapsed_time_ms))