# parking_lot vs std syncronization primitives benchmark

## Results

```bash
parking_lot_mutex_single_thread
                        time:   [23.603 ns 23.653 ns 23.708 ns]

parking_lot_mutex_multi_thread (4)
                        time:   [90.583 µs 91.508 µs 92.480 µs]

parking_lot_rwlock_single_thread_read
                        time:   [23.666 ns 23.738 ns 23.819 ns]

parking_lot_rwlock_single_thread_write
                        time:   [23.572 ns 23.623 ns 23.682 ns]

parking_lot_rwlock_multi_thread_read (4)
                        time:   [90.042 µs 90.873 µs 91.736 µs]

parking_lot_rwlock_multi_thread_write (4)
                        time:   [91.235 µs 92.040 µs 92.893 µs]

parking_lot_mutex_contended (4)
                        time:   [79.287 µs 80.201 µs 81.167 µs]

parking_lot_rwlock_contended_read (4)
                        time:   [79.370 µs 80.199 µs 81.072 µs]

parking_lot_rwlock_contended_write (4)
                        time:   [79.654 µs 80.442 µs 81.266 µs]

parking_lot_barrier     
                        time:   [91.781 µs 92.598 µs 93.468 µs]

std_mutex_single_thread 
                        time:   [23.523 ns 23.597 ns 23.677 ns]

std_mutex_multi_thread (4)
                        time:   [91.450 µs 92.406 µs 93.441 µs]

std_rwlock_single_thread_read
                        time:   [29.475 ns 29.787 ns 30.103 ns]

std_rwlock_single_thread_write
                        time:   [23.597 ns 23.664 ns 23.739 ns]

std_rwlock_multi_thread_read (4)
                        time:   [90.814 µs 91.621 µs 92.477 µs]

std_rwlock_multi_thread_write (4)
                        time:   [92.022 µs 92.841 µs 93.694 µs]

std_mutex_contended (4) 
                        time:   [80.041 µs 80.988 µs 81.986 µs]

std_rwlock_contended_read (4)
                        time:   [81.301 µs 82.246 µs 83.227 µs]

std_rwlock_contended_write (4)
                        time:   [80.308 µs 81.113 µs 81.958 µs]
std_barrier             
                        time:   [92.871 µs 93.765 µs 94.742 µs]
```
